use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

use crate::history::{HistoryActionKind, HistoryEntry, HistoryStore};
use crate::ui::components::{Badge, EmptyState};
use crate::ui::icons::{render_icon, IconKind};
use crate::ui::theme::{
    format_bytes, COLOR_BORDER, COLOR_CARD_BG, COLOR_DELETE_TEXT, COLOR_KEEP_BG, COLOR_KEEP_TEXT,
    COLOR_MUTED_TEXT, COLOR_TEXT_PRIMARY, RADIUS_MD, RADIUS_SM, SPACE_SM, SPACE_XS,
};

#[derive(Default)]
pub struct HistoryUiState {
    pub open: bool,
    pub entries: Vec<HistoryEntry>,
    status_msg: Option<(String, bool)>,
}

impl HistoryUiState {
    pub fn open_panel(&mut self) {
        self.entries = HistoryStore::load();
        self.open = true;
        self.status_msg = None;
    }
}

pub struct HistoryView;

impl HistoryView {
    pub fn render(state: &mut HistoryUiState, ctx: &egui::Context) {
        if !state.open {
            return;
        }

        let mut still_open = true;
        let mut undo_id: Option<u64> = None;

        egui::Window::new("Riwayat Pembersihan")
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .default_size([620.0, 480.0])
            .open(&mut still_open)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new("Sesi pembersihan tersimpan di perangkat ini. Aksi Recycle Bin dapat dikembalikan (Undo).")
                        .color(COLOR_MUTED_TEXT)
                        .size(12.0),
                );
                ui.add_space(SPACE_SM);

                if let Some((msg, is_err)) = &state.status_msg {
                    let (bg, fg) = if *is_err {
                        (Color32::from_rgb(69, 26, 26), COLOR_DELETE_TEXT)
                    } else {
                        (COLOR_KEEP_BG, COLOR_KEEP_TEXT)
                    };
                    Badge::show(ui, msg, bg, fg);
                    ui.add_space(SPACE_SM);
                }

                if state.entries.is_empty() {
                    EmptyState::show(
                        ui,
                        IconKind::History,
                        "Belum ada riwayat",
                        "Sesi pembersihan akan muncul di sini setelah Anda menghapus file.",
                    );
                    return;
                }

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for entry in &state.entries {
                        egui::Frame::none()
                            .fill(COLOR_CARD_BG)
                            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                            .rounding(Rounding::same(RADIUS_MD))
                            .inner_margin(12.0)
                            .show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    render_icon(
                                        ui,
                                        match entry.action {
                                            HistoryActionKind::RecycleBin => IconKind::Trash,
                                            HistoryActionKind::Quarantine => IconKind::Archive,
                                            HistoryActionKind::PermanentDelete => IconKind::AlertTriangle,
                                        },
                                        16.0,
                                        COLOR_MUTED_TEXT,
                                    );
                                    ui.add_space(SPACE_XS);

                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(&entry.source)
                                                .color(COLOR_TEXT_PRIMARY)
                                                .strong()
                                                .size(13.0),
                                        );
                                        ui.label(
                                            RichText::new(format!(
                                                "{} • {} • {} file • {}{}",
                                                entry.timestamp_label,
                                                entry.action.label(),
                                                entry.item_count,
                                                format_bytes(entry.total_bytes),
                                                if entry.restored { " • Sudah dikembalikan" } else { "" }
                                            ))
                                            .color(COLOR_MUTED_TEXT)
                                            .size(11.0),
                                        );
                                    });

                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        if entry.is_restorable() {
                                            if ui
                                                .add(
                                                    egui::Button::new(
                                                        RichText::new("Undo").size(11.5).color(COLOR_TEXT_PRIMARY),
                                                    )
                                                    .fill(COLOR_CARD_BG)
                                                    .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                                                    .rounding(Rounding::same(RADIUS_SM)),
                                                )
                                                .on_hover_text("Kembalikan file ini dari Recycle Bin ke lokasi semula")
                                                .clicked()
                                            {
                                                undo_id = Some(entry.id);
                                            }
                                        } else if entry.action == HistoryActionKind::RecycleBin && !entry.restored {
                                            ui.label(
                                                RichText::new("Tidak dapat di-undo")
                                                    .color(COLOR_MUTED_TEXT)
                                                    .italics()
                                                    .size(10.5),
                                            );
                                        }
                                    });
                                });
                            });
                        ui.add_space(SPACE_XS);
                    }
                });

                ui.add_space(SPACE_SM);
                ui.horizontal(|ui| {
                    if ui
                        .add(
                            egui::Button::new(RichText::new("Hapus Semua Riwayat").size(11.5).color(COLOR_MUTED_TEXT))
                                .fill(COLOR_CARD_BG)
                                .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
                                .rounding(Rounding::same(RADIUS_SM)),
                        )
                        .clicked()
                    {
                        HistoryStore::clear_all();
                        state.entries.clear();
                        state.status_msg = None;
                    }
                });
            });

        if let Some(id) = undo_id {
            if let Some(entry) = state.entries.iter().find(|e| e.id == id) {
                match crate::history::restore_refs(&entry.trash_refs) {
                    Ok(_) => {
                        HistoryStore::mark_restored(id);
                        if let Some(e) = state.entries.iter_mut().find(|e| e.id == id) {
                            e.restored = true;
                        }
                        state.status_msg = Some(("File berhasil dikembalikan ke lokasi semula.".to_string(), false));
                    }
                    Err(err) => {
                        state.status_msg = Some((format!("Gagal mengembalikan: {}", err), true));
                    }
                }
            }
        }

        if !still_open {
            state.open = false;
        }
    }
}
