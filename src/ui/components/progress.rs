use eframe::egui::{self, Color32, RichText};
use crate::ui::theme::{COLOR_ACCENT_HOVER, COLOR_MUTED_TEXT, SPACE_SM};

pub struct ModernProgressBar;

impl ModernProgressBar {
    pub fn show(ui: &mut egui::Ui, ratio: f32, title: &str, detail: &str) {
        ui.vertical_centered(|ui| {
            ui.heading(
                RichText::new(title)
                    .size(19.0)
                    .color(Color32::WHITE)
                    .strong(),
            );
            ui.add_space(SPACE_SM);

            let pb = egui::ProgressBar::new(ratio.clamp(0.0, 1.0))
                .desired_width(460.0)
                .desired_height(14.0)
                .show_percentage();
            ui.add(pb);

            ui.add_space(SPACE_SM);
            ui.label(
                RichText::new(detail)
                    .color(COLOR_ACCENT_HOVER)
                    .size(12.5)
                    .monospace(),
            );
            ui.add_space(4.0);
            ui.label(
                RichText::new("Aplikasi tetap responsif. Anda dapat membatalkan kapan saja.")
                    .color(COLOR_MUTED_TEXT)
                    .size(11.0),
            );
        });
    }
}

