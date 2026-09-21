use crossbeam_channel::{unbounded, Receiver};
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

use crate::cleanup::{
    CategoryScanResult, CleanupCategoryId, CleanupExecutionReport, CleanupExecutor,
    CleanupProgressEvent, CleanupScanner, SafetyLevel, CLEANUP_CATEGORIES,
};
use crate::ui::components::{Badge, EmptyState, ModernProgressBar};
use crate::ui::theme::{
    format_bytes, COLOR_ACCENT_HOVER, COLOR_ACCENT_PRIMARY, COLOR_BORDER, COLOR_BRAND_ACCENT,
    COLOR_CARD_BG, COLOR_DELETE_BG, COLOR_DELETE_TEXT, COLOR_KEEP_BG, COLOR_KEEP_TEXT,
    COLOR_MUTED_TEXT, COLOR_PANEL_BG, COLOR_SENSITIVE_BG, COLOR_SENSITIVE_TEXT,
    COLOR_TEXT_PRIMARY, RADIUS_LG, RADIUS_MD, RADIUS_SM, SPACE_LG, SPACE_MD, SPACE_SM,
    SPACE_XL, SPACE_XS,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CleanupScreen {
    Idle,
    Scanning,
    Results,
    Cleaning,
    Completed,
}

pub enum CleanupScanMessage {
    Progress { category: String, ratio: f32 },
    Finished(Vec<CategoryScanResult>),
}

pub struct CleanupState {
    pub screen: CleanupScreen,
    pub categories: Vec<CategoryScanResult>,
    pub scan_stage_name: String,
    pub scan_progress_ratio: f32,
    pub scan_rx: Option<Receiver<CleanupScanMessage>>,

    // Background cleaning state
    pub clean_rx: Option<Receiver<CleanupProgressEvent>>,
    pub clean_cancel_flag: Arc<AtomicBool>,
    pub clean_current: usize,
    pub clean_total: usize,
    pub clean_current_item: String,
    pub clean_current_cat: String,
    pub clean_bytes_freed: u64,

    pub show_confirm_modal: bool,
    pub last_report: Option<CleanupExecutionReport>,
}

impl Default for CleanupState {
    fn default() -> Self {
        let initial_categories = CLEANUP_CATEGORIES
            .iter()
            .map(CategoryScanResult::new)
            .collect();

        Self {
            screen: CleanupScreen::Idle,
            categories: initial_categories,
            scan_stage_name: String::new(),
            scan_progress_ratio: 0.0,
            scan_rx: None,

            clean_rx: None,
            clean_cancel_flag: Arc::new(AtomicBool::new(false)),
            clean_current: 0,
            clean_total: 0,
            clean_current_item: String::new(),
            clean_current_cat: String::new(),
            clean_bytes_freed: 0,

            show_confirm_modal: false,
            last_report: None,
        }
    }
}

impl CleanupState {
    pub fn start_scan(&mut self) {
        let (tx, rx) = unbounded();
        self.scan_rx = Some(rx);
        self.screen = CleanupScreen::Scanning;
        self.scan_stage_name = "Memulai analisis sampah sistem...".to_string();
        self.scan_progress_ratio = 0.05;

        thread::spawn(move || {
            let results = CleanupScanner::scan_all(Some(|cat_name: &str, ratio: f32| {
                let _ = tx.send(CleanupScanMessage::Progress {
                    category: cat_name.to_string(),
                    ratio,
                });
            }));
            let _ = tx.send(CleanupScanMessage::Finished(results));
        });
    }

    pub fn start_clean(&mut self) {
        self.clean_cancel_flag.store(false, Ordering::Relaxed);
        self.clean_current = 0;
        self.clean_total = 0;
        self.clean_current_item = String::new();
        self.clean_current_cat = String::new();
        self.clean_bytes_freed = 0;

        let rx = CleanupExecutor::start(
            self.categories.clone(),
            Arc::clone(&self.clean_cancel_flag),
        );
        self.clean_rx = Some(rx);
        self.show_confirm_modal = false;
        self.screen = CleanupScreen::Cleaning;
    }

    pub fn update_background_messages(&mut self) {
        // Poll scanning messages
        if let Some(ref rx) = self.scan_rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    CleanupScanMessage::Progress { category, ratio } => {
                        self.scan_stage_name = format!("Menganalisis: {}", category);
                        self.scan_progress_ratio = ratio;
                    }
                    CleanupScanMessage::Finished(results) => {
                        self.categories = results;
                        self.screen = CleanupScreen::Results;
                        self.scan_rx = None;
                        break;
                    }
                }
            }
        }

        // Poll cleaning messages
        if let Some(ref rx) = self.clean_rx {
            while let Ok(event) = rx.try_recv() {
                match event {
                    CleanupProgressEvent::Started { total } => {
                        self.clean_total = total;
                        self.clean_current = 0;
                    }
                    CleanupProgressEvent::Progress {
                        current,
                        total,
                        current_item,
                        category,
                        bytes_freed,
                    } => {
                        self.clean_current = current;
                        self.clean_total = total;
                        self.clean_current_item = current_item;
                        self.clean_current_cat = category;
                        self.clean_bytes_freed = bytes_freed;
                    }
                    CleanupProgressEvent::Finished { report }
                    | CleanupProgressEvent::Cancelled {
                        partial_report: report,
                    } => {
                        // Refresh remaining category items
                        for cat in &mut self.categories {
                            if cat.is_enabled {
                                if cat.id == CleanupCategoryId::RecycleBin {
                                    cat.items.clear();
                                    cat.total_bytes = 0;
                                } else {
                                    cat.items.retain(|i| i.path.exists());
                                    cat.total_bytes = cat.items.iter().map(|i| i.size).sum();
                                }
                            }
                        }
                        self.last_report = Some(report);
                        self.clean_rx = None;
                        self.screen = CleanupScreen::Completed;
                        break;
                    }
                }
            }
        }
    }

    pub fn select_all_safe(&mut self) {
        for cat in &mut self.categories {
            cat.is_enabled = cat.safety == SafetyLevel::Safe;
        }
    }

    pub fn select_all(&mut self) {
        for cat in &mut self.categories {
            cat.is_enabled = true;
        }
    }

    pub fn deselect_all(&mut self) {
        for cat in &mut self.categories {
            cat.is_enabled = false;
        }
    }

    pub fn selected_bytes(&self) -> u64 {
        self.categories
            .iter()
            .filter(|c| c.is_enabled)
            .map(|c| c.total_bytes)
            .sum()
    }

    pub fn selected_count(&self) -> usize {
        self.categories.iter().filter(|c| c.is_enabled).count()
    }

    pub fn total_detected_bytes(&self) -> u64 {
        self.categories.iter().map(|c| c.total_bytes).sum()
    }

    pub fn total_detected_items(&self) -> usize {
        self.categories.iter().map(|c| c.items.len()).sum()
    }
}

pub struct CleanupView;

impl CleanupView {
    pub fn render(state: &mut CleanupState, ctx: &egui::Context, ui: &mut egui::Ui) {
        state.update_background_messages();

        if state.screen == CleanupScreen::Scanning || state.screen == CleanupScreen::Cleaning {
            ctx.request_repaint();
        }

        match state.screen {
            CleanupScreen::Idle => Self::render_idle(state, ui),
            CleanupScreen::Scanning => Self::render_scanning(state, ui),
            CleanupScreen::Results => Self::render_results(state, ctx, ui),
            CleanupScreen::Cleaning => Self::render_cleaning(state, ui),
            CleanupScreen::Completed => Self::render_completed(state, ui),
        }

        if state.show_confirm_modal {
            Self::render_confirmation_dialog(state, ctx);
        }
    }

    fn render_idle(state: &mut CleanupState, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(SPACE_SM);
            ui.heading(
                RichText::new("🧹 Bersihkan Sampah Sistem")
                    .size(26.0)
                    .color(COLOR_TEXT_PRIMARY)
                    .strong(),
            );
            ui.add_space(SPACE_XS);
            ui.label(
                RichText::new(
                    "Pindai dan bersihkan file sementara, cache browser, thumbnail, log lama, dan cache aplikasi secara aman.",
                )
                .color(COLOR_MUTED_TEXT)
                .size(13.0),
            );

            ui.add_space(SPACE_LG);

            let analyze_btn = ui.add_sized(
                [320.0, 48.0],
                egui::Button::new(
                    RichText::new("🔍 Analisis Sampah Sekarang")
                        .size(16.0)
                        .color(Color32::from_rgb(14, 16, 21))
                        .strong(),
                )
                .fill(COLOR_BRAND_ACCENT)
                .rounding(Rounding::same(RADIUS_MD)),
            );

            if analyze_btn.clicked() {
                state.start_scan();
            }

            ui.add_space(SPACE_LG);
        });

        ui.label(
            RichText::new("Kategori yang Akan Dianalisis:")
                .strong()
                .size(14.0)
                .color(COLOR_TEXT_PRIMARY),
        );
        ui.add_space(SPACE_SM);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(SPACE_SM, SPACE_SM);
            for cat in &state.categories {
                egui::Frame::none()
                    .fill(COLOR_PANEL_BG)
                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                    .rounding(Rounding::same(RADIUS_MD))
                    .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(cat.icon).size(20.0));
                            ui.vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(cat.title)
                                            .strong()
                                            .size(13.0)
                                            .color(COLOR_TEXT_PRIMARY),
                                    );
                                    match cat.safety {
                                        SafetyLevel::Safe => {
                                            Badge::show(ui, "Aman", COLOR_KEEP_BG, COLOR_KEEP_TEXT);
                                        }
                                        SafetyLevel::NeedsReview => {
                                            Badge::show(
                                                ui,
                                                "⚠️ Perlu Review",
                                                COLOR_SENSITIVE_BG,
                                                COLOR_SENSITIVE_TEXT,
                                            );
                                        }
                                    }
                                });
                                ui.label(
                                    RichText::new(cat.description)
                                        .color(COLOR_MUTED_TEXT)
                                        .size(11.0),
                                );
                            });
                        });
                    });
            }
        });
    }

    fn render_scanning(state: &CleanupState, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(SPACE_XL);
            ui.heading(
                RichText::new("Menganalisis Sampah Sistem...")
                    .size(24.0)
                    .color(COLOR_TEXT_PRIMARY)
                    .strong(),
            );
            ui.add_space(SPACE_XS);
            ui.label(
                RichText::new(&state.scan_stage_name)
                    .color(COLOR_BRAND_ACCENT)
                    .size(14.0)
                    .strong(),
            );
            ui.add_space(SPACE_LG);

            // Container Card for Scanning Progress
            egui::Frame::none()
                .fill(COLOR_PANEL_BG)
                .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                .rounding(Rounding::same(RADIUS_LG))
                .inner_margin(egui::Margin::symmetric(24.0, 20.0))
                .show(ui, |ui| {
                    ModernProgressBar::show(
                        ui,
                        state.scan_progress_ratio,
                        &format!("{:.0}%", state.scan_progress_ratio * 100.0),
                        &state.scan_stage_name,
                    );
                });
        });
    }

    fn render_cleaning(state: &mut CleanupState, ui: &mut egui::Ui) {
        let ratio = if state.clean_total > 0 {
            state.clean_current as f32 / state.clean_total as f32
        } else {
            0.0
        };

        ui.vertical_centered(|ui| {
            ui.add_space(SPACE_XL);
            ui.heading(
                RichText::new("Sedang Membersihkan Sampah Sistem...")
                    .size(24.0)
                    .color(COLOR_TEXT_PRIMARY)
                    .strong(),
            );
            ui.add_space(SPACE_XS);

            let is_cancelling = state.clean_cancel_flag.load(Ordering::Relaxed);
            let status_text = if is_cancelling {
                "Membatalkan pembersihan... menyelesaikan item saat ini.".to_string()
            } else {
                format!(
                    "Memproses {} dari {} file ({})",
                    state.clean_current,
                    state.clean_total,
                    state.clean_current_cat
                )
            };

            ui.label(
                RichText::new(status_text)
                    .color(if is_cancelling { COLOR_DELETE_TEXT } else { COLOR_BRAND_ACCENT })
                    .size(14.0)
                    .strong(),
            );
            ui.add_space(SPACE_LG);

            // Container Card for Cleaning Progress
            egui::Frame::none()
                .fill(COLOR_PANEL_BG)
                .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                .rounding(Rounding::same(RADIUS_LG))
                .inner_margin(egui::Margin::symmetric(24.0, 20.0))
                .show(ui, |ui| {
                    let detail = format!(
                        "[{}] {} • Dibebaskan: {}",
                        state.clean_current_cat,
                        state.clean_current_item,
                        format_bytes(state.clean_bytes_freed)
                    );

                    ModernProgressBar::show(
                        ui,
                        ratio,
                        &format!("{:.0}%", ratio * 100.0),
                        &detail,
                    );
                });

            ui.add_space(SPACE_LG);
            ui.add_enabled_ui(!is_cancelling, |ui| {
                let cancel_btn = ui.add_sized(
                    [200.0, 38.0],
                    egui::Button::new(
                        RichText::new(if is_cancelling { "Membatalkan..." } else { "⏹ Batalkan" })
                            .color(Color32::WHITE)
                            .strong(),
                    )
                    .fill(COLOR_DELETE_BG)
                    .rounding(Rounding::same(RADIUS_MD)),
                );

                if cancel_btn.clicked() {
                    state.clean_cancel_flag.store(true, Ordering::Relaxed);
                }
            });
        });
    }

    fn render_results(state: &mut CleanupState, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let total_detected = state.total_detected_bytes();
        let selected_bytes = state.selected_bytes();
        let selected_count = state.selected_count();

        // Check if zero junk found
        if total_detected == 0 {
            EmptyState::show(
                ui,
                "✨",
                "Sistem Anda Bersih & Rapi!",
                "Tidak ditemukan file sampah sementara atau cache usang. Semua drive dalam kondisi optimal.",
            );
            ui.vertical_centered(|ui| {
                if ui
                    .button(RichText::new("🔄 Analisis Ulang").color(Color32::WHITE))
                    .clicked()
                {
                    state.start_scan();
                }
            });
            return;
        }

        // Top Summary Card
        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
            .rounding(Rounding::same(RADIUS_MD))
            .inner_margin(egui::Margin::symmetric(16.0, 12.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading(
                            RichText::new(format!(
                                "Total Sampah Ditemukan: {}",
                                format_bytes(total_detected)
                            ))
                            .color(Color32::WHITE)
                            .size(18.0)
                            .strong(),
                        );
                        ui.label(
                            RichText::new(format!(
                                "Terpilih: {} (dari {} kategori)",
                                format_bytes(selected_bytes),
                                selected_count
                            ))
                            .color(COLOR_ACCENT_HOVER)
                            .size(13.0)
                            .strong(),
                        );
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let can_clean = selected_bytes > 0;
                        let clean_btn = ui.add_sized(
                            [260.0, 42.0],
                            egui::Button::new(
                                RichText::new(format!(
                                    "🧹 Bersihkan ({})",
                                    format_bytes(selected_bytes)
                                ))
                                .size(14.0)
                                .color(Color32::WHITE)
                                .strong(),
                            )
                            .fill(if can_clean {
                                COLOR_DELETE_BG
                            } else {
                                Color32::from_rgb(50, 55, 65)
                            }),
                        );

                        if clean_btn.clicked() && can_clean {
                            state.show_confirm_modal = true;
                        }
                    });
                });
            });

        ui.add_space(SPACE_SM);

        // Filter / Selection Toolbar
        ui.horizontal(|ui| {
            ui.label(RichText::new("Seleksi:").strong());

            if ui.small_button("Pilih Kategori Aman (Default)").clicked() {
                state.select_all_safe();
            }

            if ui.small_button("Pilih Semua").clicked() {
                state.select_all();
            }

            if ui.small_button("Hapus Centang").clicked() {
                state.deselect_all();
            }

            ui.separator();

            if ui.small_button("🔄 Analisis Ulang").clicked() {
                state.start_scan();
            }
        });

        ui.add_space(SPACE_SM);

        // Category Cards List
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.spacing_mut().item_spacing = Vec2::new(SPACE_SM, SPACE_SM);

            for cat in &mut state.categories {
                let has_items = !cat.items.is_empty();

                egui::Frame::none()
                    .fill(if cat.is_enabled {
                        COLOR_CARD_BG
                    } else {
                        COLOR_PANEL_BG
                    })
                    .stroke(Stroke::new(
                        1.0_f32,
                        if cat.is_enabled {
                            COLOR_ACCENT_PRIMARY
                        } else {
                            COLOR_BORDER
                        },
                    ))
                    .rounding(Rounding::same(RADIUS_MD))
                    .inner_margin(egui::Margin::symmetric(14.0, 10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.checkbox(&mut cat.is_enabled, "");

                            ui.label(RichText::new(cat.icon).size(18.0));

                            ui.label(RichText::new(cat.title).strong().size(13.0));

                            match cat.safety {
                                SafetyLevel::Safe => {
                                    Badge::show(ui, "Aman", COLOR_KEEP_BG, COLOR_KEEP_TEXT);
                                }
                                SafetyLevel::NeedsReview => {
                                    Badge::show(ui, "⚠️ Perlu Review", COLOR_SENSITIVE_BG, COLOR_SENSITIVE_TEXT);
                                }
                            }

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    let size_str = format_bytes(cat.total_bytes);
                                    let items_str = if cat.items.len() == 1
                                        && cat.id == CleanupCategoryId::RecycleBin
                                    {
                                        String::new()
                                    } else {
                                        format!("({} item)", cat.items.len())
                                    };

                                    ui.label(
                                        RichText::new(format!("{} {}", size_str, items_str))
                                            .size(13.0)
                                            .color(if has_items {
                                                Color32::WHITE
                                            } else {
                                                COLOR_MUTED_TEXT
                                            })
                                            .strong(),
                                    );

                                    if has_items && cat.id != CleanupCategoryId::RecycleBin {
                                        let arrow = if cat.is_expanded { "▼ Tutup" } else { "▶ Detail" };
                                        if ui.small_button(arrow).clicked() {
                                            cat.is_expanded = !cat.is_expanded;
                                        }
                                    }
                                },
                            );
                        });

                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(cat.description)
                                .color(COLOR_MUTED_TEXT)
                                .size(11.0),
                        );

                        if let Some(warn) = cat.warning {
                            ui.label(
                                RichText::new(format!("ℹ {}", warn))
                                    .color(COLOR_ACCENT_HOVER)
                                    .size(11.0),
                            );
                        }

                        // Expanded item details list
                        if cat.is_expanded && !cat.items.is_empty() {
                            ui.add_space(6.0);
                            ui.separator();
                            ui.label(
                                RichText::new(format!("Daftar file dalam {} :", cat.title))
                                    .size(11.0)
                                    .strong(),
                            );

                            egui::ScrollArea::vertical()
                                .max_height(140.0)
                                .show(ui, |ui| {
                                    for item in cat.items.iter().take(200) {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new(format_bytes(item.size))
                                                    .size(10.0)
                                                    .color(COLOR_MUTED_TEXT),
                                            );
                                            ui.label(
                                                RichText::new(item.path.to_string_lossy().to_string())
                                                    .size(10.0)
                                                    .monospace(),
                                            );
                                        });
                                    }
                                    if cat.items.len() > 200 {
                                        ui.label(
                                            RichText::new(format!(
                                                "... dan {} file lainnya",
                                                cat.items.len() - 200
                                            ))
                                            .color(COLOR_MUTED_TEXT)
                                            .size(10.0),
                                        );
                                    }
                                });
                        }
                    });
            }
        });
    }

    fn render_completed(state: &mut CleanupState, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(SPACE_XL);
            ui.label(RichText::new("🎉").size(48.0));
            ui.heading(
                RichText::new("Pembersihan Selesai!")
                    .size(26.0)
                    .color(Color32::from_rgb(34, 197, 94))
                    .strong(),
            );
            ui.add_space(SPACE_LG);

            if let Some(ref report) = state.last_report {
                egui::Frame::none()
                    .fill(COLOR_PANEL_BG)
                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                    .rounding(Rounding::same(RADIUS_LG))
                    .inner_margin(egui::Margin::symmetric(28.0, 20.0))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(format!(
                                "Ruang disk dibebaskan: {}",
                                format_bytes(report.bytes_freed)
                            ))
                            .size(18.0)
                            .color(Color32::from_rgb(74, 222, 128))
                            .strong(),
                        );

                        ui.add_space(SPACE_SM);
                        ui.label(
                            RichText::new(format!(
                                "Total file dibersihkan: {} • Dilewati/Locked: {}",
                                report.successful_deleted, report.skipped_locked
                            ))
                            .color(COLOR_TEXT_PRIMARY)
                            .size(13.0),
                        );

                        if report.skipped_locked > 0 {
                            ui.add_space(SPACE_XS);
                            ui.label(
                                RichText::new(
                                    "ℹ File yang dilewati sedang aktif digunakan atau dikunci oleh sistem/aplikasi lain.",
                                )
                                .color(COLOR_ACCENT_HOVER)
                                .size(11.0),
                            );
                        }

                        ui.add_space(SPACE_MD);
                        ui.label(
                            RichText::new(format!("Log audit disimpan di: {}", report.log_path.display()))
                                .color(COLOR_MUTED_TEXT)
                                .size(11.0)
                                .monospace(),
                        );
                    });
            }

            ui.add_space(SPACE_XL);

            let ok_btn = ui.add_sized(
                [240.0, 44.0],
                egui::Button::new(
                    RichText::new("Selesai & Analisis Ulang")
                        .size(14.0)
                        .color(Color32::from_rgb(14, 16, 21))
                        .strong(),
                )
                .fill(COLOR_BRAND_ACCENT)
                .rounding(Rounding::same(RADIUS_MD)),
            );

            if ok_btn.clicked() {
                state.screen = CleanupScreen::Idle;
                state.start_scan();
            }
        });
    }

    fn render_confirmation_dialog(state: &mut CleanupState, ctx: &egui::Context) {
        let selected_bytes = state.selected_bytes();
        let selected_cats: Vec<&CategoryScanResult> =
            state.categories.iter().filter(|c| c.is_enabled).collect();

        let mut should_close = false;
        let mut should_execute = false;

        egui::Window::new("Konfirmasi Pembersihan Sampah")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .fixed_size([500.0, 360.0])
            .show(ctx, |ui| {
                ui.heading(
                    RichText::new("Konfirmasi Penghapusan File")
                        .color(Color32::WHITE)
                        .size(18.0)
                        .strong(),
                );

                ui.add_space(SPACE_SM);
                ui.label(
                    RichText::new(format!(
                        "Anda akan membersihkan {} kategori sampah sistem dengan estimasi ruang dibebaskan: {}",
                        selected_cats.len(),
                        format_bytes(selected_bytes)
                    ))
                    .color(Color32::WHITE)
                    .size(13.0),
                );

                ui.add_space(SPACE_SM);
                ui.label(RichText::new("Kategori yang akan dibersihkan:").strong().size(12.0));

                egui::Frame::none()
                    .fill(COLOR_PANEL_BG)
                    .rounding(Rounding::same(RADIUS_SM))
                    .inner_margin(SPACE_SM)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical().max_height(140.0).show(ui, |ui| {
                            for cat in &selected_cats {
                                ui.horizontal(|ui| {
                                    ui.label(RichText::new(cat.icon).size(14.0));
                                    ui.label(RichText::new(cat.title).size(12.0));
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            ui.label(
                                                RichText::new(format_bytes(cat.total_bytes))
                                                    .color(COLOR_MUTED_TEXT)
                                                    .size(11.0),
                                            );
                                        },
                                    );
                                });
                            }
                        });
                    });

                ui.add_space(10.0);
                ui.label(
                    RichText::new("ℹ Catatan Keamanan: File yang sedang aktif dipakai oleh Windows akan otomatis dilewati tanpa mengganggu sistem.")
                        .color(COLOR_MUTED_TEXT)
                        .size(11.0),
                );

                ui.add_space(SPACE_MD);
                ui.horizontal(|ui| {
                    let cancel_btn = ui.add_sized(
                        [120.0, 36.0],
                        egui::Button::new(RichText::new("Batal").color(Color32::WHITE)),
                    );
                    if cancel_btn.clicked() {
                        should_close = true;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let clean_btn = ui.add_sized(
                            [200.0, 36.0],
                            egui::Button::new(
                                RichText::new("Ya, Bersihkan Sekarang")
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .fill(COLOR_DELETE_BG),
                        );
                        if clean_btn.clicked() {
                            should_execute = true;
                        }
                    });
                });
            });

        if should_close {
            state.show_confirm_modal = false;
        }
        if should_execute {
            state.start_clean();
        }
    }
}
