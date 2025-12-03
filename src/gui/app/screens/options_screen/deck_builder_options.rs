// src/gui/app/screens/options_screen/deck_builder_options.rs
use eframe::egui;

#[derive(Debug, Clone)]
pub struct DeckBuilderOptions {
    /// Automatically save the deck file while editing.
    pub autosave_enabled: bool,

    /// Autosave interval in seconds.
    pub autosave_interval_secs: f32,

    /// Whether new cards should start with blank fields, or copy front/back 
    /// formats from the previous card.
    pub new_card_mode: NewCardMode,

    /// Show optional “Advanced” metadata fields (tags, media path, etc.)
    pub show_advanced_fields: bool,

    /// Warn the user before closing the builder when unsaved changes exist.
    pub warn_on_unsaved_exit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewCardMode {
    Blank,
    ClonePrevious,
}

impl Default for DeckBuilderOptions {
    fn default() -> Self {
        Self {
            autosave_enabled: true,
            autosave_interval_secs: 10.0,
            new_card_mode: NewCardMode::Blank,
            show_advanced_fields: false,
            warn_on_unsaved_exit: true,
        }
    }
}

pub fn draw_deck_builder_options_section(
    ui: &mut egui::Ui,
    opts: &mut DeckBuilderOptions,
) {
    ui.heading("Deck Builder");
    ui.add_space(8.0);

    // === Autosave ===
    ui.checkbox(&mut opts.autosave_enabled, "Enable autosave");

    if opts.autosave_enabled {
        ui.horizontal(|ui| {
            ui.label("Autosave interval (seconds):");
            ui.add(
                egui::DragValue::new(&mut opts.autosave_interval_secs)
                    .clamp_range(2.0..=120.0)
                    .speed(0.2),
            );
        });
    }
    ui.add_space(16.0);

    // === New card creation mode ===
    ui.label("Default new card mode:");
    ui.radio_value(&mut opts.new_card_mode, NewCardMode::Blank, "Start blank");
    ui.radio_value(
        &mut opts.new_card_mode,
        NewCardMode::ClonePrevious,
        "Clone previous card’s format",
    );
    ui.add_space(16.0);

    // === Advanced fields ===
    ui.checkbox(
        &mut opts.show_advanced_fields,
        "Show advanced metadata fields",
    );
    ui.label("Advanced fields include tags, media paths, hyperlinks, etc.");
    ui.add_space(16.0);

    // === Unsaved exit warning ===
    ui.checkbox(
        &mut opts.warn_on_unsaved_exit,
        "Warn before closing builder if there are unsaved changes",
    );
}
