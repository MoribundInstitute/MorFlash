// src/gui/app/screens/options_screen.rs
use eframe::egui;
use rfd::FileDialog;
use std::{fs, path::Path};

const CUSTOM_FONT_INDEX: &str = "assets/fonts/custom_fonts.txt";
const CUSTOM_SFX_INDEX: &str = "assets/sfx/custom_sfx.txt";
const CUSTOM_BG_INDEX: &str = "assets/backgrounds/custom_backgrounds.txt";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BackgroundChoice {
    BuiltIn, // default paper texture
    Custom,  // user imported image
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FontChoice {
    MorflashSerif, // Cormorant – default
    Pixel,         // PublicPixel
    System,        // Linux UI / egui default
    Custom,        // User-chosen TTF/OTF file (copied into assets/fonts)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SoundSource {
    BuiltIn,
    Custom,
}

#[derive(Clone, Debug)]
pub struct SoundSlotConfig {
    pub source: SoundSource,
    pub custom_path: Option<String>, // path to chosen .wav/.ogg
}

impl Default for SoundSlotConfig {
    fn default() -> Self {
        Self {
            source: SoundSource::BuiltIn,
            custom_path: None,
        }
    }
}

pub struct OptionsState {
    // ===== Audio =====
    pub sound_enabled: bool,
    /// All imported custom sounds that should persist across runs.
    pub known_custom_sfx: Vec<String>,
    /// Slot for "correct answer" sound.
    pub sound_correct: SoundSlotConfig,
    /// Slot for "incorrect answer" sound.
    pub sound_incorrect: SoundSlotConfig,
    /// Slot for "completion / celebration" sound.
    pub sound_complete: SoundSlotConfig,
    /// Bumped whenever any audio setting changes so the app can reload sounds.
    pub sound_version: u64,

    // ===== Background =====
    pub background_choice: BackgroundChoice,
    /// Path to the current custom background file
    /// Example: "assets/backgrounds/BluePaper.png"
    pub custom_bg_path: Option<String>,
    /// All imported background images that should persist across runs.
    pub known_custom_backgrounds: Vec<String>,

    // ===== Fonts =====
    pub font_choice: FontChoice,
    /// Path to the *current* custom font (usually inside assets/fonts),
    /// e.g. "assets/fonts/MyCustomFont.ttf".
    pub custom_font_path: Option<String>,
    /// All imported custom fonts that should persist across runs.
    pub known_custom_fonts: Vec<String>,
}

impl Default for OptionsState {
    fn default() -> Self {
        Self {
            // Audio
            sound_enabled: true,
            known_custom_sfx: load_known_custom_sfx(),
            sound_correct: SoundSlotConfig::default(),
            sound_incorrect: SoundSlotConfig::default(),
            sound_complete: SoundSlotConfig::default(),
            sound_version: 0,

            // Background
            background_choice: BackgroundChoice::BuiltIn,
            custom_bg_path: None,
            known_custom_backgrounds: load_known_custom_backgrounds(),

            // Fonts
            font_choice: FontChoice::MorflashSerif,
            custom_font_path: None,
            known_custom_fonts: load_known_custom_fonts(),
        }
    }
}

/// Load previously imported custom fonts from disk.
fn load_known_custom_fonts() -> Vec<String> {
    let path = Path::new(CUSTOM_FONT_INDEX);
    if let Ok(text) = fs::read_to_string(path) {
        text.lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    }
}

/// Save the list of known custom fonts to disk.
fn save_known_custom_fonts(list: &[String]) {
    let path = Path::new(CUSTOM_FONT_INDEX);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let body = list.join("\n");
    let _ = fs::write(path, body);
}

/// Load previously imported custom SFX from disk.
fn load_known_custom_sfx() -> Vec<String> {
    let path = Path::new(CUSTOM_SFX_INDEX);
    if let Ok(text) = fs::read_to_string(path) {
        text.lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    }
}

/// Save the list of known custom SFX to disk.
fn save_known_custom_sfx(list: &[String]) {
    let path = Path::new(CUSTOM_SFX_INDEX);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let body = list.join("\n");
    let _ = fs::write(path, body);
}

/// Load previously imported custom backgrounds from disk.
fn load_known_custom_backgrounds() -> Vec<String> {
    let path = Path::new(CUSTOM_BG_INDEX);
    if let Ok(text) = fs::read_to_string(path) {
        text.lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    }
}

/// Save the list of known custom backgrounds to disk.
fn save_known_custom_backgrounds(list: &[String]) {
    let path = Path::new(CUSTOM_BG_INDEX);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let body = list.join("\n");
    let _ = fs::write(path, body);
}

/// Draw the controls for a single sound slot.
fn draw_sound_slot(
    ui: &mut egui::Ui,
    label: &str,
    slot: &mut SoundSlotConfig,
    known_custom_sfx: &mut Vec<String>,
    sound_version: &mut u64,
) {
    ui.group(|ui| {
        ui.label(label);

        ui.horizontal(|ui| {
            if ui
                .radio_value(&mut slot.source, SoundSource::BuiltIn, "Built-in")
                .changed()
            {
                *sound_version = sound_version.wrapping_add(1);
            }
            if ui
                .radio_value(&mut slot.source, SoundSource::Custom, "Custom")
                .changed()
            {
                *sound_version = sound_version.wrapping_add(1);
            }
        });

        if matches!(slot.source, SoundSource::Custom) {
            // List of already-imported custom sounds.
            if !known_custom_sfx.is_empty() {
                for path in known_custom_sfx.iter() {
                    let name = Path::new(path)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or(path.as_str());

                    let is_selected = slot.custom_path.as_deref() == Some(path.as_str());
                    if ui.radio(is_selected, name).clicked() {
                        slot.custom_path = Some(path.clone());
                        *sound_version = sound_version.wrapping_add(1);
                    }
                }
            }

            // Import button
            if ui.button("Import sound…").clicked() {
                if let Some(src) = FileDialog::new()
                    .add_filter("Sound", &["wav", "ogg"])
                    .pick_file()
                {
                    let dest_dir = Path::new("assets/sfx");
                    let _ = fs::create_dir_all(dest_dir);

                    let file_name = src.file_name().unwrap_or_else(|| "custom_sfx.wav".as_ref());
                    let dest_path = dest_dir.join(file_name);

                    match fs::copy(&src, &dest_path) {
                        Ok(_) => {
                            let dest_str = dest_path.to_string_lossy().to_string();

                            if !known_custom_sfx.iter().any(|p| p == &dest_str) {
                                known_custom_sfx.push(dest_str.clone());
                                save_known_custom_sfx(known_custom_sfx);
                            }

                            slot.source = SoundSource::Custom;
                            slot.custom_path = Some(dest_str);
                            *sound_version = sound_version.wrapping_add(1);
                        }
                        Err(e) => {
                            eprintln!("MorFlash: failed to copy custom sound: {e}");
                        }
                    }
                }
            }
        }
    });
}

/// Draw the options screen. Returns `true` if user pressed "Back".
pub fn draw_options(ui: &mut egui::Ui, state: &mut OptionsState) -> bool {
    let mut back = false;

    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading("Options");
        ui.add_space(24.0);

        // ========== AUDIO ==========
        ui.label("Audio");
        if ui
            .checkbox(&mut state.sound_enabled, "Enable sound effects")
            .changed()
        {
            state.sound_version = state.sound_version.wrapping_add(1);
        }

        ui.add_space(8.0);

        draw_sound_slot(
            ui,
            "Correct answer sound",
            &mut state.sound_correct,
            &mut state.known_custom_sfx,
            &mut state.sound_version,
        );

        draw_sound_slot(
            ui,
            "Incorrect answer sound",
            &mut state.sound_incorrect,
            &mut state.known_custom_sfx,
            &mut state.sound_version,
        );

        draw_sound_slot(
            ui,
            "Completion sound (when set is finished)",
            &mut state.sound_complete,
            &mut state.known_custom_sfx,
            &mut state.sound_version,
        );

        ui.add_space(24.0);

        // ========== BACKGROUND ==========
        ui.label("Background");

        ui.horizontal(|ui| {
            ui.radio_value(
                &mut state.background_choice,
                BackgroundChoice::BuiltIn,
                "Built-in paper texture",
            );
            ui.radio_value(
                &mut state.background_choice,
                BackgroundChoice::Custom,
                "Custom background",
            );
        });

        if matches!(state.background_choice, BackgroundChoice::Custom) {
            ui.add_space(8.0);

            // List known custom backgrounds
            if !state.known_custom_backgrounds.is_empty() {
                for path in &state.known_custom_backgrounds {
                    let name = Path::new(path)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or(path);

                    let is_current = state
                        .custom_bg_path
                        .as_deref()
                        .map(|p| p == path.as_str())
                        .unwrap_or(false);

                    if ui.radio(is_current, name).clicked() {
                        state.custom_bg_path = Some(path.clone());
                    }
                }
            }

            // Import custom background
            if ui.button("Import background…").clicked() {
                if let Some(src) = FileDialog::new()
                    .add_filter("Images", &["png", "jpg", "jpeg"])
                    .pick_file()
                {
                    let dest_dir = Path::new("assets/backgrounds");
                    let _ = fs::create_dir_all(dest_dir);

                    let file_name = src
                        .file_name()
                        .unwrap_or_else(|| "custom_background.png".as_ref());
                    let dest_path = dest_dir.join(file_name);

                    match fs::copy(&src, &dest_path) {
                        Ok(_) => {
                            let dest_str = dest_path.to_string_lossy().to_string();

                            if !state
                                .known_custom_backgrounds
                                .iter()
                                .any(|p| p == &dest_str)
                            {
                                state.known_custom_backgrounds.push(dest_str.clone());
                                save_known_custom_backgrounds(&state.known_custom_backgrounds);
                            }

                            state.background_choice = BackgroundChoice::Custom;
                            state.custom_bg_path = Some(dest_str);
                        }
                        Err(e) => {
                            eprintln!("MorFlash: failed to copy custom background: {e}");
                        }
                    }
                }
            }

            ui.label("Tip: use a seamless / tiling image for best results.");
        }

        ui.add_space(24.0);

        // ========== FONT ==========
        ui.label("Font");

        ui.radio_value(
            &mut state.font_choice,
            FontChoice::MorflashSerif,
            "MorFlash serif (Cormorant)",
        );
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
            "Custom font (file)",
        );

        // When Custom is selected, show the file picker + imported custom font options.
        if matches!(state.font_choice, FontChoice::Custom) {
            ui.add_space(8.0);

            // Always have a string while editing
            let path_buf = state.custom_font_path.get_or_insert_with(String::new);

            ui.horizontal(|ui| {
                ui.label("Font file:");
                ui.text_edit_singleline(path_buf);

                if ui.button("Browse…").clicked() {
                    if let Some(src_path) = FileDialog::new()
                        .add_filter("Fonts", &["ttf", "otf"])
                        .pick_file()
                    {
                        let dest_dir = Path::new("assets/fonts");
                        let _ = fs::create_dir_all(dest_dir);

                        let file_name = src_path
                            .file_name()
                            .unwrap_or_else(|| "custom_font.ttf".as_ref());
                        let dest_path = dest_dir.join(file_name);

                        match fs::copy(&src_path, &dest_path) {
                            Ok(_) => {
                                let dest_str = dest_path.to_string_lossy().to_string();
                                *path_buf = dest_str.clone();

                                // Add to known list if not already present.
                                if !state.known_custom_fonts.iter().any(|p| p == &dest_str) {
                                    state.known_custom_fonts.push(dest_str);
                                    save_known_custom_fonts(&state.known_custom_fonts);
                                }

                                // Ensure we're in Custom mode so the font is used right away.
                                state.font_choice = FontChoice::Custom;

                                eprintln!("Copied custom font to {}", path_buf);
                            }
                            Err(e) => {
                                eprintln!("Failed to copy custom font: {e}");
                                *path_buf = src_path.to_string_lossy().to_string();
                            }
                        }
                    }
                }
            });

            ui.label("Choose a .ttf or .otf font file.");

            // ===== Imported custom fonts as part of the main font list =====
            if !state.known_custom_fonts.is_empty() {
                ui.add_space(8.0);

                for font_path in &state.known_custom_fonts {
                    let name = Path::new(font_path)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or(font_path);

                    let is_current = state
                        .custom_font_path
                        .as_deref()
                        .map(|p| p == font_path)
                        .unwrap_or(false)
                        && matches!(state.font_choice, FontChoice::Custom);

                    // Radio behaves like the others; clicking it selects this font.
                    if ui.radio(is_current, name).clicked() {
                        state.custom_font_path = Some(font_path.clone());
                        state.font_choice = FontChoice::Custom;
                    }
                }
            }
        }

        ui.add_space(32.0);

        if ui.button("⬛ Back").clicked() {
            back = true;
        }
    });

    back
}
