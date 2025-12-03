// src/gui/app/screens/options_screen/state.rs

use std::{fs, path::Path};

use super::{
    completion_options::CompletionOptions,
    deck_builder_options::DeckBuilderOptions,
    global_options::GlobalOptions,
    main_menu_options::MainMenuOptions,
    study_options::StudyOptions,
};

/// Index files for custom assets (paths to user-imported resources).
pub const CUSTOM_FONT_INDEX: &str = "assets/fonts/custom_fonts.txt";
pub const CUSTOM_SFX_INDEX: &str = "assets/sfx/custom_sfx.txt";
pub const CUSTOM_BG_INDEX: &str = "assets/backgrounds/custom_backgrounds.txt";

/// Shared helper: load a simple newline-separated index file into a Vec<String>.
fn load_index(path_str: &str) -> Vec<String> {
    let path = Path::new(path_str);
    fs::read_to_string(path)
        .map(|text| {
            text.lines()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Shared helper: save a newline-separated index file from a Vec<String>.
fn save_index(path_str: &str, list: &[String]) {
    let path = Path::new(path_str);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let body = list.join("\n");
    let _ = fs::write(path, body);
}

/// Load the list of known custom font files from disk.
pub(crate) fn load_known_custom_fonts() -> Vec<String> {
    load_index(CUSTOM_FONT_INDEX)
}

/// Persist the list of known custom font files to disk.
pub(crate) fn save_known_custom_fonts(list: &[String]) {
    save_index(CUSTOM_FONT_INDEX, list)
}

/// Load the list of known custom SFX files from disk.
pub(crate) fn load_known_custom_sfx() -> Vec<String> {
    load_index(CUSTOM_SFX_INDEX)
}

/// Persist the list of known custom SFX files to disk.
pub(crate) fn save_known_custom_sfx(list: &[String]) {
    save_index(CUSTOM_SFX_INDEX, list)
}

/// Load the list of known custom background images from disk.
pub(crate) fn load_known_custom_backgrounds() -> Vec<String> {
    load_index(CUSTOM_BG_INDEX)
}

/// Persist the list of known custom background images to disk.
pub(crate) fn save_known_custom_backgrounds(list: &[String]) {
    save_index(CUSTOM_BG_INDEX, list)
}

/// How study cards choose their color.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CardColorMode {
    BuiltIn,
    Custom,
}

impl Default for CardColorMode {
    fn default() -> Self {
        CardColorMode::BuiltIn
    }
}

/// Shared background choice enum (used by global options for tiling BG).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BackgroundChoice {
    BuiltIn,
    Custom,
}

impl Default for BackgroundChoice {
    fn default() -> Self {
        BackgroundChoice::BuiltIn
    }
}

/// Shared font choice enum (used by global options for all screens).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FontChoice {
    MorflashSerif,
    Pixel,
    System,
    Custom,
}

impl Default for FontChoice {
    fn default() -> Self {
        FontChoice::MorflashSerif
    }
}

/// Where a sound comes from (built-in vs custom file).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SoundSource {
    BuiltIn,
    Custom,
}

impl Default for SoundSource {
    fn default() -> Self {
        SoundSource::BuiltIn
    }
}

/// Configuration for a single sound slot (correct / incorrect / complete, etc.).
#[derive(Clone, Debug)]
pub struct SoundSlotConfig {
    pub source: SoundSource,
    /// Path to the custom sound file, if any.
    pub custom_path: Option<String>,
}

impl Default for SoundSlotConfig {
    fn default() -> Self {
        Self {
            source: SoundSource::BuiltIn,
            custom_path: None,
        }
    }
}

/// Top-level options state that groups all option sections.
///
/// Each section (`global`, `study`, `completion`, `main_menu`, `deck_builder`)
/// owns its own fields; `OptionsState` is just the container that the rest of
/// the app passes around.
#[derive(Clone, Debug)]
pub struct OptionsState {
    pub global: GlobalOptions,
    pub study: StudyOptions,
    pub completion: CompletionOptions,
    pub main_menu: MainMenuOptions,
    pub deck_builder: DeckBuilderOptions,
}

impl Default for OptionsState {
    fn default() -> Self {
        Self {
            global: GlobalOptions::default(),
            study: StudyOptions::default(),
            completion: CompletionOptions::default(),
            main_menu: MainMenuOptions::default(),
            deck_builder: DeckBuilderOptions::default(),
        }
    }
}
