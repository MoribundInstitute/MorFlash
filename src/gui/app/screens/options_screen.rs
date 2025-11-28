// src/gui/options_screen.rs
use eframe::egui;
use std::path::PathBuf;

/// Which font the user wants to use.
#[derive(Clone, Copy, PartialEq)]
pub enum FontChoice {
    Pixel,
    System,
    Custom,
}

/// All settings controlled by the Options screen.
pub struct OptionsState {
    pub sound_enabled: bool,
    pub font_choice: FontChoice,
    pub custom_font_path: Option<PathBuf>,
}

pub fn draw_options(ui: &mut egui::Ui, state: &mut OptionsState) -> bool {
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add_space(30.0);

        ui.label(
            egui::RichText::new("Options")
                .size(36.0)
                .strong()
                .color(egui::Color32::WHITE),
        );

        ui.add_space(20.0);

        // === Audio ===
        ui.heading("Audio");
        ui.checkbox(&mut state.sound_enabled, "Enable sound effects");

        ui.add_space(24.0);

        // === Fonts ===
        ui.heading("Font");
        ui.add_space(4.0);

        ui.radio_value(
            &mut state.font_choice,
            FontChoice::Pixel,
            "Pixel font (PublicPixel)",
        );
        ui.radio_value(
            &mut state.font_choice,
            FontChoice::System,
            "System / default font",
        );
        ui.radio_value(
            &mut state.font_choice,
            FontChoice::Custom,
            "Custom font (path)",
        );

        if state.font_choice == FontChoice::Custom {
            ui.add_space(8.0);

            // simple text box to enter a path for now
            let mut path_str = state
                .custom_font_path
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();

            let resp = ui.add(
                egui::TextEdit::singleline(&mut path_str).hint_text("Enter path to .ttf or .otf"),
            );

            if resp.changed() {
                if !path_str.trim().is_empty() {
                    state.custom_font_path = Some(PathBuf::from(path_str));
                } else {
                    state.custom_font_path = None;
                }
            }

            ui.label(
                egui::RichText::new("Later you can replace this with a file picker.")
                    .size(12.0)
                    .color(egui::Color32::from_gray(170)),
            );
        }

        ui.add_space(30.0);

        let back_btn = ui.button("← Back");
        back_btn.clicked()
    })
    .inner
}
