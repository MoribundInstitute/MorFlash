// src/bin/morflash-gui.rs

use eframe::{egui, NativeOptions};
use morflash_core::gui::app::MorflashGui;

fn main() -> eframe::Result<()> {
    let app_title = "MorFlash";

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(1280.0, 720.0))
            .with_min_inner_size(egui::vec2(800.0, 450.0))
            .with_resizable(true)
            .with_decorations(true)
            .with_title(app_title.to_owned()),
        // TODO: When you have an .ico/.png ready, wire it up here:
        // viewport: egui::ViewportBuilder::default()
        //     .with_icon(egui::IconData { ... }),
        ..Default::default()
    };

    eframe::run_native(
        app_title,
        native_options,
        Box::new(|cc| Ok(Box::new(MorflashGui::new(cc)))),
    )
}
