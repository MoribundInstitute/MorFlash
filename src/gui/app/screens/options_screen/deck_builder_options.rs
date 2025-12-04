// src/gui/app/screens/options_screen/deck_builder_options.rs
use eframe::egui;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LanguageEntry {
    /// Human-readable language name, e.g. "English".
    pub name: String,
    /// Language code, e.g. "en", "ja".
    pub code: String,
    /// Whether this language is available in the Deck Builder dropdowns.
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NewCardMode {
    /// New cards start completely blank.
    Blank,
    /// New cards copy the previous card's structure/fields.
    ClonePrevious,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

    /// Languages that can appear in the Deck Builder term/definition dropdowns.
    pub languages: Vec<LanguageEntry>,
}

impl Default for DeckBuilderOptions {
    fn default() -> Self {
        Self {
            autosave_enabled: true,
            autosave_interval_secs: 10.0,
            new_card_mode: NewCardMode::Blank,
            show_advanced_fields: false,
            warn_on_unsaved_exit: true,
            languages: vec![
                LanguageEntry {
                    name: "English".into(),
                    code: "en".into(),
                    enabled: true,
                },
                LanguageEntry {
                    name: "Japanese".into(),
                    code: "ja".into(),
                    enabled: true,
                },
                LanguageEntry {
                    name: "Korean".into(),
                    code: "ko".into(),
                    enabled: false,
                },
                LanguageEntry {
                    name: "Chinese".into(),
                    code: "zh".into(),
                    enabled: false,
                },
                LanguageEntry {
                    name: "French".into(),
                    code: "fr".into(),
                    enabled: false,
                },
                LanguageEntry {
                    name: "Spanish".into(),
                    code: "es".into(),
                    enabled: false,
                },
                // Add more defaults here if you like.
            ],
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
                    .range(2.0..=120.0)
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
    ui.add_space(16.0);

    // === Language availability ===
    ui.collapsing("Languages available in Deck Builder", |ui| {
        ui.label("Toggle which languages appear in the card language dropdowns.");

        egui::ScrollArea::vertical()
            .max_height(180.0)
            .show(ui, |ui| {
                for lang in &mut opts.languages {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut lang.enabled, "");
                        ui.label(&lang.name);
                        ui.label(format!("({})", lang.code));
                    });
                }
            });
    });
}
