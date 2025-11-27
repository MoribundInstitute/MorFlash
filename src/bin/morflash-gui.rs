// src/bin/morflash-gui.rs

use eframe::{egui, NativeOptions};
use morflash_core::gui::app::MorflashGui;

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions::default();

    eframe::run_native(
        "MorFlash",
        native_options,
        Box::new(|cc| {
            // ============================
            // 🔧 UI SCALING
            // ============================
            cc.egui_ctx.set_pixels_per_point(1.7);

            // ============================
            // 🔤 CUSTOM FONT: IM Fell English
            // ============================
            let mut fonts = egui::FontDefinitions::default();

            // Make sure the file exists at: assets/fonts/IMFellEnglish-Regular.ttf
            fonts.font_data.insert(
                "IMFellEnglish".to_owned(),
                egui::FontData::from_static(include_bytes!(
                    "../../assets/fonts/IMFellEnglish-Regular.ttf"
                )),
            );

            // Use IM Fell English as default proportional font
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "IMFellEnglish".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            // ============================
            // 🎨 COLOR THEME
            // ============================
            let mut style = (*cc.egui_ctx.style()).clone();

            let bg = egui::Color32::from_rgb(0x08, 0x08, 0x08); // #080808
            let text = egui::Color32::from_rgb(0xEC, 0xEC, 0xEB); // #ECECEB
            let button = egui::Color32::from_rgb(0xA3, 0xA5, 0xDB); // #A3A5DB
            let button_hover = egui::Color32::from_rgb(0xB7, 0xB8, 0xE8); // slightly brighter
            let button_text = egui::Color32::BLACK;

            style.visuals.dark_mode = true;
            style.visuals.override_text_color = Some(text);
            style.visuals.window_fill = bg;
            style.visuals.panel_fill = bg;

            {
                use egui::style::WidgetVisuals;
                use egui::{Rounding, Stroke};

                // Inactive (normal) buttons
                let base_inactive = style.visuals.widgets.inactive;
                style.visuals.widgets.inactive = WidgetVisuals {
                    bg_fill: button,
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, button_text),
                    rounding: Rounding::same(6.0),
                    ..base_inactive
                };

                // Hovered buttons
                let base_hovered = style.visuals.widgets.hovered;
                style.visuals.widgets.hovered = WidgetVisuals {
                    bg_fill: button_hover,
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, button_text),
                    rounding: Rounding::same(6.0),
                    expansion: 1.0,
                    ..base_hovered
                };

                // Active (pressed) buttons
                let base_active = style.visuals.widgets.active;
                style.visuals.widgets.active = WidgetVisuals {
                    bg_fill: button,
                    bg_stroke: Stroke::NONE,
                    fg_stroke: Stroke::new(1.0, button_text),
                    rounding: Rounding::same(6.0),
                    ..base_active
                };
            }

            cc.egui_ctx.set_style(style);

            // ============================
            // 🚀 LAUNCH APP
            // ============================
            Box::new(MorflashGui::new())
        }),
    )
}
