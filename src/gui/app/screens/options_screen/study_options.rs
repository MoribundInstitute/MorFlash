// src/gui/app/screens/options_screen/study_options.rs

use eframe::egui;

use super::state::CardColorMode;

/// Options that control how the study card looks and behaves during review.
#[derive(Clone, Debug)]
pub struct StudyOptions {
    /// How the card background is chosen:
    /// - BuiltIn  => use theme default (Theme::CARD_BG)
    /// - Custom   => use the user-selected color (`card_color`)
    pub card_color_mode: CardColorMode,

    /// The custom background color shown when card_color_mode = Custom.
    pub card_color: egui::Color32,

    /// Whether to show the term first (term → definition) or the opposite
    /// (definition → term).
    pub show_term_first: bool,

    /// Scales the font size on the study card (1.0 = normal).
    pub font_scale: f32,

    /// If true, keep the study card centered and "anchored" in the main area.
    /// If false, you can later implement a draggable floating window style.
    pub center_card: bool,

    /// Enable custom button colors instead of theme defaults.
    pub use_custom_button_color: bool,
    pub button_color: egui::Color32,

    /// Enable a custom text color on the card.
    pub use_custom_font_color: bool,
    pub font_color: egui::Color32,

    /// Enable custom progress bar colors.
    pub use_custom_progress_colors: bool,
    pub progress_fg_color: egui::Color32,
    pub progress_bg_color: egui::Color32,
}

impl Default for StudyOptions {
    fn default() -> Self {
        Self {
            card_color_mode: CardColorMode::BuiltIn,

            // A dark bluish background very close to your original Theme::CARD_BG.
            card_color: egui::Color32::from_rgb(24, 30, 60),

            show_term_first: true,
            font_scale: 1.0,
            center_card: true,

            use_custom_button_color: false,
            // Slightly bright bluish button as a starting point.
            button_color: egui::Color32::from_rgb(70, 150, 230),

            use_custom_font_color: false,
            // Off-white text.
            font_color: egui::Color32::from_rgb(230, 230, 240),

            use_custom_progress_colors: false,
            // Foreground = teal-ish; background = dark muted.
            progress_fg_color: egui::Color32::from_rgb(80, 210, 180),
            progress_bg_color: egui::Color32::from_rgb(30, 40, 60),
        }
    }
}

/// Draw the "Study" options section within the Options screen.
pub fn draw_study_options_section(ui: &mut egui::Ui, study: &mut StudyOptions) {
    ui.heading("Study");
    ui.add_space(8.0);

    // === Layout & content ===
    ui.label("Card content & layout:");
    ui.add_space(4.0);

    ui.checkbox(
        &mut study.show_term_first,
        "Show term first (term → definition)",
    );
    ui.checkbox(
        &mut study.center_card,
        "Keep card centered in the study area",
    );

    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label("Font scale:");
        ui.add(
            egui::Slider::new(&mut study.font_scale, 0.75..=1.5)
                .step_by(0.01)
                .show_value(true),
        );
    });
    ui.label("Tip: increase font scale for couch distance, decrease for dense info.");

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    // === Card background ===
    ui.label("Card background color:");
    ui.horizontal(|ui| {
        ui.radio_value(
            &mut study.card_color_mode,
            CardColorMode::BuiltIn,
            "Use theme color",
        );
        ui.radio_value(
            &mut study.card_color_mode,
            CardColorMode::Custom,
            "Use custom color",
        );
    });

    if matches!(study.card_color_mode, CardColorMode::Custom) {
        ui.add_space(8.0);
        ui.label("Pick a custom card color:");
        ui.color_edit_button_srgba(&mut study.card_color);
        ui.label("Tip: choose a soft or dark shade so text stays readable.");
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    // === Buttons & text ===
    ui.label("Buttons & text:");

    ui.checkbox(
        &mut study.use_custom_button_color,
        "Use custom answer-button color",
    );
    if study.use_custom_button_color {
        ui.add_space(4.0);
        ui.label("Answer button color:");
        ui.color_edit_button_srgba(&mut study.button_color);
    }

    ui.add_space(8.0);
    ui.checkbox(&mut study.use_custom_font_color, "Use custom card text color");
    if study.use_custom_font_color {
        ui.add_space(4.0);
        ui.label("Card text color:");
        ui.color_edit_button_srgba(&mut study.font_color);
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    // === Progress bar ===
    ui.label("Progress bar:");

    ui.checkbox(
        &mut study.use_custom_progress_colors,
        "Use custom progress bar colors",
    );
    if study.use_custom_progress_colors {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Foreground (filled):");
            ui.color_edit_button_srgba(&mut study.progress_fg_color);
        });
        ui.horizontal(|ui| {
            ui.label("Background:");
            ui.color_edit_button_srgba(&mut study.progress_bg_color);
        });
    }
}
