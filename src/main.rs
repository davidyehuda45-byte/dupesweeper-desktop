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

struct SetupWizard {
    create_desktop: bool,
    create_start_menu: bool,
    install_complete: bool,
    error_msg: Option<String>,
}

impl SetupWizard {
    fn new() -> Self {
        Self {
            create_desktop: true,
            create_start_menu: true,
            install_complete: false,
            error_msg: None,
        }
    }
}

impl eframe::App for SetupWizard {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        use eframe::egui::{self, Color32, RichText, Rounding, Stroke};

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(14, 16, 21))
                    .inner_margin(28.0),
            )
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    dupesweeper::ui::icons::render_icon_circle(
                        ui,
                        dupesweeper::ui::icons::IconKind::Lightning,
                        52.0,
                        24.0,
                        Color32::from_rgba_premultiplied(59, 130, 246, 35),
                        Color32::from_rgb(96, 165, 250),
                    );
                    ui.add_space(10.0);
                    ui.heading(
                        RichText::new("Pemasangan DupeSweeper")
                            .color(Color32::from_rgb(243, 244, 246))
                            .size(20.0)
                            .strong(),
                    );
                    ui.label(
                        RichText::new("v6.0.0 — Duplicate File Finder & Cleaner")
                            .color(Color32::from_rgb(156, 163, 175))
                            .size(12.0),
                    );
                });

                ui.add_space(18.0);

                if self.install_complete {
                    ui.vertical_centered(|ui| {
                        dupesweeper::ui::icons::render_icon_circle(
                            ui,
                            dupesweeper::ui::icons::IconKind::Check,
                            48.0,
                            22.0,
                            Color32::from_rgba_premultiplied(34, 197, 94, 35),
                            Color32::from_rgb(34, 197, 94),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            RichText::new("DupeSweeper Berhasil Dipasang!")
                                .color(Color32::from_rgb(34, 197, 94))
                                .size(16.0)
                                .strong(),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new("Shortcut aplikasi 'DupeSweeper' telah siap di Desktop dan Start Menu.")
                                .color(Color32::from_rgb(156, 163, 175))
                                .size(12.0),
                        );
                        ui.add_space(20.0);
                        if ui
                            .add_sized(
                                [200.0, 38.0],
                                egui::Button::new(
                                    RichText::new("Buka DupeSweeper")
                                        .color(Color32::from_rgb(14, 16, 21))
                                        .strong(),
                                )
                                .fill(Color32::from_rgb(96, 165, 250))
                                .rounding(Rounding::same(8.0)),
                            )
                            .clicked()
                        {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                } else {
                    egui::Frame::none()
                        .fill(Color32::from_rgb(20, 24, 33))
                        .stroke(Stroke::new(1.0_f32, Color32::from_rgb(37, 43, 58)))
                        .rounding(Rounding::same(8.0))
                        .inner_margin(14.0)
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("Aplikasi akan dipasang ke sistem:")
                                    .color(Color32::from_rgb(243, 244, 246))
                                    .size(12.5)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            let target_path = dirs::data_local_dir()
                                .map(|d| d.join("Programs").join("DupeSweeper").display().to_string())
                                .unwrap_or_else(|| "%LOCALAPPDATA%\\Programs\\DupeSweeper".to_string());
                            ui.label(
                                RichText::new(target_path)
                                    .color(Color32::from_rgb(156, 163, 175))
                                    .size(11.0),
                            );

                            ui.add_space(10.0);
                            ui.checkbox(
                                &mut self.create_desktop,
                                RichText::new("Buat shortcut di Desktop (bernama \"DupeSweeper\")").size(12.0),
                            );
                            ui.add_space(4.0);
                            ui.checkbox(
                                &mut self.create_start_menu,
                                RichText::new("Buat shortcut di Start Menu Windows (bernama \"DupeSweeper\")").size(12.0),
                            );
                        });

                    if let Some(ref err) = self.error_msg {
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new(format!("Error: {}", err))
                                .color(Color32::from_rgb(239, 68, 68))
                                .size(11.5),
                        );
                    }

                    ui.add_space(16.0);
                    ui.vertical_centered(|ui| {
                        let btn = ui.add_sized(
                            [280.0, 40.0],
                            egui::Button::new(
                                RichText::new("Pasang & Buka DupeSweeper")
                                    .color(Color32::from_rgb(14, 16, 21))
                                    .size(13.5)
                                    .strong(),
                            )
                            .fill(Color32::from_rgb(96, 165, 250))
                            .rounding(Rounding::same(8.0)),
                        );

                        if btn.clicked() {
                            match dupesweeper::installer::install_current_exe(self.create_desktop, self.create_start_menu) {
                                Ok(exe_path) => {
                                    self.install_complete = true;
                                    let _ = std::process::Command::new(exe_path).spawn();
                                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                                }
                                Err(err) => {
                                    self.error_msg = Some(err);
                                }
                            }
                        }
                    });
                }
            });
    }
}

fn run_setup_wizard() -> eframe::Result<()> {
    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([500.0, 390.0])
        .with_resizable(false)
        .with_title("Pemasangan DupeSweeper");

    if let Some(icon) = load_icon() {
        viewport = viewport.with_icon(icon);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "DupeSweeper Setup",
        native_options,
        Box::new(|_cc| Ok(Box::new(SetupWizard::new()))),
    )
}

fn main() -> eframe::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Uninstall argument handler
    if args.iter().any(|a| a == "--uninstall" || a == "/uninstall") {
        let _ = dupesweeper::installer::uninstall();
        return Ok(());
    }

    // Setup mode handler (if executable is named Setup/Installer or passed --install)
    if dupesweeper::installer::is_setup_mode() {
        return run_setup_wizard();
    }

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
