// src/gui/app/screens/options_screen/main_menu_options.rs
use eframe::egui;

#[derive(Debug, Clone)]
pub struct MainMenuOptions {
    /// Show the little critter next to the focused button.
    pub show_critter: bool,

    /// Animation speed multiplier for the critter (1.0 = normal).
    pub critter_anim_speed: f32,

    /// Slightly enlarge / emphasize buttons on hover.
    pub emphasize_hovered_buttons: bool,

    /// Show the helper tip text at the bottom of the main menu.
    pub show_tip_text: bool,

    /// Use a more compact main menu layout instead of wide spacing.
    pub compact_layout: bool,
}

impl Default for MainMenuOptions {
    fn default() -> Self {
        Self {
            show_critter: true,
            critter_anim_speed: 1.0,
            emphasize_hovered_buttons: true,
            show_tip_text: true,
            compact_layout: false,
        }
    }
}

pub fn draw_main_menu_options_section(
    ui: &mut egui::Ui,
    main_menu: &mut MainMenuOptions,
) {
    ui.heading("Main Menu");
    ui.add_space(8.0);

    // === Critter ===
    ui.checkbox(&mut main_menu.show_critter, "Show critter")
        .on_hover_text("Toggle the little companion that sits by the focused button.");

    ui.add_enabled_ui(main_menu.show_critter, |ui| {
        ui.horizontal(|ui| {
            ui.label("Critter animation speed:");
            ui.add(
                egui::Slider::new(&mut main_menu.critter_anim_speed, 0.25..=2.0)
                    .step_by(0.05)
                    .show_value(true),
            );
        })
        .response
        .on_hover_text("Slow it down or speed it up to taste.");
    });

    ui.add_space(12.0);

    // === Button behavior ===
    ui.checkbox(
        &mut main_menu.emphasize_hovered_buttons,
        "Emphasize hovered buttons",
    )
    .on_hover_text("Adds a subtle visual bump or glow when you hover a main menu button.");

    ui.add_space(12.0);

    // === Layout & Tips ===
    ui.checkbox(&mut main_menu.show_tip_text, "Show tip text at the bottom")
        .on_hover_text("Controls the little help/snark line under the main menu.");

    ui.checkbox(&mut main_menu.compact_layout, "Use compact layout")
        .on_hover_text("Reduces vertical spacing between main menu elements.");
}
