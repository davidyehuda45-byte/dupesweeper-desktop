use crossbeam_channel::Receiver;
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::analyzer::{AnalyzerProgress, FolderAnalyzer, SizeEntry};
use crate::ui::components::{Badge, EmptyState, ModernProgressBar};
use crate::ui::icons::{render_icon, IconKind};
use crate::ui::theme::{
    format_bytes, COLOR_BORDER, COLOR_BRAND_ACCENT, COLOR_CARD_BG, COLOR_CARD_HOVER,
    COLOR_MUTED_TEXT, COLOR_PANEL_BG, COLOR_TEXT_PRIMARY, RADIUS_MD, RADIUS_SM, SPACE_MD,
    SPACE_SM, SPACE_XS,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AnalyzerScreen {
    Idle,
    Scanning,
    Results,
}

pub struct AnalyzerState {
    pub screen: AnalyzerScreen,
    pub root: Option<PathBuf>,
    pub path_stack: Vec<PathBuf>,
    pub entries: Vec<SizeEntry>,
    pub total_size: u64,
    pub scanned_count: usize,
    pub error: Option<String>,
    cancel_flag: Arc<AtomicBool>,
    rx: Option<Receiver<AnalyzerProgress>>,
}

impl Default for AnalyzerState {
    fn default() -> Self {
        Self {
            screen: AnalyzerScreen::Idle,
            root: None,
            path_stack: Vec::new(),
            entries: Vec::new(),
            total_size: 0,
            scanned_count: 0,
            error: None,
            cancel_flag: Arc::new(AtomicBool::new(false)),
            rx: None,
        }
    }
}

impl AnalyzerState {
    pub fn current_dir(&self) -> Option<&PathBuf> {
        self.path_stack.last()
    }

    pub fn pick_root(&mut self, path: PathBuf) {
        self.root = Some(path.clone());
        self.path_stack = vec![path];
        self.start_scan_current();
    }

    pub fn enter(&mut self, path: PathBuf) {
        self.path_stack.push(path);
        self.start_scan_current();
    }

    pub fn go_to_breadcrumb(&mut self, index: usize) {
        if index < self.path_stack.len() {
            self.path_stack.truncate(index + 1);
            self.start_scan_current();
        }
    }

    pub fn reset(&mut self) {
        self.cancel_flag.store(true, Ordering::Relaxed);
        *self = Self::default();
    }

    fn start_scan_current(&mut self) {
        let Some(dir) = self.current_dir().cloned() else {
            return;
        };
        self.cancel_flag = Arc::new(AtomicBool::new(false));
        self.entries.clear();
        self.total_size = 0;
        self.scanned_count = 0;
        self.error = None;
        self.screen = AnalyzerScreen::Scanning;
        self.rx = Some(FolderAnalyzer::start(dir, Arc::clone(&self.cancel_flag)));
    }

    pub fn update_background_messages(&mut self) {
        let mut finished: Option<(Vec<SizeEntry>, u64)> = None;
        let mut cancelled = false;
        let mut errored: Option<String> = None;

        if let Some(rx) = &self.rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    AnalyzerProgress::Scanning { scanned_count } => {
                        self.scanned_count = scanned_count;
                    }
                    AnalyzerProgress::Finished {
                        entries,
                        total_size,
                        elapsed: _,
                    } => {
                        finished = Some((entries, total_size));
                    }
                    AnalyzerProgress::Cancelled => {
                        cancelled = true;
                    }
                    AnalyzerProgress::Error(e) => {
                        errored = Some(e);
                    }
                }
            }
        }

        if let Some((entries, total_size)) = finished {
            self.entries = entries;
            self.total_size = total_size;
            self.screen = AnalyzerScreen::Results;
            self.rx = None;
        } else if cancelled {
            self.rx = None;
        } else if let Some(e) = errored {
            self.error = Some(e);
            self.screen = AnalyzerScreen::Idle;
            self.rx = None;
        }
    }
}

pub struct AnalyzerView;

impl AnalyzerView {
    pub fn render(state: &mut AnalyzerState, ctx: &egui::Context, ui: &mut egui::Ui) {
        state.update_background_messages();

        match state.screen {
            AnalyzerScreen::Idle => Self::render_idle(state, ui),
            AnalyzerScreen::Scanning => {
                ctx.request_repaint();
                Self::render_scanning(state, ui);
            }
            AnalyzerScreen::Results => Self::render_results(state, ui),
        }
    }

    fn render_idle(state: &mut AnalyzerState, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(SPACE_MD);
            ui.heading(
                RichText::new("Analisis Ukuran Folder")
                    .size(26.0)
                    .color(COLOR_TEXT_PRIMARY)
                    .strong(),
            );
            ui.add_space(SPACE_XS);
            ui.label(
                RichText::new(
                    "Telusuri isi sebuah folder terurut dari yang paling besar, untuk menemukan apa yang memakan ruang disk Anda.",
                )
                .color(COLOR_MUTED_TEXT)
                .size(13.0),
            );
            ui.add_space(SPACE_MD);

            if let Some(err) = &state.error {
                Badge::show(ui, err, Color32::from_rgb(69, 26, 26), Color32::from_rgb(254, 202, 202));
                ui.add_space(SPACE_SM);
            }

            if ui
                .add(
                    egui::Button::new(
                        RichText::new("Pilih Folder untuk Dianalisis")
                            .color(Color32::BLACK)
                            .strong()
                            .size(13.5),
                    )
                    .fill(COLOR_BRAND_ACCENT)
                    .rounding(Rounding::same(RADIUS_MD))
                    .min_size(Vec2::new(260.0, 40.0)),
                )
                .clicked()
            {
                if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                    state.pick_root(folder);
                }
            }
        });

        ui.add_space(SPACE_MD);
        EmptyState::show(
            ui,
            IconKind::ChartBar,
            "Belum ada folder dipilih",
            "Pilih folder di atas untuk melihat rincian ukuran isinya.",
        );
    }

    fn render_scanning(state: &AnalyzerState, ui: &mut egui::Ui) {
        ui.add_space(SPACE_MD * 2.0);
        let label = state
            .current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or_default();
        ModernProgressBar::show(
            ui,
            0.5,
            "Menghitung ukuran folder...",
            &format!("{} file dipindai — {}", state.scanned_count, label),
        );
    }

    fn render_results(state: &mut AnalyzerState, ui: &mut egui::Ui) {
        // Header: breadcrumb + actions
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal_wrapped(|ui| {
                    render_icon(ui, IconKind::ChartBar, 16.0, COLOR_BRAND_ACCENT);
                    ui.add_space(SPACE_XS);
                    let stack = state.path_stack.clone();
                    for (idx, path) in stack.iter().enumerate() {
                        let name = path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| path.display().to_string());
                        let is_last = idx == stack.len() - 1;
                        let text = RichText::new(name)
                            .size(12.5)
                            .color(if is_last { COLOR_TEXT_PRIMARY } else { COLOR_MUTED_TEXT })
                            .strong();
                        if ui.add(egui::Label::new(text).sense(egui::Sense::click())).clicked()
                            && !is_last
                        {
                            state.go_to_breadcrumb(idx);
                        }
                        if !is_last {
                            ui.label(RichText::new("/").color(COLOR_MUTED_TEXT).size(12.5));
                        }
                    }
                });
                ui.add_space(2.0);
                ui.label(
                    RichText::new(format!(
                        "{} — {} item",
                        format_bytes(state.total_size),
                        state.entries.len()
                    ))
                    .color(COLOR_MUTED_TEXT)
                    .size(12.0),
                );
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(
                        egui::Button::new(RichText::new("Pilih Folder Lain").size(12.0).color(COLOR_TEXT_PRIMARY))
                            .fill(COLOR_CARD_BG)
                            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                            .rounding(Rounding::same(RADIUS_SM)),
                    )
                    .clicked()
                {
                    state.reset();
                }
                if state.path_stack.len() > 1
                    && ui
                        .add(
                            egui::Button::new(RichText::new("Muat Ulang").size(12.0).color(COLOR_TEXT_PRIMARY))
                                .fill(COLOR_CARD_BG)
                                .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                                .rounding(Rounding::same(RADIUS_SM)),
                        )
                        .clicked()
                {
                    state.start_scan_current();
                }
            });
        });

        ui.add_space(SPACE_MD);

        if state.entries.is_empty() {
            EmptyState::show(
                ui,
                IconKind::Folder,
                "Folder kosong",
                "Tidak ada file atau folder di dalam direktori ini.",
            );
            return;
        }

        let max_size = state.entries.iter().map(|e| e.size).max().unwrap_or(1).max(1);
        let mut entry_to_enter: Option<PathBuf> = None;
        let mut path_to_open: Option<PathBuf> = None;

        egui::ScrollArea::vertical().show(ui, |ui| {
            for entry in &state.entries {
                let ratio = entry.size as f32 / max_size as f32;
                let row_resp = egui::Frame::none()
                    .fill(COLOR_CARD_BG)
                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                    .rounding(Rounding::same(RADIUS_MD))
                    .inner_margin(egui::Margin::symmetric(12.0, 10.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            render_icon(
                                ui,
                                if entry.is_dir { IconKind::Folder } else { IconKind::FileText },
                                16.0,
                                if entry.is_dir { COLOR_BRAND_ACCENT } else { COLOR_MUTED_TEXT },
                            );
                            ui.add_space(SPACE_XS);

                            ui.vertical(|ui| {
                                ui.set_width(ui.available_width() - 190.0);
                                ui.label(
                                    RichText::new(&entry.name)
                                        .color(COLOR_TEXT_PRIMARY)
                                        .size(13.0)
                                        .strong(),
                                );
                                let (bar_rect, _) = ui.allocate_exact_size(
                                    Vec2::new(ui.available_width(), 6.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter().rect_filled(bar_rect, Rounding::same(3.0), COLOR_PANEL_BG);
                                let mut fill_rect = bar_rect;
                                fill_rect.set_width(bar_rect.width() * ratio.clamp(0.01, 1.0));
                                ui.painter().rect_filled(fill_rect, Rounding::same(3.0), COLOR_BRAND_ACCENT);
                            });

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui
                                    .add(
                                        egui::Button::new(RichText::new("Buka").size(11.0).color(COLOR_TEXT_PRIMARY))
                                            .fill(COLOR_CARD_HOVER)
                                            .rounding(Rounding::same(RADIUS_SM)),
                                    )
                                    .clicked()
                                {
                                    path_to_open = Some(entry.path.clone());
                                }
                                ui.add_space(SPACE_SM);
                                ui.label(
                                    RichText::new(format_bytes(entry.size))
                                        .color(COLOR_TEXT_PRIMARY)
                                        .strong()
                                        .size(13.0),
                                );
                            });
                        });
                    })
                    .response;

                if entry.is_dir
                    && row_resp
                        .interact(egui::Sense::click())
                        .on_hover_text("Klik untuk masuk ke folder ini")
                        .clicked()
                {
                    entry_to_enter = Some(entry.path.clone());
                }
                ui.add_space(SPACE_XS);
            }
        });

        if let Some(p) = entry_to_enter {
            state.enter(p);
        }
        if let Some(p) = path_to_open {
            crate::platform::open_in_file_manager(&p);
        }
    }
}
