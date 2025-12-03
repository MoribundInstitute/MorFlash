// src/bin/morflash-gui.rs

use eframe::{egui, NativeOptions};
use morflash_core::gui::app::MorflashGui;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let app_title = "MorFlash";
    let app_icon = load_app_icon(); // Arc<IconData>

    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(1280.0, 720.0))
            .with_min_inner_size(egui::vec2(300.0, 300.0))
            .with_resizable(true)
            .with_decorations(true)
            .with_title(app_title.to_owned())
            .with_icon((*app_icon).clone()), // Correct: clone Arc<IconData>
        ..Default::default()
    };

    eframe::run_native(
        app_title,
        native_options,
        Box::new(|cc| Ok(Box::new(MorflashGui::new(cc)))),
    )
}

fn load_app_icon() -> Arc<egui::IconData> {
    // Load PNG from embedded bytes
    let image = image::load_from_memory(include_bytes!(
        "../../assets/logo/Moribund Institute Logo.png"
    ))
    .expect("app icon PNG should be valid")
    .into_rgba8();

    let (width, height) = image.dimensions();

    Arc::new(egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    })
}
