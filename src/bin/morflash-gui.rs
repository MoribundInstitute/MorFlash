// src/bin/morflash-gui.rs

use eframe::NativeOptions;
use morflash_core::gui::app::MorflashGui;

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions::default();

    eframe::run_native(
        "MorFlash",
        native_options,
        Box::new(|cc| Box::new(MorflashGui::new(cc))),
    )
}
