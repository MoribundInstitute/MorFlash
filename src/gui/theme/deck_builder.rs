use eframe::egui;

/// Simple utilitarian theme for the Deck Builder screen.
pub struct DeckBuilderTheme;

impl DeckBuilderTheme {
    pub fn panel_frame() -> egui::Frame {
        egui::Frame {
            fill: egui::Color32::TRANSPARENT,
            stroke: egui::Stroke::NONE,
            rounding: egui::Rounding::same(0.0),
            inner_margin: egui::Margin::same(4.0),
            ..Default::default()
        }
    }
}
