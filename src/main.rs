#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dupesweeper::app::DupeSweeperApp;

fn load_icon() -> Option<eframe::egui::IconData> {
    let bytes = include_bytes!("../assets/icon.png");
    image::load_from_memory(bytes).ok().map(|img| {
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        eframe::egui::IconData {
            rgba: rgba.into_raw(),
            width,
            height,
        }
    })
}

fn main() -> eframe::Result<()> {
    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([1040.0, 720.0])
        .with_min_inner_size([800.0, 560.0])
        .with_title("DupeSweeper - Duplicate File Finder & Cleaner")
        .with_drag_and_drop(true);

    if let Some(icon) = load_icon() {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "DupeSweeper",
        native_options,
        Box::new(|cc| Ok(Box::new(DupeSweeperApp::new(cc)))),
    )
}

