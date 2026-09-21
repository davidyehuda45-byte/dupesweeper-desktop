#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dupesweeper::app::DupeSweeperApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1040.0, 720.0])
            .with_min_inner_size([800.0, 560.0])
            .with_title("DupeSweeper - Duplicate File Finder & Cleaner")
            .with_drag_and_drop(true),
        ..Default::default()
    };

    eframe::run_native(
        "DupeSweeper",
        native_options,
        Box::new(|cc| Ok(Box::new(DupeSweeperApp::new(cc)))),
    )
}

