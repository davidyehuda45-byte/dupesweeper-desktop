use eframe::egui::{self, Color32, RichText, Rounding, Vec2};
use crate::ui::theme::RADIUS_SM;

pub struct Badge;

impl Badge {
    pub fn show(ui: &mut egui::Ui, text: &str, bg: Color32, text_color: Color32) {
        egui::Frame::none()
            .fill(bg)
            .rounding(Rounding::same(RADIUS_SM))
            .inner_margin(Vec2::new(6.0, 2.0))
            .show(ui, |ui| {
                ui.label(
                    RichText::new(text)
                        .size(10.5)
                        .color(text_color)
                        .strong(),
                );
            });
    }
}

