use eframe::egui::{self, Color32, RichText};
use std::time::Instant;

use crate::ui::theme::{
    COLOR_BG_DARK, COLOR_BRAND_ACCENT, COLOR_BRAND_HOVER, COLOR_MUTED_TEXT, SPACE_MD, SPACE_SM,
};

pub struct SplashScreen {
    start_time: Instant,
    duration_ms: u128,
}

impl SplashScreen {
    pub fn new(duration_ms: u128) -> Self {
        Self {
            start_time: Instant::now(),
            duration_ms,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.start_time.elapsed().as_millis() >= self.duration_ms
    }

    pub fn render(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ctx.request_repaint();

        let elapsed = self.start_time.elapsed().as_millis();
        let progress = (elapsed as f32 / self.duration_ms as f32).clamp(0.0, 1.0);

        // Gentle pulse effect
        let pulse = (elapsed as f32 / 120.0).sin().abs();
        let icon_color = if pulse > 0.5 {
            COLOR_BRAND_HOVER
        } else {
            COLOR_BRAND_ACCENT
        };

        let screen_rect = ui.max_rect();
        ui.painter().rect_filled(screen_rect, 0.0, COLOR_BG_DARK);

        ui.allocate_new_ui(egui::UiBuilder::new().max_rect(screen_rect), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(screen_rect.height() * 0.28);

                // Lightning Vector Icon
                crate::ui::icons::render_icon(ui, crate::ui::icons::IconKind::Lightning, 64.0, icon_color);

                ui.add_space(SPACE_SM);

                // Title
                ui.heading(
                    RichText::new("DupeSweeper")
                        .size(32.0)
                        .color(Color32::WHITE)
                        .strong(),
                );

                ui.add_space(4.0);

                // Version & Subtitle
                ui.label(
                    RichText::new("v6.0.0 • 100% Offline Cross-Platform Storage Cleaner")
                        .size(13.0)
                        .color(COLOR_MUTED_TEXT),
                );

                ui.add_space(SPACE_MD * 1.5);

                // Thin elegant loading bar
                let pb = egui::ProgressBar::new(progress)
                    .desired_width(260.0)
                    .desired_height(4.0)
                    .fill(COLOR_BRAND_ACCENT)
                    .show_percentage();
                ui.add(pb);

                ui.add_space(SPACE_SM);
                ui.label(
                    RichText::new("Memuat database fingerprint...")
                        .size(11.0)
                        .color(COLOR_MUTED_TEXT),
                );
            });
        });
    }
}
