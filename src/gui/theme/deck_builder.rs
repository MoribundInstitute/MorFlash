// src/gui/theme/deck_builder.rs

use eframe::egui;

/// Simple utilitarian theme for the Deck Builder screen.
pub struct DeckBuilderTheme;

impl DeckBuilderTheme {
    pub fn panel_frame() -> egui::Frame {
        egui::Frame {
            fill: egui::Color32::from_rgb(22, 24, 30),
            stroke: egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 90, 110)),
            rounding: egui::Rounding::same(6.0),
            inner_margin: egui::Margin::same(10.0),
            ..Default::default()
        }
    }
}
