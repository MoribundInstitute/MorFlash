// src/bin/morflash-gui.rs

use eframe::{egui, NativeOptions};
use morflash_core::gui::app::MorflashGui;
use std::sync::Arc;

fn main() -> eframe::Result<()> {
    let app_title = "MorFlash";
    let app_icon = load_app_icon(); // Option<Arc<IconData>>

    // Base viewport config shared for both "icon" and "no icon" cases.
    let base_viewport = egui::ViewportBuilder::default()
        .with_inner_size(egui::vec2(1280.0, 720.0))
        .with_min_inner_size(egui::vec2(300.0, 300.0))
        .with_resizable(true)
        .with_decorations(true)
        .with_title(app_title.to_owned());

    // Only attach the icon if we successfully loaded it.
    let viewport = if let Some(icon) = app_icon {
        base_viewport.with_icon((*icon).clone()) // IconData clone from Arc
    } else {
        base_viewport
    };

    let native_options = NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        app_title,
        native_options,
        Box::new(|cc| Ok(Box::new(MorflashGui::new(cc)))),
    )
}

fn load_app_icon() -> Option<Arc<egui::IconData>> {
    use eframe::egui::IconData;
    use std::fs;

    let path = "assets/logos/linux_logos/MorFlashLogo-256x256.png";

    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("MorFlash: failed to read icon at {path}: {e}");
            return None;
        }
    };

    let image = match image::load_from_memory(&bytes) {
        Ok(img) => img.to_rgba8(),
        Err(e) => {
            eprintln!("MorFlash: failed to decode icon PNG: {e}");
            return None;
        }
    };

    let (width, height) = image.dimensions();
    eprintln!("MorFlash: loaded icon {path} ({width}x{height})");

    let rgba = image.into_raw();
    Some(Arc::new(IconData { rgba, width, height }))
}
