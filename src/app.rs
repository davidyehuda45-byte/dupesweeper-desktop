use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use crate::actions::{ActionKind, ActionReport, DeleteProgressEvent, DeleteWorker};
use crate::scanner::{
    DuplicateGroup, ScanProgress, Scanner, SelectionStrategy, TemplateCategory, WalkerConfig,
};
use crate::ui::cleanup_view::{CleanupState, CleanupView};
use crate::ui::components::{Badge, EmptyState, ModernProgressBar};
use crate::ui::splash_screen::SplashScreen;
use crate::ui::theme::{
    file_extension_category, format_bytes, format_system_time, setup_custom_theme,
    COLOR_ACCENT_HOVER, COLOR_ACCENT_PRIMARY, COLOR_BG_DARK, COLOR_BORDER, COLOR_CARD_BG,
    COLOR_DELETE_BG, COLOR_DELETE_TEXT, COLOR_KEEP_BG, COLOR_KEEP_TEXT, COLOR_MUTED_TEXT,
    COLOR_PANEL_BG, COLOR_SENSITIVE_BG, COLOR_SENSITIVE_TEXT,
};
use crate::ui::thumbnail::ThumbnailCache;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AppMode {
    DuplicateFinder,
    GeneralCleanup,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppScreen {
    Splash,
    Setup,
    Scanning,
    Results,
    Deleting,
    Completed,
}

#[derive(Clone, Copy, Debug)]
enum DisplayRow {
    GroupHeader { group_idx: usize, is_expanded: bool },
    FileRow { group_idx: usize, file_idx: usize },
}

pub struct DupeSweeperApp {
    // Current application mode
    mode: AppMode,

    // General cleanup mode state
    cleanup_state: CleanupState,

    // Splash screen state
    splash: SplashScreen,

    // Current screen state for duplicate finder
    screen: AppScreen,

    // Background deletion state
    delete_cancel_flag: Arc<AtomicBool>,
    delete_rx: Option<Receiver<DeleteProgressEvent>>,
    delete_current: usize,
    delete_total: usize,
    delete_current_file: String,
    delete_bytes_freed: u64,

    // Scan settings
    roots: Vec<PathBuf>,
    exclude_dirs: Vec<PathBuf>,
    exclude_exts_input: String,
    min_size_kb: u64,
    include_hidden: bool,
    scan_all_folders: bool,

    // Scanning state & communication
    cancel_flag: Arc<AtomicBool>,
    scan_rx: Option<Receiver<ScanProgress>>,
    scan_stage_name: String,
    scan_file_detail: String,
    scan_progress_ratio: f32,
    scan_items_stat: String,
    scan_bytes_stat: String,
    scan_start_instant: Option<Instant>,

    // Results state
    groups: Vec<DuplicateGroup>,
    expanded_groups: HashSet<usize>,
    total_files_scanned: usize,
    folders_skipped: usize,
    filter_query: String,
    current_strategy: SelectionStrategy,

    // Modal state for confirmation
    show_confirm_dialog: bool,
    selected_action_kind: ActionOption,
    quarantine_dir: PathBuf,
    permanent_delete_acknowledged: bool,
    sensitive_delete_acknowledged: bool,

    // Completion / Report state
    last_report: Option<ActionReport>,

    // Thumbnail cache with background worker
    thumb_cache: ThumbnailCache,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ActionOption {
    RecycleBin,
    Quarantine,
    PermanentDelete,
}

impl Default for DupeSweeperApp {
    fn default() -> Self {
        Self {
            mode: AppMode::DuplicateFinder,
            cleanup_state: CleanupState::default(),
            splash: SplashScreen::new(650),
            screen: AppScreen::Splash,
            delete_cancel_flag: Arc::new(AtomicBool::new(false)),
            delete_rx: None,
            delete_current: 0,
            delete_total: 0,
            delete_current_file: String::new(),
            delete_bytes_freed: 0,

            roots: Vec::new(),
            exclude_dirs: Vec::new(),
            exclude_exts_input: String::new(),
            min_size_kb: 1,
            include_hidden: false,
            scan_all_folders: false,

            cancel_flag: Arc::new(AtomicBool::new(false)),
            scan_rx: None,
            scan_stage_name: String::new(),
            scan_file_detail: String::new(),
            scan_progress_ratio: 0.0,
            scan_items_stat: String::new(),
            scan_bytes_stat: String::new(),
            scan_start_instant: None,

            groups: Vec::new(),
            expanded_groups: HashSet::new(),
            total_files_scanned: 0,
            folders_skipped: 0,
            filter_query: String::new(),
            current_strategy: SelectionStrategy::KeepOldest,

            show_confirm_dialog: false,
            selected_action_kind: ActionOption::RecycleBin,
            quarantine_dir: dirs_fallback_quarantine(),
            permanent_delete_acknowledged: false,
            sensitive_delete_acknowledged: false,

            last_report: None,
            thumb_cache: ThumbnailCache::new(),
        }
    }
}

fn dirs_fallback_quarantine() -> PathBuf {
    let mut p = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    p.push("DupeSweeper_Quarantine");
    p
}

impl DupeSweeperApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_theme(&cc.egui_ctx);
        Self::default()
    }

    fn start_scan(&mut self) {
        if self.roots.is_empty() {
            return;
        }

        self.cancel_flag = Arc::new(AtomicBool::new(false));
        let cancel_flag_clone = Arc::clone(&self.cancel_flag);
        let (tx, rx): (Sender<ScanProgress>, Receiver<ScanProgress>) = unbounded();
        self.scan_rx = Some(rx);
        self.expanded_groups.clear();

        let exclude_extensions = self
            .exclude_exts_input
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let config = WalkerConfig {
            roots: self.roots.clone(),
            exclude_dirs: self.exclude_dirs.clone(),
            exclude_extensions,
            min_size: self.min_size_kb * 1024,
            max_size: None,
            include_hidden: self.include_hidden,
            scan_all_folders: self.scan_all_folders,
        };

        self.screen = AppScreen::Scanning;
        self.scan_stage_name = "Membaca struktur folder...".to_string();
        self.scan_file_detail = "Memulai scan...".to_string();
        self.scan_progress_ratio = 0.0;
        self.scan_items_stat = "0 file ditemukan".to_string();
        self.scan_bytes_stat = String::new();
        self.scan_start_instant = Some(Instant::now());

        std::thread::spawn(move || {
            Scanner::run(config, cancel_flag_clone, tx);
        });
    }

    fn cancel_scan(&mut self) {
        self.cancel_flag.store(true, Ordering::Relaxed);
        self.scan_stage_name = "Membatalkan scan...".to_string();
    }

    fn open_in_explorer(path: &Path) {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("explorer")
                .arg(format!("/select,{}", path.display()))
                .spawn();
        }
        #[cfg(not(target_os = "windows"))]
        {
            if let Some(parent) = path.parent() {
                let _ = std::process::Command::new("xdg-open").arg(parent).spawn();
            }
        }
    }

    fn open_file(path: &Path) {
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("cmd")
                .args(["/c", "start", "", &path.to_string_lossy()])
                .spawn();
        }
        #[cfg(not(target_os = "windows"))]
        {
            let _ = std::process::Command::new("xdg-open").arg(path).spawn();
        }
    }

    fn apply_global_strategy(&mut self, strategy: SelectionStrategy) {
        self.current_strategy = strategy;
        for group in &mut self.groups {
            group.apply_strategy(strategy);
        }
    }

    fn select_all_duplicates(&mut self) {
        for group in &mut self.groups {
            group.select_all_duplicates();
        }
    }

    fn deselect_all_files(&mut self) {
        for group in &mut self.groups {
            group.deselect_all();
        }
    }

    fn get_selected_files_and_bytes(&self) -> (Vec<crate::scanner::FileItem>, u64) {
        let mut list = Vec::new();
        let mut bytes = 0;
        for g in &self.groups {
            for f in &g.files {
                if f.is_selected {
                    list.push(f.clone());
                    bytes += f.size;
                }
            }
        }
        (list, bytes)
    }

    fn execute_selected_action(&mut self) {
        let (selected_files, _) = self.get_selected_files_and_bytes();
        if selected_files.is_empty() {
            return;
        }

        let kind = match self.selected_action_kind {
            ActionOption::RecycleBin => ActionKind::RecycleBin,
            ActionOption::Quarantine => ActionKind::Quarantine(self.quarantine_dir.clone()),
            ActionOption::PermanentDelete => ActionKind::PermanentDelete,
        };

        self.delete_cancel_flag = Arc::new(AtomicBool::new(false));
        self.delete_current = 0;
        self.delete_total = selected_files.len();
        self.delete_current_file = "Menyiapkan antrian pembersihan...".to_string();
        self.delete_bytes_freed = 0;

        let rx = DeleteWorker::start(
            kind,
            selected_files,
            Arc::clone(&self.delete_cancel_flag),
        );
        self.delete_rx = Some(rx);
        self.show_confirm_dialog = false;
        self.screen = AppScreen::Deleting;
    }

    fn finalize_deletion(&mut self, report: ActionReport) {
        self.delete_rx = None;

        // Remove deleted files from groups (files that no longer exist on disk)
        for group in &mut self.groups {
            group.files.retain(|f| f.path.exists());
        }
        // Remove groups that now have < 2 files
        self.groups.retain(|g| g.files.len() >= 2);

        self.last_report = Some(report);
        self.screen = AppScreen::Completed;
    }
}

impl eframe::App for DupeSweeperApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Render splash screen if in splash state
        if self.screen == AppScreen::Splash {
            if self.splash.is_finished() {
                self.screen = AppScreen::Setup;
            } else {
                egui::CentralPanel::default()
                    .frame(egui::Frame::none().fill(COLOR_BG_DARK))
                    .show(ctx, |ui| {
                        self.splash.render(ctx, ui);
                    });
                return;
            }
        }

        // Non-blocking upload of background decoded thumbnails
        self.thumb_cache.update(ctx);

        // Handle drag and drop files onto window
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                for drop in &i.raw.dropped_files {
                    if let Some(path) = &drop.path {
                        if path.is_dir() && !self.roots.contains(path) {
                            self.roots.push(path.clone());
                        }
                    }
                }
            }
        });

        // Receive scanning background messages safely
        let mut finished_groups = None;
        let mut finished_files_scanned = 0;
        let mut finished_folders_skipped = 0;
        let mut reset_to_setup = false;

        if let Some(ref rx) = self.scan_rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    ScanProgress::Walking {
                        files_scanned,
                        current_path,
                    } => {
                        self.scan_stage_name = "Tahap 1: Membaca struktur direktori...".to_string();
                        self.scan_file_detail = current_path;
                        self.scan_items_stat = format!("{} file terbaca", files_scanned);
                        self.scan_progress_ratio = 0.15;
                    }
                    ScanProgress::PartialHashing {
                        current,
                        total,
                        current_file,
                    } => {
                        self.scan_stage_name =
                            "Tahap 2: Quick Hash 4KB (Eliminasi False Candidates)...".to_string();
                        self.scan_file_detail = current_file;
                        self.scan_items_stat = format!("{} / {} file", current, total);
                        let ratio = if total > 0 {
                            current as f32 / total as f32
                        } else {
                            0.0
                        };
                        self.scan_progress_ratio = 0.15 + (ratio * 0.35);
                    }
                    ScanProgress::FullHashing {
                        current,
                        total,
                        current_file,
                        bytes_hashed,
                        total_candidate_bytes,
                    } => {
                        self.scan_stage_name =
                            "Tahap 3: Deep BLAKE3 Streaming Hash (100% Exact Match)...".to_string();
                        self.scan_file_detail = current_file;
                        self.scan_items_stat = format!("{} / {} file terverifikasi", current, total);
                        self.scan_bytes_stat = format!(
                            "{} / {} diproses",
                            format_bytes(bytes_hashed),
                            format_bytes(total_candidate_bytes)
                        );
                        let ratio = if total > 0 {
                            current as f32 / total as f32
                        } else {
                            0.0
                        };
                        self.scan_progress_ratio = 0.50 + (ratio * 0.50);
                    }
                    ScanProgress::Finished {
                        groups,
                        total_files_scanned,
                        folders_skipped,
                        total_duplicates: _,
                        wasted_bytes: _,
                        elapsed: _,
                    } => {
                        finished_groups = Some(groups);
                        finished_files_scanned = total_files_scanned;
                        finished_folders_skipped = folders_skipped;
                    }
                    ScanProgress::Cancelled | ScanProgress::Error(_) => {
                        reset_to_setup = true;
                    }
                }
            }
        }

        if let Some(groups) = finished_groups {
            self.groups = groups;
            self.expanded_groups.clear();
            self.total_files_scanned = finished_files_scanned;
            self.folders_skipped = finished_folders_skipped;
            self.scan_rx = None;
            self.screen = AppScreen::Results;
        } else if reset_to_setup {
            self.scan_rx = None;
            self.screen = AppScreen::Setup;
        }

        // Receive deletion background messages safely
        let mut deletion_finished_report = None;
        if let Some(ref rx) = self.delete_rx {
            while let Ok(msg) = rx.try_recv() {
                match msg {
                    DeleteProgressEvent::Started { total } => {
                        self.delete_current = 0;
                        self.delete_total = total;
                        self.delete_current_file = "Memulai proses pembersihan...".to_string();
                        self.delete_bytes_freed = 0;
                    }
                    DeleteProgressEvent::Progress {
                        current,
                        total,
                        current_file,
                        bytes_freed,
                    } => {
                        self.delete_current = current;
                        self.delete_total = total;
                        self.delete_current_file = current_file;
                        self.delete_bytes_freed = bytes_freed;
                    }
                    DeleteProgressEvent::Finished { report } => {
                        deletion_finished_report = Some(report);
                    }
                    DeleteProgressEvent::Cancelled { partial_report } => {
                        deletion_finished_report = Some(partial_report);
                    }
                }
            }
        }

        if let Some(report) = deletion_finished_report {
            self.finalize_deletion(report);
        }

        // Request repaint while scanning or deleting so progress bar is smooth
        if self.screen == AppScreen::Scanning || self.screen == AppScreen::Deleting {
            ctx.request_repaint();
        }

        // Top Navigation Bar
        egui::TopBottomPanel::top("top_header")
            .frame(egui::Frame::none().fill(COLOR_PANEL_BG).inner_margin(16.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading(
                        RichText::new("⚡ DupeSweeper")
                            .color(Color32::WHITE)
                            .size(20.0)
                            .strong(),
                    );
                    ui.label(
                        RichText::new("v5.0.0 • 100% Offline")
                            .color(COLOR_MUTED_TEXT)
                            .size(13.0),
                    );

                    ui.add_space(20.0);

                    // Mode switch tabs
                    let dup_tab = ui.selectable_label(
                        self.mode == AppMode::DuplicateFinder,
                        RichText::new("🔍 Cari File Duplikat").strong().size(13.0),
                    );
                    if dup_tab.clicked() {
                        self.mode = AppMode::DuplicateFinder;
                    }

                    let clean_tab = ui.selectable_label(
                        self.mode == AppMode::GeneralCleanup,
                        RichText::new("🧹 Bersihkan Sampah Sistem").strong().size(13.0),
                    );
                    if clean_tab.clicked() {
                        self.mode = AppMode::GeneralCleanup;
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if self.mode == AppMode::DuplicateFinder && self.screen == AppScreen::Results {
                            if ui
                                .button(RichText::new("🔄 Scan Baru").color(Color32::WHITE))
                                .clicked()
                            {
                                self.screen = AppScreen::Setup;
                                self.groups.clear();
                                self.expanded_groups.clear();
                                self.thumb_cache.clear();
                            }
                        }
                    });
                });
            });

        // Main Central View
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(COLOR_CARD_BG).inner_margin(20.0))
            .show(ctx, |ui| match self.mode {
                AppMode::DuplicateFinder => match self.screen {
                    AppScreen::Splash => {}
                    AppScreen::Setup => self.render_setup_view(ctx, ui),
                    AppScreen::Scanning => self.render_scanning_view(ui),
                    AppScreen::Results => self.render_results_view(ctx, ui),
                    AppScreen::Deleting => self.render_deleting_view(ui),
                    AppScreen::Completed => self.render_completed_view(ui),
                },
                AppMode::GeneralCleanup => {
                    CleanupView::render(&mut self.cleanup_state, ctx, ui);
                }
            });

        // Modal Confirmation Dialog
        if self.show_confirm_dialog {
            self.render_confirmation_modal(ctx);
        }
    }
}

// ----------------------------------------------------------------------------
// Screen Implementations
// ----------------------------------------------------------------------------
impl DupeSweeperApp {
    fn render_setup_view(&mut self, _ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(10.0);
            ui.heading(
                RichText::new("Cari & Bersihkan File Duplikat")
                    .size(24.0)
                    .strong(),
            );
            ui.label(
                RichText::new(
                    "Pilih satu atau beberapa folder/drive untuk di-scan secara mendalam dengan BLAKE3.",
                )
                .color(COLOR_MUTED_TEXT),
            );
            ui.add_space(16.0);
        });

        // Drop Zone Card
        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .stroke(Stroke::new(1.5_f32, COLOR_BORDER))
            .rounding(Rounding::same(8.0))
            .inner_margin(24.0)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("📁 Drag & drop folder ke sini, atau klik tombol di bawah")
                            .size(16.0)
                            .color(Color32::WHITE),
                    );
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.add_space((ui.available_width() - 260.0) / 2.0);
                        if ui
                            .add_sized(
                                [260.0, 36.0],
                                egui::Button::new(
                                    RichText::new("➕ Pilih Folder / Drive...")
                                        .color(Color32::WHITE)
                                        .strong(),
                                )
                                .fill(COLOR_ACCENT_PRIMARY),
                            )
                            .clicked()
                        {
                            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                                if !self.roots.contains(&folder) {
                                    self.roots.push(folder);
                                }
                            }
                        }
                    });
                });
            });

        ui.add_space(16.0);

        // Selected Roots List
        ui.label(
            RichText::new(format!("Folder yang akan di-scan ({})", self.roots.len()))
                .strong()
                .size(14.0),
        );
        ui.add_space(4.0);

        if self.roots.is_empty() {
            ui.label(
                RichText::new("Belum ada folder dipilih. Tambahkan minimal 1 folder untuk memulai.")
                    .color(COLOR_MUTED_TEXT)
                    .italics(),
            );
        } else {
            egui::ScrollArea::vertical()
                .max_height(140.0)
                .show(ui, |ui| {
                    let mut remove_idx = None;
                    for (i, root) in self.roots.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label("📁");
                            ui.label(RichText::new(root.display().to_string()).strong());
                            if ui.small_button("✕ Hapus").clicked() {
                                remove_idx = Some(i);
                            }
                        });
                    }
                    if let Some(idx) = remove_idx {
                        self.roots.remove(idx);
                    }
                });
        }

        ui.add_space(16.0);

        // Scan All Toggle
        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
            .rounding(Rounding::same(6.0))
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.checkbox(
                    &mut self.scan_all_folders,
                    RichText::new("Scan semua folder (termasuk dependency node_modules/vendor/target — untuk audit disk)").color(Color32::WHITE),
                );
                ui.label(
                    RichText::new("Secara default, folder dependency/build otomatis di-skip untuk kecepatan & proteksi file project.")
                        .color(COLOR_MUTED_TEXT)
                        .size(11.0),
                );
            });

        ui.add_space(10.0);

        // Advanced Settings Collapsible
        ui.collapsing("⚙ Pengaturan Scan Tambahan (Opsional)", |ui| {
            ui.horizontal(|ui| {
                ui.label("Ukuran File Minimum (KB):");
                ui.add(egui::DragValue::new(&mut self.min_size_kb).range(0..=1_000_000));
                ui.label(
                    RichText::new("(File lebih kecil dari ini akan dilewati)")
                        .color(COLOR_MUTED_TEXT),
                );
            });

            ui.add_space(6.0);
            ui.checkbox(&mut self.include_hidden, "Scan file/folder tersembunyi (hidden)");

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label("Abaikan Ekstensi (pisahkan dengan koma):");
                ui.text_edit_singleline(&mut self.exclude_exts_input);
                ui.label(RichText::new("Contoh: tmp, bak, log").color(COLOR_MUTED_TEXT));
            });
        });

        ui.add_space(20.0);

        // Big Start Scan Button
        ui.vertical_centered(|ui| {
            let can_scan = !self.roots.is_empty();
            ui.add_enabled_ui(can_scan, |ui| {
                let btn = ui.add_sized(
                    [320.0, 48.0],
                    egui::Button::new(
                        RichText::new("🚀 Mulai Scan Duplikat")
                            .size(16.0)
                            .color(Color32::WHITE)
                            .strong(),
                    )
                    .fill(if can_scan {
                        COLOR_ACCENT_PRIMARY
                    } else {
                        Color32::from_rgb(60, 65, 80)
                    }),
                );

                if btn.clicked() {
                    self.start_scan();
                }
            });
        });
    }

    fn render_scanning_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading(
                RichText::new("Sedang Memindai File Duplikat...")
                    .size(24.0)
                    .strong(),
            );
            ui.add_space(8.0);
            ui.label(
                RichText::new(&self.scan_stage_name)
                    .color(Color32::WHITE)
                    .size(15.0),
            );
            ui.add_space(20.0);

            // Modern Progress Bar
            let sub_text = if !self.scan_bytes_stat.is_empty() {
                format!("{} • {}", self.scan_items_stat, self.scan_bytes_stat)
            } else {
                self.scan_items_stat.clone()
            };

            ModernProgressBar::show(
                ui,
                self.scan_progress_ratio,
                &format!("{:.0}%", self.scan_progress_ratio * 100.0),
                &sub_text,
            );

            ui.add_space(14.0);
            ui.label(
                RichText::new(format!("File saat ini: {}", self.scan_file_detail))
                    .color(COLOR_MUTED_TEXT)
                    .size(12.0),
            );

            if let Some(start) = self.scan_start_instant {
                ui.add_space(6.0);
                ui.label(
                    RichText::new(format!("Waktu berjalan: {:.1} detik", start.elapsed().as_secs_f32()))
                        .color(COLOR_MUTED_TEXT)
                        .size(12.0),
                );
            }

            ui.add_space(30.0);
            if ui
                .add_sized(
                    [200.0, 36.0],
                    egui::Button::new(
                        RichText::new("⏹ Batalkan Scan")
                            .color(Color32::WHITE)
                            .strong(),
                    )
                    .fill(COLOR_DELETE_BG),
                )
                .clicked()
            {
                self.cancel_scan();
            }
        });
    }

    fn render_deleting_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading(
                RichText::new("Sedang Membersihkan File Duplikat...")
                    .size(24.0)
                    .strong(),
            );
            ui.add_space(8.0);

            let progress_ratio = if self.delete_total > 0 {
                (self.delete_current as f32 / self.delete_total as f32).clamp(0.0, 1.0)
            } else {
                0.0
            };

            let is_cancelling = self.delete_cancel_flag.load(Ordering::Relaxed);
            let status_text = if is_cancelling {
                "Membatalkan pembersihan... menyelesaikan item saat ini.".to_string()
            } else {
                format!("Memproses {} dari {} file", self.delete_current, self.delete_total)
            };

            ui.label(
                RichText::new(status_text)
                    .color(Color32::WHITE)
                    .size(15.0),
            );
            ui.add_space(20.0);

            ModernProgressBar::show(
                ui,
                progress_ratio,
                &format!("{:.0}%", progress_ratio * 100.0),
                &format!("Ruang dibebaskan: {}", format_bytes(self.delete_bytes_freed)),
            );

            ui.add_space(14.0);
            ui.label(
                RichText::new(format!("File saat ini: {}", self.delete_current_file))
                    .color(COLOR_MUTED_TEXT)
                    .size(12.0),
            );

            ui.add_space(30.0);
            ui.add_enabled_ui(!is_cancelling, |ui| {
                if ui
                    .add_sized(
                        [200.0, 36.0],
                        egui::Button::new(
                            RichText::new(if is_cancelling { "Membatalkan..." } else { "⏹ Batalkan" })
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(COLOR_DELETE_BG),
                    )
                    .clicked()
                {
                    self.delete_cancel_flag.store(true, Ordering::Relaxed);
                }
            });
        });
    }

    fn render_results_view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if self.groups.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(60.0);
                EmptyState::show(
                    ui,
                    "🎉",
                    "Tidak Ada File Duplikat!",
                    "Penyimpanan Anda bersih dan teratur. Tidak ditemukan file duplikat dalam folder yang dipilih.",
                );
                ui.add_space(24.0);
                if ui
                    .add_sized(
                        [200.0, 40.0],
                        egui::Button::new(
                            RichText::new("🔄 Scan Folder Lain")
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(COLOR_ACCENT_PRIMARY),
                    )
                    .clicked()
                {
                    self.screen = AppScreen::Setup;
                    self.groups.clear();
                    self.roots.clear();
                }
            });
            return;
        }

        let total_groups = self.groups.len();
        let total_duplicates: usize = self.groups.iter().map(|g| g.files.len().saturating_sub(1)).sum();
        let total_wasted_bytes: u64 = self.groups.iter().map(|g| g.total_wasted_bytes()).sum();

        let (selected_files, selected_bytes) = self.get_selected_files_and_bytes();

        // Top Summary Card
        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
            .rounding(Rounding::same(8.0))
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading(
                            RichText::new(format!(
                                "🎉 Ditemukan {} Grup Duplikat ({} File)",
                                total_groups, total_duplicates
                            ))
                            .size(18.0)
                            .color(Color32::WHITE),
                        );
                        ui.label(
                            RichText::new(format!(
                                "Total potensi ruang hemat: {} • Total file di-scan: {}",
                                format_bytes(total_wasted_bytes),
                                self.total_files_scanned
                            ))
                            .color(COLOR_MUTED_TEXT),
                        );
                        if self.folders_skipped > 0 {
                            ui.label(
                                RichText::new(format!(
                                    "🚫 {} folder dependency/build di-skip otomatis (menghemat waktu scan)",
                                    self.folders_skipped
                                ))
                                .color(COLOR_ACCENT_HOVER)
                                .size(12.0),
                            );
                        }
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let can_clean = !selected_files.is_empty();
                        let clean_text = format!(
                            "🗑 Bersihkan {} File ({})",
                            selected_files.len(),
                            format_bytes(selected_bytes)
                        );
                        ui.add_enabled_ui(can_clean, |ui| {
                            let clean_btn = ui.add_sized(
                                [260.0, 40.0],
                                egui::Button::new(
                                    RichText::new(clean_text)
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

                            if clean_btn.clicked() {
                                self.permanent_delete_acknowledged = false;
                                self.sensitive_delete_acknowledged = false;
                                self.show_confirm_dialog = true;
                            }
                        });
                    });
                });
            });

        ui.add_space(8.0);

        // Action & Strategy Bar
        let filter_lower = self.filter_query.to_lowercase();
        let matching_indices: Vec<usize> = self
            .groups
            .iter()
            .enumerate()
            .filter_map(|(idx, group)| {
                if filter_lower.is_empty() {
                    Some(idx)
                } else {
                    let match_found = group.files.iter().any(|f| {
                        f.file_name().to_lowercase().contains(&filter_lower)
                            || f.path.to_string_lossy().to_lowercase().contains(&filter_lower)
                    });
                    if match_found {
                        Some(idx)
                    } else {
                        None
                    }
                }
            })
            .collect();

        ui.horizontal(|ui| {
            ui.label(RichText::new("Seleksi:").strong());

            if ui
                .selectable_label(
                    self.current_strategy == SelectionStrategy::KeepOldest,
                    "Simpan Terlama",
                )
                .clicked()
            {
                self.apply_global_strategy(SelectionStrategy::KeepOldest);
            }

            if ui
                .selectable_label(
                    self.current_strategy == SelectionStrategy::KeepNewest,
                    "Simpan Terbaru",
                )
                .clicked()
            {
                self.apply_global_strategy(SelectionStrategy::KeepNewest);
            }

            if ui
                .selectable_label(
                    self.current_strategy == SelectionStrategy::KeepShortestPath,
                    "Simpan Path Terpendek",
                )
                .clicked()
            {
                self.apply_global_strategy(SelectionStrategy::KeepShortestPath);
            }

            ui.separator();

            if ui.small_button("Pilih Semua").clicked() {
                self.select_all_duplicates();
            }

            if ui.small_button("Hapus Centang").clicked() {
                self.deselect_all_files();
            }

            ui.separator();

            // Collapse / Expand All controls
            if ui.small_button("▶ Buka Semua").clicked() {
                for &idx in &matching_indices {
                    self.expanded_groups.insert(idx);
                }
            }

            if ui.small_button("▼ Tutup Semua").clicked() {
                self.expanded_groups.clear();
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.filter_query)
                        .hint_text("🔍 Cari nama file..."),
                );
            });
        });

        ui.add_space(8.0);

        // Build flat virtual rows for display
        let mut display_rows: Vec<DisplayRow> = Vec::new();
        for &g_idx in &matching_indices {
            let is_expanded = self.expanded_groups.contains(&g_idx);
            display_rows.push(DisplayRow::GroupHeader {
                group_idx: g_idx,
                is_expanded,
            });

            if is_expanded {
                if let Some(group) = self.groups.get(g_idx) {
                    for f_idx in 0..group.files.len() {
                        display_rows.push(DisplayRow::FileRow {
                            group_idx: g_idx,
                            file_idx: f_idx,
                        });
                    }
                }
            }
        }

        let total_rows = display_rows.len();
        const ROW_HEIGHT: f32 = 44.0;

        let mut toggle_expand_group = None;
        let mut open_explorer_path = None;
        let mut open_file_path = None;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show_rows(ui, ROW_HEIGHT, total_rows, |ui, row_range| {
                ui.spacing_mut().item_spacing = Vec2::new(4.0, 4.0);

                for row_idx in row_range {
                    if row_idx >= display_rows.len() {
                        continue;
                    }

                    match display_rows[row_idx] {
                        DisplayRow::GroupHeader {
                            group_idx,
                            is_expanded,
                        } => {
                            let group = &self.groups[group_idx];
                            let file_count = group.files.len();
                            let wasted = group.total_wasted_bytes();
                            let selected_count = group.selected_count();
                            let has_sensitive = group.files.iter().any(|f| f.is_sensitive);
                            let primary_name = group
                                .files
                                .first()
                                .map(|f| f.file_name())
                                .unwrap_or_default();
                            let hash_short = if group.hash.len() > 8 {
                                &group.hash[..8]
                            } else {
                                &group.hash
                            };

                            egui::Frame::none()
                                .fill(if is_expanded {
                                    Color32::from_rgb(38, 44, 58)
                                } else {
                                    COLOR_PANEL_BG
                                })
                                .stroke(Stroke::new(
                                    1.0_f32,
                                    if is_expanded {
                                        COLOR_ACCENT_PRIMARY
                                    } else {
                                        COLOR_BORDER
                                    },
                                ))
                                .rounding(Rounding::same(6.0))
                                .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                                .show(ui, |ui| {
                                    ui.set_height(ROW_HEIGHT - 12.0);
                                    ui.horizontal(|ui| {
                                        let arrow = if is_expanded { "▼" } else { "▶" };
                                        if ui.button(RichText::new(arrow).size(12.0).strong()).clicked() {
                                            toggle_expand_group = Some(group_idx);
                                        }

                                        ui.label(
                                            RichText::new(format!("Grup #{}", group_idx + 1))
                                                .strong()
                                                .color(COLOR_ACCENT_PRIMARY),
                                        );

                                        let name_btn = ui.selectable_label(
                                            false,
                                            RichText::new(&primary_name)
                                                .color(Color32::WHITE)
                                                .strong(),
                                        );
                                        if name_btn.clicked() {
                                            toggle_expand_group = Some(group_idx);
                                        }

                                        ui.label(
                                            RichText::new(format!("({} salinan)", file_count))
                                                .color(COLOR_MUTED_TEXT)
                                                .size(12.0),
                                        );

                                        ui.label(
                                            RichText::new(format!(
                                                "• {}/file • Hemat: {}",
                                                format_bytes(group.file_size),
                                                format_bytes(wasted)
                                            ))
                                            .color(Color32::WHITE)
                                            .size(12.0),
                                        );

                                        if has_sensitive {
                                            Badge::show(ui, "⚠️ Sensitif", COLOR_SENSITIVE_BG, COLOR_SENSITIVE_TEXT);
                                        }

                                        if let Some(tm) = group.files.first().and_then(|f| f.template_match.as_ref()) {
                                            let (badge_txt, badge_bg, badge_fg) = match tm.category {
                                                TemplateCategory::Documentation => (
                                                    format!("🗑️ Template ({})", tm.framework),
                                                    Color32::from_rgb(40, 52, 48),
                                                    Color32::from_rgb(140, 215, 180),
                                                ),
                                                TemplateCategory::Functional => (
                                                    format!("📋 Bawaan ({})", tm.framework),
                                                    Color32::from_rgb(32, 45, 65),
                                                    Color32::from_rgb(130, 180, 240),
                                                ),
                                            };
                                            Badge::show(ui, &badge_txt, badge_bg, badge_fg);
                                        }

                                        if selected_count > 0 {
                                            Badge::show(
                                                ui,
                                                &format!("{}/{} dipilih", selected_count, file_count),
                                                COLOR_DELETE_BG,
                                                COLOR_DELETE_TEXT,
                                            );
                                        }

                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                ui.label(
                                                    RichText::new(format!("Hash: {}...", hash_short))
                                                        .color(COLOR_MUTED_TEXT)
                                                        .size(11.0),
                                                );
                                            },
                                        );
                                    });
                                });
                        }
                        DisplayRow::FileRow {
                            group_idx,
                            file_idx,
                        } => {
                            let group = &mut self.groups[group_idx];
                            let file = &mut group.files[file_idx];

                            egui::Frame::none()
                                .fill(Color32::from_rgb(22, 25, 33))
                                .stroke(Stroke::new(0.5_f32, Color32::from_rgb(45, 50, 65)))
                                .rounding(Rounding::same(4.0))
                                .inner_margin(egui::Margin::symmetric(8.0, 4.0))
                                .show(ui, |ui| {
                                    ui.set_height(ROW_HEIGHT - 8.0);
                                    ui.horizontal(|ui| {
                                        ui.add_space(22.0); // Indent to align with group header

                                        ui.checkbox(&mut file.is_selected, "");

                                        // Async lazy-loaded thumbnail or category icon
                                        if let Some(texture) =
                                            self.thumb_cache.get_or_load(ctx, &file.path, 32)
                                        {
                                            ui.image((texture.id(), Vec2::new(24.0, 24.0)));
                                        } else {
                                            let (cat_text, cat_color) =
                                                file_extension_category(&file.path);
                                            egui::Frame::none()
                                                .fill(cat_color)
                                                .rounding(Rounding::same(3.0))
                                                .inner_margin(Vec2::new(5.0, 1.0))
                                                .show(ui, |ui| {
                                                    ui.label(
                                                        RichText::new(cat_text)
                                                            .size(10.0)
                                                            .color(Color32::WHITE)
                                                            .strong(),
                                                    );
                                                });
                                        }

                                        // Sensitive file badge
                                        if file.is_sensitive {
                                            Badge::show(ui, "⚠️ Sensitif", COLOR_SENSITIVE_BG, COLOR_SENSITIVE_TEXT);
                                        }

                                        // Template Fingerprint Match badge
                                        if let Some(ref tm) = file.template_match {
                                            let (badge_txt, badge_bg, badge_fg) = match tm.category {
                                                TemplateCategory::Documentation => (
                                                    format!("🗑️ Template Default ({})", tm.framework),
                                                    Color32::from_rgb(40, 52, 48),
                                                    Color32::from_rgb(140, 215, 180),
                                                ),
                                                TemplateCategory::Functional => (
                                                    format!("📋 Bawaan {} (Belum Dimodifikasi)", tm.framework),
                                                    Color32::from_rgb(32, 45, 65),
                                                    Color32::from_rgb(130, 180, 240),
                                                ),
                                            };
                                            Badge::show(ui, &badge_txt, badge_bg, badge_fg);
                                        }

                                        // Recommendation badge
                                        if file.is_recommended_keep {
                                            Badge::show(ui, "⭐ Simpan", COLOR_KEEP_BG, COLOR_KEEP_TEXT);
                                        } else if file.is_selected {
                                            Badge::show(ui, "Akan Dihapus", COLOR_DELETE_BG, COLOR_DELETE_TEXT);
                                        }

                                        ui.label(
                                            RichText::new(file.file_name()).strong().size(12.0),
                                        );

                                        ui.label(
                                            RichText::new(file.parent_dir_str())
                                                .color(COLOR_MUTED_TEXT)
                                                .size(11.0),
                                        );

                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Center),
                                            |ui| {
                                                if ui.small_button("👁 Preview").clicked() {
                                                    open_file_path = Some(file.path.clone());
                                                }
                                                if ui.small_button("📂 Buka").clicked() {
                                                    open_explorer_path = Some(file.path.clone());
                                                }
                                                ui.label(
                                                    RichText::new(format_system_time(file.modified))
                                                        .color(COLOR_MUTED_TEXT)
                                                        .size(11.0),
                                                );
                                            },
                                        );
                                    });
                                });
                        }
                    }
                }
            });

        if let Some(g_idx) = toggle_expand_group {
            if self.expanded_groups.contains(&g_idx) {
                self.expanded_groups.remove(&g_idx);
            } else {
                self.expanded_groups.insert(g_idx);
            }
        }

        if let Some(path) = open_explorer_path {
            Self::open_in_explorer(&path);
        }

        if let Some(path) = open_file_path {
            Self::open_file(&path);
        }
    }

    fn render_confirmation_modal(&mut self, ctx: &egui::Context) {
        let (selected_files, selected_bytes) = self.get_selected_files_and_bytes();
        let has_sensitive_selected = selected_files.iter().any(|f| f.is_sensitive);

        egui::Window::new("Konfirmasi Pembersihan File Duplikat")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([540.0, 420.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading(
                        RichText::new("Pilih Tindakan untuk File Terpilih")
                            .size(18.0)
                            .strong(),
                    );
                    ui.add_space(6.0);

                    ui.label(RichText::new(format!(
                        "Anda akan memproses {} file duplikat (total ruang: {}).",
                        selected_files.len(),
                        format_bytes(selected_bytes)
                    )));
                    ui.add_space(10.0);

                    // Warning for sensitive files (Kategori B)
                    if has_sensitive_selected {
                        egui::Frame::none()
                            .fill(COLOR_SENSITIVE_BG)
                            .stroke(Stroke::new(1.0_f32, COLOR_SENSITIVE_TEXT))
                            .rounding(Rounding::same(6.0))
                            .inner_margin(10.0)
                            .show(ui, |ui| {
                                ui.label(
                                    RichText::new("⚠️ PERINGATAN: TERDETEKSI FILE SENSITIF!")
                                        .color(COLOR_SENSITIVE_TEXT)
                                        .strong()
                                        .size(13.0),
                                );
                                ui.label(
                                    RichText::new(
                                        "Anda memilih menghapus file yang terdeteksi sebagai config/kredensial sensitif (.env, key, secrets, credentials, dll). File jenis ini biasanya TIDAK memiliki backup. Lanjutkan?",
                                    )
                                    .color(Color32::WHITE)
                                    .size(12.0),
                                );
                                ui.add_space(4.0);
                                ui.checkbox(
                                    &mut self.sensitive_delete_acknowledged,
                                    RichText::new("Ya, saya yakin ingin memproses/menghapus file sensitif ini").strong(),
                                );
                            });
                        ui.add_space(10.0);
                    }

                    // Option 1: Recycle Bin (Default)
                    ui.radio_value(
                        &mut self.selected_action_kind,
                        ActionOption::RecycleBin,
                        "♻ Pindahkan ke Recycle Bin (Aman & Disarankan)",
                    );
                    ui.label(
                        RichText::new("File dapat dipulihkan kapan saja dari Recycle Bin Windows.")
                            .color(COLOR_MUTED_TEXT)
                            .size(12.0),
                    );

                    ui.add_space(6.0);

                    // Option 2: Quarantine
                    ui.radio_value(
                        &mut self.selected_action_kind,
                        ActionOption::Quarantine,
                        "📦 Pindahkan ke Folder Karantina",
                    );
                    if self.selected_action_kind == ActionOption::Quarantine {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(self.quarantine_dir.display().to_string())
                                    .size(11.0),
                            );
                            if ui.small_button("Pilih Folder...").clicked() {
                                if let Some(p) = rfd::FileDialog::new().pick_folder() {
                                    self.quarantine_dir = p;
                                }
                            }
                        });
                    }

                    ui.add_space(6.0);

                    // Option 3: Permanent Delete
                    ui.radio_value(
                        &mut self.selected_action_kind,
                        ActionOption::PermanentDelete,
                        "⚠ Hapus Permanen (Data Tidak Dapat Dipulihkan)",
                    );

                    if self.selected_action_kind == ActionOption::PermanentDelete {
                        ui.checkbox(
                            &mut self.permanent_delete_acknowledged,
                            "Saya mengerti bahwa file akan dihapus permanen dari storage.",
                        );
                    }

                    ui.add_space(16.0);

                    // Buttons
                    ui.horizontal(|ui| {
                        let can_proceed = (!has_sensitive_selected || self.sensitive_delete_acknowledged)
                            && (self.selected_action_kind != ActionOption::PermanentDelete
                                || self.permanent_delete_acknowledged);

                        if ui.button("Batal").clicked() {
                            self.show_confirm_dialog = false;
                        }

                        if ui
                            .add_enabled(
                                can_proceed,
                                egui::Button::new(
                                    RichText::new("✓ Jalankan Sekarang")
                                        .color(Color32::WHITE)
                                        .strong(),
                                )
                                .fill(COLOR_ACCENT_PRIMARY),
                            )
                            .clicked()
                        {
                            self.execute_selected_action();
                        }
                    });
                });
            });
    }

    fn render_completed_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.heading(
                RichText::new("✨ Pembersihan Selesai!")
                    .size(24.0)
                    .color(Color32::from_rgb(34, 197, 94))
                    .strong(),
            );
            ui.add_space(12.0);

            if let Some(ref report) = self.last_report {
                ui.label(
                    RichText::new(format!(
                        "Berhasil membersihkan {} file, menghemat {}.",
                        report.successful,
                        format_bytes(report.bytes_freed)
                    ))
                    .size(16.0)
                    .color(Color32::WHITE),
                );

                if report.failed > 0 {
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(format!(
                            "Gagal diproses: {} file (mungkin terkunci oleh aplikasi lain).",
                            report.failed
                        ))
                        .color(COLOR_DELETE_TEXT),
                    );
                }

                ui.add_space(16.0);
                ui.label(
                    RichText::new(format!("Log sesi tersimpan di: {}", report.log_path.display()))
                        .color(COLOR_MUTED_TEXT)
                        .size(12.0),
                );

                ui.add_space(12.0);
                if ui
                    .add_sized(
                        [220.0, 36.0],
                        egui::Button::new(
                            RichText::new("📄 Buka File Log")
                                .color(Color32::WHITE)
                                .strong(),
                        ),
                    )
                    .clicked()
                {
                    Self::open_file(&report.log_path);
                }
            }

            ui.add_space(30.0);
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 360.0) / 2.0);

                if ui
                    .add_sized(
                        [170.0, 40.0],
                        egui::Button::new("Lihat Sisa Duplikat"),
                    )
                    .clicked()
                {
                    self.screen = AppScreen::Results;
                }

                if ui
                    .add_sized(
                        [170.0, 40.0],
                        egui::Button::new(
                            RichText::new("🔄 Scan Folder Baru")
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(COLOR_ACCENT_PRIMARY),
                    )
                    .clicked()
                {
                    self.screen = AppScreen::Setup;
                    self.groups.clear();
                    self.expanded_groups.clear();
                    self.roots.clear();
                    self.thumb_cache.clear();
                }
            });
        });
    }
}
