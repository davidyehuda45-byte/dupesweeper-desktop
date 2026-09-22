use crossbeam_channel::{unbounded, Receiver, Sender};
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use crate::actions::{ActionKind, ActionReport, DeleteProgressEvent, DeleteWorker};
use crate::history::{HistoryActionKind, HistoryEntry, HistoryStore};
use crate::scanner::{
    DuplicateGroup, ScanProgress, Scanner, SelectionStrategy, TemplateCategory, WalkerConfig,
};
use crate::settings::{AppSettings, AutoScanInterval};
use crate::ui::analyzer_view::{AnalyzerState, AnalyzerView};
use crate::ui::cleanup_view::{CleanupState, CleanupView};
use crate::ui::components::{Badge, EmptyState, ModernProgressBar};
use crate::ui::history_view::{HistoryUiState, HistoryView};
use crate::ui::icons::{paint_icon, render_icon, render_icon_circle, IconKind};
use crate::ui::splash_screen::SplashScreen;
use crate::ui::theme::{
    file_extension_category, format_bytes, format_system_time, paint_dashed_rect,
    setup_custom_theme, COLOR_ACCENT_HOVER, COLOR_ACCENT_PRIMARY, COLOR_BG_DARK, COLOR_BORDER,
    COLOR_BORDER_SUBTLE, COLOR_BRAND_ACCENT, COLOR_BRAND_HOVER, COLOR_CARD_BG, COLOR_CARD_HOVER,
    COLOR_DELETE_BG, COLOR_DELETE_TEXT, COLOR_DISABLED_BG,
    COLOR_DISABLED_BORDER, COLOR_DISABLED_TEXT, COLOR_KEEP_BG, COLOR_KEEP_TEXT, COLOR_MUTED_TEXT,
    COLOR_PANEL_BG, COLOR_SENSITIVE_BG, COLOR_SENSITIVE_TEXT, COLOR_TEXT_PRIMARY, RADIUS_LG,
    RADIUS_MD, RADIUS_SM, SPACE_LG, SPACE_MD, SPACE_SM, SPACE_XL, SPACE_XS,
};
use crate::ui::thumbnail::ThumbnailCache;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AppMode {
    DuplicateFinder,
    GeneralCleanup,
    SizeAnalyzer,
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

    // Folder size analyzer mode state
    analyzer_state: AnalyzerState,

    // History & Undo overlay
    history_ui: HistoryUiState,

    // Persisted settings & auto-scan scheduler
    settings: AppSettings,
    show_settings_popup: bool,
    last_scan_finished_at: Option<Instant>,

    // Tracks what was targeted in the current/last delete op, so a history
    // entry (and Recycle Bin trash refs for Undo) can be recorded afterwards.
    pending_delete_paths: Vec<PathBuf>,
    last_action_kind: Option<ActionKind>,

    // Transient toast notification: (message, is_error, shown_at)
    toast: Option<(String, bool, Instant)>,
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

            analyzer_state: AnalyzerState::default(),
            history_ui: HistoryUiState::default(),
            settings: AppSettings::default(),
            show_settings_popup: false,
            last_scan_finished_at: None,
            pending_delete_paths: Vec::new(),
            last_action_kind: None,
            toast: None,
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
        let mut app = Self::default();
        app.settings = AppSettings::load();
        if app.roots.is_empty() {
            app.roots = app
                .settings
                .last_scan_roots
                .iter()
                .filter(|p| p.exists())
                .cloned()
                .collect();
        }
        if app.settings.auto_scan_interval.as_secs().is_some() {
            app.last_scan_finished_at = Some(Instant::now());
        }
        app
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
        crate::platform::open_in_file_manager(path);
    }

    fn open_file(path: &Path) {
        crate::platform::open_file(path);
    }

    /// Small square icon-only button used in the top toolbar.
    fn icon_toolbar_button(ui: &mut egui::Ui, icon: IconKind, tooltip: &str) -> bool {
        let size = 30.0;
        let (rect, resp) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::click());
        let bg = if resp.hovered() { COLOR_CARD_HOVER } else { COLOR_CARD_BG };
        ui.painter().rect_filled(rect, Rounding::same(RADIUS_SM), bg);
        ui.painter()
            .rect_stroke(rect, Rounding::same(RADIUS_SM), Stroke::new(1.0_f32, COLOR_BORDER));
        let icon_rect = egui::Rect::from_center_size(rect.center(), Vec2::splat(15.0));
        paint_icon(ui.painter(), icon_rect, icon, COLOR_TEXT_PRIMARY);
        resp.on_hover_text(tooltip).clicked()
    }

    fn render_settings_popup(&mut self, ctx: &egui::Context) {
        let mut still_open = true;
        egui::Window::new("Pengaturan")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .fixed_size([380.0, 300.0])
            .open(&mut still_open)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new("Auto-Scan Terjadwal")
                        .strong()
                        .size(14.0)
                        .color(COLOR_TEXT_PRIMARY),
                );
                ui.label(
                    RichText::new(
                        "Otomatis memindai ulang folder terakhir yang di-scan, selama aplikasi ini tetap terbuka.",
                    )
                    .color(COLOR_MUTED_TEXT)
                    .size(11.5),
                );
                ui.add_space(SPACE_SM);

                let mut changed = false;
                for interval in AutoScanInterval::ALL {
                    if ui
                        .radio_value(&mut self.settings.auto_scan_interval, interval, interval.label())
                        .changed()
                    {
                        changed = true;
                    }
                }
                if changed {
                    self.settings.last_scan_roots = self.roots.clone();
                    self.settings.save();
                    self.last_scan_finished_at = Some(Instant::now());
                }

                ui.add_space(SPACE_MD);
                if self.roots.is_empty() {
                    ui.label(
                        RichText::new(
                            "Pilih minimal satu folder di tab \"Cari File Duplikat\" agar auto-scan dapat berjalan.",
                        )
                        .color(COLOR_SENSITIVE_TEXT)
                        .size(11.0),
                    );
                }
            });
        self.show_settings_popup = still_open;
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

        self.pending_delete_paths = selected_files.iter().map(|f| f.path.clone()).collect();
        self.last_action_kind = Some(kind.clone());

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
        self.record_duplicate_history(&report);

        // Remove deleted files from groups (files that no longer exist on disk)
        for group in &mut self.groups {
            group.files.retain(|f| f.path.exists());
        }
        // Remove groups that now have < 2 files
        self.groups.retain(|g| g.files.len() >= 2);

        self.last_report = Some(report);
        self.screen = AppScreen::Completed;
    }

    /// Records a history entry for the just-finished delete op. For Recycle
    /// Bin actions, also captures trash references so it can later be undone.
    fn record_duplicate_history(&mut self, report: &ActionReport) {
        let action_kind = self.last_action_kind.take();
        let targeted_paths = std::mem::take(&mut self.pending_delete_paths);

        if report.successful == 0 {
            return;
        }

        let history_kind = match action_kind {
            Some(ActionKind::RecycleBin) | None => HistoryActionKind::RecycleBin,
            Some(ActionKind::Quarantine(_)) => HistoryActionKind::Quarantine,
            Some(ActionKind::PermanentDelete) => HistoryActionKind::PermanentDelete,
        };

        let trash_refs = if history_kind == HistoryActionKind::RecycleBin {
            let failed: HashSet<&PathBuf> = report.error_details.iter().map(|(p, _)| p).collect();
            let successful_paths: Vec<PathBuf> = targeted_paths
                .into_iter()
                .filter(|p| !failed.contains(p))
                .collect();
            crate::history::capture_trash_refs(&successful_paths)
        } else {
            Vec::new()
        };

        let entry = HistoryEntry::new(
            "Pencari File Duplikat",
            history_kind,
            report.successful,
            report.bytes_freed,
            trash_refs,
        );
        HistoryStore::add_entry(entry);
    }

    fn export_duplicates(&mut self, as_json: bool) {
        let default_name = if as_json {
            "dupesweeper_duplicates.json"
        } else {
            "dupesweeper_duplicates.csv"
        };
        let Some(path) = rfd::FileDialog::new().set_file_name(default_name).save_file() else {
            return;
        };
        let result = if as_json {
            crate::export::export_duplicates_json(&self.groups, &path)
        } else {
            crate::export::export_duplicates_csv(&self.groups, &path)
        };
        self.toast = Some(match result {
            Ok(_) => (
                format!("Laporan berhasil diekspor ke {}", path.display()),
                false,
                Instant::now(),
            ),
            Err(e) => (format!("Gagal mengekspor: {}", e), true, Instant::now()),
        });
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
            self.last_scan_finished_at = Some(Instant::now());
            self.settings.last_scan_roots = self.roots.clone();
            self.settings.save();
        } else if reset_to_setup {
            self.scan_rx = None;
            self.screen = AppScreen::Setup;
        }

        // In-app auto-scan scheduler: while idle on the Duplicate Finder tab,
        // periodically re-scan the last-used folders per the configured interval.
        if self.mode == AppMode::DuplicateFinder
            && matches!(self.screen, AppScreen::Setup | AppScreen::Results)
            && self.scan_rx.is_none()
            && !self.roots.is_empty()
        {
            if let Some(interval_secs) = self.settings.auto_scan_interval.as_secs() {
                let due = self
                    .last_scan_finished_at
                    .map(|t| t.elapsed().as_secs() >= interval_secs)
                    .unwrap_or(false);
                if due {
                    self.start_scan();
                    self.toast = Some((
                        "Auto-scan terjadwal dijalankan.".to_string(),
                        false,
                        Instant::now(),
                    ));
                }
            }
        }

        // Record history for a just-finished General Cleanup session
        if let Some(report) = self.cleanup_state.just_completed.take() {
            if report.successful_deleted > 0 {
                HistoryStore::add_entry(HistoryEntry::new(
                    "Pembersih Sampah Umum",
                    HistoryActionKind::PermanentDelete,
                    report.successful_deleted,
                    report.bytes_freed,
                    Vec::new(),
                ));
            }
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
            .frame(
                egui::Frame::none()
                    .fill(COLOR_PANEL_BG)
                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                    .inner_margin(egui::Margin::symmetric(24.0, 12.0)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Logo + Brand Name
                    render_icon(ui, IconKind::Lightning, 20.0, COLOR_BRAND_ACCENT);
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new("DupeSweeper")
                            .color(COLOR_TEXT_PRIMARY)
                            .size(17.0)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    Badge::show(ui, "v11.0.0", COLOR_CARD_BG, COLOR_MUTED_TEXT);
                    Badge::show(ui, "Offline", Color32::from_rgb(20, 36, 28), Color32::from_rgb(110, 231, 183));

                    ui.add_space(SPACE_LG);

                    // Sleek Segmented Switcher
                    egui::Frame::none()
                        .fill(COLOR_BG_DARK)
                        .stroke(Stroke::new(1.0_f32, COLOR_BORDER_SUBTLE))
                        .rounding(Rounding::same(RADIUS_MD))
                        .inner_margin(egui::Margin::symmetric(3.0, 3.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                let is_dup = self.mode == AppMode::DuplicateFinder;
                                let dup_btn = ui.add(
                                    egui::Button::new(
                                        RichText::new("Cari File Duplikat")
                                            .color(if is_dup {
                                                COLOR_TEXT_PRIMARY
                                            } else {
                                                COLOR_MUTED_TEXT
                                            })
                                            .strong()
                                            .size(12.5),
                                    )
                                    .fill(if is_dup {
                                        COLOR_CARD_BG
                                    } else {
                                        Color32::TRANSPARENT
                                    })
                                    .stroke(if is_dup {
                                        Stroke::new(1.0_f32, COLOR_BORDER)
                                    } else {
                                        Stroke::NONE
                                    })
                                    .rounding(Rounding::same(RADIUS_SM)),
                                );
                                if dup_btn.clicked() {
                                    self.mode = AppMode::DuplicateFinder;
                                }

                                let is_clean = self.mode == AppMode::GeneralCleanup;
                                let clean_btn = ui.add(
                                    egui::Button::new(
                                        RichText::new("Bersihkan Sampah")
                                            .color(if is_clean {
                                                COLOR_TEXT_PRIMARY
                                            } else {
                                                COLOR_MUTED_TEXT
                                            })
                                            .strong()
                                            .size(12.5),
                                    )
                                    .fill(if is_clean {
                                        COLOR_CARD_BG
                                    } else {
                                        Color32::TRANSPARENT
                                    })
                                    .stroke(if is_clean {
                                        Stroke::new(1.0_f32, COLOR_BORDER)
                                    } else {
                                        Stroke::NONE
                                    })
                                    .rounding(Rounding::same(RADIUS_SM)),
                                );
                                if clean_btn.clicked() {
                                    self.mode = AppMode::GeneralCleanup;
                                }

                                let is_analyzer = self.mode == AppMode::SizeAnalyzer;
                                let analyzer_btn = ui.add(
                                    egui::Button::new(
                                        RichText::new("Analisis Disk")
                                            .color(if is_analyzer {
                                                COLOR_TEXT_PRIMARY
                                            } else {
                                                COLOR_MUTED_TEXT
                                            })
                                            .strong()
                                            .size(12.5),
                                    )
                                    .fill(if is_analyzer {
                                        COLOR_CARD_BG
                                    } else {
                                        Color32::TRANSPARENT
                                    })
                                    .stroke(if is_analyzer {
                                        Stroke::new(1.0_f32, COLOR_BORDER)
                                    } else {
                                        Stroke::NONE
                                    })
                                    .rounding(Rounding::same(RADIUS_SM)),
                                );
                                if analyzer_btn.clicked() {
                                    self.mode = AppMode::SizeAnalyzer;
                                }
                            });
                        });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if Self::icon_toolbar_button(ui, IconKind::Settings, "Pengaturan") {
                            self.show_settings_popup = true;
                        }
                        ui.add_space(SPACE_XS);
                        if Self::icon_toolbar_button(ui, IconKind::History, "Riwayat & Undo") {
                            self.history_ui.open_panel();
                        }
                        ui.add_space(SPACE_SM);

                        if self.mode == AppMode::DuplicateFinder && self.screen == AppScreen::Results {
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("Scan Baru")
                                            .color(COLOR_TEXT_PRIMARY)
                                            .strong()
                                            .size(12.0),
                                    )
                                    .fill(COLOR_CARD_BG)
                                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                                    .rounding(Rounding::same(RADIUS_SM)),
                                )
                                .clicked()
                            {
                                self.screen = AppScreen::Setup;
                                self.groups.clear();
                                self.expanded_groups.clear();
                                self.thumb_cache.clear();
                            }
                            ui.add_space(SPACE_SM);
                        }

                        if let Some((msg, is_err, when)) = &self.toast {
                            if when.elapsed().as_secs() < 5 {
                                let (bg, fg) = if *is_err {
                                    (Color32::from_rgb(69, 26, 26), COLOR_DELETE_TEXT)
                                } else {
                                    (Color32::from_rgb(20, 36, 28), Color32::from_rgb(110, 231, 183))
                                };
                                Badge::show(ui, msg, bg, fg);
                                ctx.request_repaint_after(std::time::Duration::from_millis(500));
                            }
                        }
                    });
                });
            });

        // Main Central View with Level 0 deepest dark background
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(COLOR_BG_DARK).inner_margin(24.0))
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
                AppMode::SizeAnalyzer => {
                    AnalyzerView::render(&mut self.analyzer_state, ctx, ui);
                }
            });

        // Modal Confirmation Dialog
        if self.show_confirm_dialog {
            self.render_confirmation_modal(ctx);
        }

        HistoryView::render(&mut self.history_ui, ctx);
        if self.show_settings_popup {
            self.render_settings_popup(ctx);
        }
    }
}

// ----------------------------------------------------------------------------
// Screen Implementations
// ----------------------------------------------------------------------------
impl DupeSweeperApp {
    fn render_setup_view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(SPACE_SM);
            ui.heading(
                RichText::new("Cari & Bersihkan File Duplikat")
                    .size(26.0)
                    .color(COLOR_TEXT_PRIMARY)
                    .strong(),
            );
            ui.add_space(SPACE_XS);
            ui.label(
                RichText::new(
                    "Pindai drive atau folder untuk mendeteksi file duplikat berdasarkan hash BLAKE3 secara akurat dan aman.",
                )
                .color(COLOR_MUTED_TEXT)
                .size(13.0),
            );
            ui.add_space(SPACE_MD);
        });

        // --------------------------------------------------------------------
        // 1. Drop Zone Card (Interactive Dashed Border + Drag-Over Feedback)
        // --------------------------------------------------------------------
        let is_window_drag = ctx.input(|i| !i.raw.hovered_files.is_empty());
        let drop_height = 145.0;

        let (drop_rect, drop_response) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), drop_height),
            egui::Sense::hover(),
        );

        let is_drop_active = drop_response.hovered() || is_window_drag;

        // Background fill with slight amber glow on hover/drag
        let bg_fill = if is_drop_active {
            COLOR_CARD_HOVER
        } else {
            COLOR_PANEL_BG
        };
        ui.painter().rect_filled(drop_rect, Rounding::same(RADIUS_LG), bg_fill);

        if is_drop_active {
            ui.painter().rect_filled(
                drop_rect,
                Rounding::same(RADIUS_LG),
                Color32::from_rgba_unmultiplied(245, 158, 11, 24),
            );
        }

        // Dashed border in brand accent color
        let border_stroke = if is_drop_active {
            Stroke::new(2.0_f32, COLOR_BRAND_HOVER)
        } else {
            Stroke::new(1.5_f32, COLOR_BRAND_ACCENT)
        };
        paint_dashed_rect(ui.painter(), drop_rect, border_stroke, 10.0, 6.0);

        // Content inside Drop Zone
        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(drop_rect), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(SPACE_MD);
                render_icon(ui, IconKind::FolderOpen, 36.0, COLOR_BRAND_ACCENT);
                ui.add_space(SPACE_XS);
                ui.label(
                    RichText::new(if is_drop_active {
                        "Lepaskan folder di sini untuk menambahkan ke antrean scan!"
                    } else {
                        "Tarik & Lepas (Drag & Drop) folder ke sini"
                    })
                    .size(15.0)
                    .color(if is_drop_active {
                        COLOR_BRAND_HOVER
                    } else {
                        COLOR_TEXT_PRIMARY
                    })
                    .strong(),
                );
                ui.label(
                    RichText::new("atau gunakan tombol di bawah untuk memilih dari File Explorer")
                        .color(COLOR_MUTED_TEXT)
                        .size(12.0),
                );
                ui.add_space(SPACE_SM);

                let pick_btn = ui.add_sized(
                    [220.0, 34.0],
                    egui::Button::new(
                        RichText::new("Pilih Folder / Drive...")
                            .color(Color32::WHITE)
                            .strong()
                            .size(13.0),
                    )
                    .fill(COLOR_ACCENT_PRIMARY)
                    .rounding(Rounding::same(RADIUS_MD)),
                );

                if pick_btn.clicked() {
                    if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                        if !self.roots.contains(&folder) {
                            self.roots.push(folder);
                        }
                    }
                }
            });
        });

        ui.add_space(SPACE_MD);

        // --------------------------------------------------------------------
        // 2. Folder List Card
        // --------------------------------------------------------------------
        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
            .rounding(Rounding::same(RADIUS_MD))
            .inner_margin(egui::Margin::symmetric(16.0, 14.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("Folder yang Akan Di-scan ({})", self.roots.len()))
                            .strong()
                            .size(14.0)
                            .color(COLOR_TEXT_PRIMARY),
                    );

                    if !self.roots.is_empty() {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("Kosongkan Semua")
                                            .size(11.0)
                                            .color(COLOR_DELETE_TEXT),
                                    )
                                    .fill(COLOR_DELETE_BG)
                                    .rounding(Rounding::same(RADIUS_SM)),
                                )
                                .clicked()
                            {
                                self.roots.clear();
                            }
                        });
                    }
                });

                ui.add_space(SPACE_SM);

                if self.roots.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(SPACE_SM);
                        render_icon(ui, IconKind::Folder, 28.0, COLOR_MUTED_TEXT);
                        ui.add_space(SPACE_XS);
                        ui.label(
                            RichText::new("Belum ada folder yang dipilih.")
                                .color(COLOR_MUTED_TEXT)
                                .size(13.0),
                        );
                        ui.label(
                            RichText::new("Tarik folder ke area di atas atau klik tombol Pilih Folder untuk memulai.")
                                .color(COLOR_MUTED_TEXT)
                                .size(11.0),
                        );
                        ui.add_space(SPACE_SM);
                    });
                } else {
                    egui::ScrollArea::vertical()
                        .max_height(130.0)
                        .show(ui, |ui| {
                            ui.spacing_mut().item_spacing = Vec2::new(SPACE_SM, SPACE_XS);
                            let mut remove_idx = None;
                            for (i, root) in self.roots.iter().enumerate() {
                                egui::Frame::none()
                                    .fill(COLOR_CARD_BG)
                                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER_SUBTLE))
                                    .rounding(Rounding::same(RADIUS_SM))
                                    .inner_margin(egui::Margin::symmetric(10.0, 6.0))
                                    .show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new(format!("{}.", i + 1))
                                                    .size(11.0)
                                                    .color(COLOR_MUTED_TEXT),
                                            );
                                            render_icon(ui, IconKind::Folder, 14.0, COLOR_BRAND_ACCENT);
                                            ui.label(
                                                RichText::new(root.display().to_string())
                                                    .strong()
                                                    .size(12.0)
                                                    .color(COLOR_TEXT_PRIMARY),
                                            );
                                            ui.with_layout(
                                                egui::Layout::right_to_left(egui::Align::Center),
                                                |ui| {
                                                    if ui
                                                        .add(
                                                            egui::Button::new(
                                                                RichText::new("Hapus")
                                                                    .size(11.0)
                                                                    .color(COLOR_MUTED_TEXT),
                                                            )
                                                            .fill(Color32::TRANSPARENT)
                                                            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                                                            .rounding(Rounding::same(RADIUS_SM)),
                                                        )
                                                        .clicked()
                                                    {
                                                        remove_idx = Some(i);
                                                    }
                                                },
                                            );
                                        });
                                    });
                            }
                            if let Some(idx) = remove_idx {
                                self.roots.remove(idx);
                            }
                        });
                }
            });

        ui.add_space(SPACE_MD);

        // --------------------------------------------------------------------
        // 3. Scan Options & Advanced Settings Card
        // --------------------------------------------------------------------
        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
            .rounding(Rounding::same(RADIUS_MD))
            .inner_margin(egui::Margin::symmetric(16.0, 14.0))
            .show(ui, |ui| {
                ui.checkbox(
                    &mut self.scan_all_folders,
                    RichText::new("Scan semua folder (termasuk dependency node_modules/vendor/target)")
                        .color(COLOR_TEXT_PRIMARY)
                        .strong()
                        .size(13.0),
                );
                ui.label(
                    RichText::new("Secara default, folder dependency & build otomatis dilewati untuk performa optimal dan perlindungan file project.")
                        .color(COLOR_MUTED_TEXT)
                        .size(11.0),
                );

                ui.add_space(SPACE_SM);

                ui.collapsing(
                    RichText::new("Pengaturan Scan Tambahan (Opsional)")
                        .strong()
                        .color(COLOR_MUTED_TEXT)
                        .size(12.0),
                    |ui| {
                        ui.add_space(SPACE_XS);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Ukuran File Minimum:").size(12.0));
                            ui.add(
                                egui::DragValue::new(&mut self.min_size_kb)
                                    .range(0..=1_000_000)
                                    .suffix(" KB"),
                            );
                            ui.label(
                                RichText::new("(File lebih kecil dari ini akan dilewati)")
                                    .color(COLOR_MUTED_TEXT)
                                    .size(11.0),
                            );
                        });

                        ui.add_space(SPACE_XS);
                        ui.checkbox(
                            &mut self.include_hidden,
                            RichText::new("Scan file/folder tersembunyi (hidden)").size(12.0),
                        );

                        ui.add_space(SPACE_XS);
                        ui.horizontal(|ui| {
                            ui.label(RichText::new("Abaikan Ekstensi:").size(12.0));
                            ui.add(
                                egui::TextEdit::singleline(&mut self.exclude_exts_input)
                                    .desired_width(180.0)
                                    .hint_text("tmp, bak, log"),
                            );
                            ui.label(
                                RichText::new("(Pisahkan dengan tanda koma)")
                                    .color(COLOR_MUTED_TEXT)
                                    .size(11.0),
                            );
                        });
                    },
                );
            });

        ui.add_space(SPACE_LG);

        // --------------------------------------------------------------------
        // 4. Primary CTA: Start Duplicate Scan Button
        // --------------------------------------------------------------------
        ui.vertical_centered(|ui| {
            let can_scan = !self.roots.is_empty();
            if can_scan {
                let btn = ui.add_sized(
                    [320.0, 46.0],
                    egui::Button::new(
                        RichText::new("Mulai Scan Duplikat")
                            .size(15.0)
                            .color(Color32::from_rgb(14, 16, 21))
                            .strong(),
                    )
                    .fill(COLOR_BRAND_ACCENT)
                    .rounding(Rounding::same(RADIUS_MD)),
                );

                if btn.clicked() {
                    self.start_scan();
                }
            } else {
                ui.add_sized(
                    [320.0, 46.0],
                    egui::Button::new(
                        RichText::new("Mulai Scan Duplikat")
                            .size(15.0)
                            .color(COLOR_DISABLED_TEXT)
                            .strong(),
                    )
                    .fill(COLOR_DISABLED_BG)
                    .stroke(Stroke::new(1.0_f32, COLOR_DISABLED_BORDER))
                    .rounding(Rounding::same(RADIUS_MD)),
                );
            }
        });
    }

    fn render_scanning_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(SPACE_XL);
            ui.heading(
                RichText::new("Sedang Memindai File Duplikat...")
                    .size(24.0)
                    .color(COLOR_TEXT_PRIMARY)
                    .strong(),
            );
            ui.add_space(SPACE_XS);
            ui.label(
                RichText::new(&self.scan_stage_name)
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

                    ui.add_space(SPACE_MD);
                    ui.label(
                        RichText::new(format!("File saat ini: {}", self.scan_file_detail))
                            .color(COLOR_MUTED_TEXT)
                            .size(12.0),
                    );

                    if let Some(start) = self.scan_start_instant {
                        ui.add_space(SPACE_XS);
                        ui.label(
                            RichText::new(format!(
                                "Waktu berjalan: {:.1} detik",
                                start.elapsed().as_secs_f32()
                            ))
                            .color(COLOR_MUTED_TEXT)
                            .size(12.0),
                        );
                    }
                });

            ui.add_space(SPACE_LG);
            if ui
                .add_sized(
                    [200.0, 38.0],
                    egui::Button::new(
                        RichText::new("Batalkan Scan")
                            .color(Color32::WHITE)
                            .strong(),
                    )
                    .fill(COLOR_DELETE_BG)
                    .rounding(Rounding::same(RADIUS_MD)),
                )
                .clicked()
            {
                self.cancel_scan();
            }
        });
    }

    fn render_deleting_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(SPACE_XL);
            ui.heading(
                RichText::new("Sedang Membersihkan File Duplikat...")
                    .size(24.0)
                    .color(COLOR_TEXT_PRIMARY)
                    .strong(),
            );
            ui.add_space(SPACE_XS);

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
                    .color(if is_cancelling { COLOR_DELETE_TEXT } else { COLOR_BRAND_ACCENT })
                    .size(14.0)
                    .strong(),
            );
            ui.add_space(SPACE_LG);

            // Container Card for Deletion Progress
            egui::Frame::none()
                .fill(COLOR_PANEL_BG)
                .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                .rounding(Rounding::same(RADIUS_LG))
                .inner_margin(egui::Margin::symmetric(24.0, 20.0))
                .show(ui, |ui| {
                    ModernProgressBar::show(
                        ui,
                        progress_ratio,
                        &format!("{:.0}%", progress_ratio * 100.0),
                        &format!("Ruang dibebaskan: {}", format_bytes(self.delete_bytes_freed)),
                    );

                    ui.add_space(SPACE_MD);
                    ui.label(
                        RichText::new(format!("File saat ini: {}", self.delete_current_file))
                            .color(COLOR_MUTED_TEXT)
                            .size(12.0),
                    );
                });

            ui.add_space(SPACE_LG);
            ui.add_enabled_ui(!is_cancelling, |ui| {
                if ui
                    .add_sized(
                        [200.0, 38.0],
                        egui::Button::new(
                            RichText::new(if is_cancelling { "Membatalkan..." } else { "Batalkan" })
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(COLOR_DELETE_BG)
                        .rounding(Rounding::same(RADIUS_MD)),
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
                    IconKind::Sparkles,
                    "Tidak Ada File Duplikat!",
                    "Penyimpanan Anda bersih dan teratur. Tidak ditemukan file duplikat dalam folder yang dipilih.",
                );
                ui.add_space(24.0);
                if ui
                    .add_sized(
                        [200.0, 40.0],
                        egui::Button::new(
                            RichText::new("Scan Folder Lain")
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
                                "Ditemukan {} Grup Duplikat ({} File)",
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
                                    "Info: {} folder dependency/build di-skip otomatis (menghemat waktu scan)",
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
                            "Bersihkan {} File ({})",
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

        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
            .rounding(Rounding::same(RADIUS_MD))
            .inner_margin(egui::Margin::symmetric(14.0, 8.0))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Seleksi:").strong().color(COLOR_TEXT_PRIMARY));

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
                    if ui.small_button("Buka Semua").clicked() {
                        for &idx in &matching_indices {
                            self.expanded_groups.insert(idx);
                        }
                    }

                    if ui.small_button("Tutup Semua").clicked() {
                        self.expanded_groups.clear();
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.filter_query)
                                .hint_text("Cari nama file..."),
                        );
                        ui.add_space(SPACE_SM);
                        if ui.small_button("Export JSON").clicked() {
                            self.export_duplicates(true);
                        }
                        if ui.small_button("Export CSV").clicked() {
                            self.export_duplicates(false);
                        }
                    });
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
                                        let (c_rect, c_resp) = ui.allocate_exact_size(Vec2::splat(16.0), egui::Sense::click());
                                        paint_icon(
                                            ui.painter(),
                                            c_rect,
                                            if is_expanded { IconKind::ChevronDown } else { IconKind::ChevronRight },
                                            COLOR_TEXT_PRIMARY,
                                        );
                                        if c_resp.clicked() {
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
                                            Badge::show(ui, "Sensitif", COLOR_SENSITIVE_BG, COLOR_SENSITIVE_TEXT);
                                        }

                                        if let Some(tm) = group.files.first().and_then(|f| f.template_match.as_ref()) {
                                            let (badge_txt, badge_bg, badge_fg) = match tm.category {
                                                TemplateCategory::Documentation => (
                                                    format!("Template ({})", tm.framework),
                                                    Color32::from_rgb(40, 52, 48),
                                                    Color32::from_rgb(140, 215, 180),
                                                ),
                                                TemplateCategory::Functional => (
                                                    format!("Bawaan ({})", tm.framework),
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
                                            Badge::show(ui, "Sensitif", COLOR_SENSITIVE_BG, COLOR_SENSITIVE_TEXT);
                                        }

                                        // Template Fingerprint Match badge
                                        if let Some(ref tm) = file.template_match {
                                            let (badge_txt, badge_bg, badge_fg) = match tm.category {
                                                TemplateCategory::Documentation => (
                                                    format!("Template ({})", tm.framework),
                                                    Color32::from_rgb(40, 52, 48),
                                                    Color32::from_rgb(140, 215, 180),
                                                ),
                                                TemplateCategory::Functional => (
                                                    format!("Bawaan ({})", tm.framework),
                                                    Color32::from_rgb(32, 45, 65),
                                                    Color32::from_rgb(130, 180, 240),
                                                ),
                                            };
                                            Badge::show(ui, &badge_txt, badge_bg, badge_fg);
                                        }

                                        // Recommendation badge
                                        if file.is_recommended_keep {
                                            Badge::show(ui, "Pertahankan", COLOR_KEEP_BG, COLOR_KEEP_TEXT);
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
                                                if ui.small_button("Preview").clicked() {
                                                    open_file_path = Some(file.path.clone());
                                                }
                                                if ui.small_button("Buka Folder").clicked() {
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
                            .rounding(Rounding::same(RADIUS_MD))
                            .inner_margin(12.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    render_icon(ui, IconKind::AlertTriangle, 16.0, COLOR_SENSITIVE_TEXT);
                                    ui.add_space(4.0);
                                    ui.label(
                                        RichText::new("PERINGATAN: TERDETEKSI FILE SENSITIF!")
                                            .color(COLOR_SENSITIVE_TEXT)
                                            .strong()
                                            .size(13.0),
                                    );
                                });
                                ui.add_space(4.0);
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
                        "Pindahkan ke Recycle Bin (Aman & Disarankan)",
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
                        "Pindahkan ke Folder Karantina",
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
                        "Hapus Permanen (Data Tidak Dapat Dipulihkan)",
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
                                    RichText::new("Jalankan Pembersihan")
                                        .color(Color32::WHITE)
                                        .strong(),
                                )
                                .fill(COLOR_ACCENT_PRIMARY)
                                .rounding(Rounding::same(RADIUS_SM)),
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
            ui.add_space(SPACE_XL);
            render_icon_circle(
                ui,
                IconKind::Check,
                56.0,
                26.0,
                Color32::from_rgba_premultiplied(34, 197, 94, 30),
                Color32::from_rgb(34, 197, 94),
            );
            ui.add_space(SPACE_MD);
            ui.heading(
                RichText::new("Pembersihan Selesai!")
                    .size(24.0)
                    .color(COLOR_TEXT_PRIMARY)
                    .strong(),
            );
            ui.add_space(SPACE_LG);

            if let Some(ref report) = self.last_report {
                egui::Frame::none()
                    .fill(COLOR_PANEL_BG)
                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                    .rounding(Rounding::same(RADIUS_LG))
                    .inner_margin(egui::Margin::symmetric(28.0, 20.0))
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(format!(
                                "Berhasil membersihkan {} file, menghemat {}.",
                                report.successful,
                                format_bytes(report.bytes_freed)
                            ))
                            .size(16.0)
                            .color(COLOR_TEXT_PRIMARY)
                            .strong(),
                        );

                        if report.failed > 0 {
                            ui.add_space(SPACE_XS);
                            ui.label(
                                RichText::new(format!(
                                    "Gagal diproses: {} file (mungkin terkunci oleh aplikasi lain).",
                                    report.failed
                                ))
                                .color(COLOR_DELETE_TEXT)
                                .size(13.0),
                            );
                        }

                        ui.add_space(SPACE_MD);
                        ui.label(
                            RichText::new(format!("Log sesi tersimpan di: {}", report.log_path.display()))
                                .color(COLOR_MUTED_TEXT)
                                .size(12.0),
                        );

                        ui.add_space(SPACE_MD);
                        if ui
                            .add_sized(
                                [200.0, 36.0],
                                egui::Button::new(
                                    RichText::new("Buka File Log")
                                        .color(COLOR_TEXT_PRIMARY)
                                        .strong(),
                                )
                                .fill(COLOR_CARD_BG)
                                .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                                .rounding(Rounding::same(RADIUS_MD)),
                            )
                            .clicked()
                        {
                            Self::open_file(&report.log_path);
                        }
                    });
            }

            ui.add_space(SPACE_XL);
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 556.0) / 2.0);

                if ui
                    .add_sized(
                        [180.0, 42.0],
                        egui::Button::new(
                            RichText::new("Lihat Sisa Duplikat")
                                .color(COLOR_TEXT_PRIMARY)
                                .strong(),
                        )
                        .fill(COLOR_PANEL_BG)
                        .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                        .rounding(Rounding::same(RADIUS_MD)),
                    )
                    .clicked()
                {
                    self.screen = AppScreen::Results;
                }

                if ui
                    .add_sized(
                        [180.0, 42.0],
                        egui::Button::new(
                            RichText::new("Riwayat & Undo")
                                .color(COLOR_TEXT_PRIMARY)
                                .strong(),
                        )
                        .fill(COLOR_PANEL_BG)
                        .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                        .rounding(Rounding::same(RADIUS_MD)),
                    )
                    .clicked()
                {
                    self.history_ui.open_panel();
                }

                if ui
                    .add_sized(
                        [180.0, 42.0],
                        egui::Button::new(
                            RichText::new("Scan Folder Baru")
                                .color(Color32::from_rgb(14, 16, 21))
                                .strong(),
                        )
                        .fill(COLOR_BRAND_ACCENT)
                        .rounding(Rounding::same(RADIUS_MD)),
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
