use eframe::egui::{self, Color32, RichText};
use crate::ui::icons::{render_icon_circle, IconKind};
use crate::ui::theme::{COLOR_BRAND_ACCENT, COLOR_CARD_BG, COLOR_MUTED_TEXT, SPACE_MD, SPACE_SM, SPACE_XS};

pub struct EmptyState;

impl EmptyState {
    pub fn show(ui: &mut egui::Ui, icon: IconKind, title: &str, subtitle: &str) {
        ui.vertical_centered(|ui| {
            ui.add_space(SPACE_MD * 2.0);
            render_icon_circle(
                ui,
                icon,
                64.0,
                32.0,
                COLOR_CARD_BG,
                COLOR_BRAND_ACCENT,
            );
            ui.add_space(SPACE_SM);
            ui.heading(
                RichText::new(title)
                    .size(18.0)
                    .color(Color32::WHITE)
                    .strong(),
            );
            ui.add_space(SPACE_XS);
            ui.label(RichText::new(subtitle).color(COLOR_MUTED_TEXT).size(13.0));
            ui.add_space(SPACE_MD);
        });
    }
}
