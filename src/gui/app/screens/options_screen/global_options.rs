// src/gui/app/screens/options_screen/global_options.rs

use eframe::egui;
use rfd::FileDialog;
use std::{fs, path::Path};

use crate::gui::theme::MenuTheme;

use super::state::{
    BackgroundChoice,
    FontChoice,
    SoundSlotConfig,
    load_known_custom_backgrounds,
    load_known_custom_fonts,
    load_known_custom_sfx,
    save_known_custom_backgrounds,
    save_known_custom_fonts,
    save_known_custom_sfx,
};

/// Tiny PC-98 style square toggle used throughout the options UI.
fn square_choice(ui: &mut egui::Ui, selected: bool, label: &str) -> bool {
    let mut clicked = false;

    ui.horizontal(|ui| {
        let size = egui::vec2(14.0, 14.0);
        let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());
        let painter = ui.painter_at(rect);

        let border = MenuTheme::BUTTON_OUTLINE;
        let bg_off = MenuTheme::PANEL_BG;
        let bg_on = egui::Color32::from_rgba_unmultiplied(0, 200, 255, 40);

        painter.rect_filled(rect, 2.0, if selected { bg_on } else { bg_off });
        painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, border));

        if selected {
            let inner = rect.shrink(3.0);
            painter.rect_filled(inner, 1.0, egui::Color32::from_rgb(0, 200, 255));
        }

        if resp.clicked() {
            clicked = true;
        }

        let label_resp = ui.label(label);
        if label_resp.clicked() {
            clicked = true;
        }
    });

    clicked
}

/// Generic helper for importing a file into the assets tree.
fn copy_chosen_file(
    filter_desc: &str,
    exts: &[&str],
    dest_dir_str: &str,
    default_name: &str,
) -> Option<String> {
    let src = FileDialog::new()
        .add_filter(filter_desc, exts)
        .pick_file()?;

    let dest_dir = Path::new(dest_dir_str);
    let _ = fs::create_dir_all(dest_dir);

    let file_name = src
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new(default_name));
    let dest_path = dest_dir.join(file_name);

    match fs::copy(&src, &dest_path) {
        Ok(_) => Some(dest_path.to_string_lossy().to_string()),
        Err(e) => {
            eprintln!("MorFlash: failed to copy file to {dest_dir_str}: {e}");
            None
        }
    }
}

/// Draw UI for a single sound slot (built-in vs custom, with import + list).
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
            if square_choice(
                ui,
                matches!(slot.source, super::state::SoundSource::BuiltIn),
                "Built-in",
            ) {
                slot.source = super::state::SoundSource::BuiltIn;
                *sound_version = sound_version.wrapping_add(1);
            }

            if square_choice(
                ui,
                matches!(slot.source, super::state::SoundSource::Custom),
                "Custom",
            ) {
                slot.source = super::state::SoundSource::Custom;
                *sound_version = sound_version.wrapping_add(1);
            }
        });

        if matches!(slot.source, super::state::SoundSource::Custom) {
            if !known_custom_sfx.is_empty() {
                for path in known_custom_sfx.iter() {
                    let name = Path::new(path)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or(path.as_str());

                    let is_selected = slot.custom_path.as_deref() == Some(path.as_str());
                    if square_choice(ui, is_selected, name) {
                        slot.custom_path = Some(path.clone());
                        *sound_version = sound_version.wrapping_add(1);
                    }
                }
            }

            if ui.button("Import sound…").clicked() {
                if let Some(dest_str) = copy_chosen_file(
                    "Sound",
                    &["wav", "ogg"],
                    "assets/sfx",
                    "custom_sfx.ogg",
                ) {
                    if !known_custom_sfx.iter().any(|p| p == &dest_str) {
                        known_custom_sfx.push(dest_str.clone());
                        save_known_custom_sfx(known_custom_sfx);
                    }

                    slot.source = super::state::SoundSource::Custom;
                    slot.custom_path = Some(dest_str);
                    *sound_version = sound_version.wrapping_add(1);
                }
            }
        }
    });
}

#[derive(Debug, Clone)]
pub struct GlobalOptions {
    // Audio
    pub sound_enabled: bool,
    pub master_volume: f32,
    pub sound_version: u64,
    pub sound_correct: SoundSlotConfig,
    pub sound_incorrect: SoundSlotConfig,
    pub sound_complete: SoundSlotConfig,
    pub sound_ui_select: SoundSlotConfig,      // ← NEW
    pub known_custom_sfx: Vec<String>,

    // Background (tiling image that applies to all screens)
    pub background_choice: BackgroundChoice,
    pub custom_bg_path: Option<String>,
    pub known_custom_backgrounds: Vec<String>,

    // Fonts (applied globally to all screens)
    pub font_choice: FontChoice,
    pub custom_font_path: Option<String>,
    pub known_custom_fonts: Vec<String>,

    // UI / debug
    pub ui_scale: f32,
    pub debug_enabled: bool,
}

impl Default for GlobalOptions {
    fn default() -> Self {
        Self {
            // Audio
            sound_enabled: true,
            master_volume: 1.0,
            sound_version: 0,
            sound_correct: SoundSlotConfig::default(),
            sound_incorrect: SoundSlotConfig::default(),
            sound_complete: SoundSlotConfig::default(),
            sound_ui_select: SoundSlotConfig::default(),  // ← NEW
            known_custom_sfx: load_known_custom_sfx(),

            // Background
            background_choice: BackgroundChoice::BuiltIn,
            custom_bg_path: None,
            known_custom_backgrounds: load_known_custom_backgrounds(),

            // Fonts
            font_choice: FontChoice::MorflashSerif,
            custom_font_path: None,
            known_custom_fonts: load_known_custom_fonts(),

            // UI / debug
            ui_scale: 1.0,
            debug_enabled: false,
        }
    }
}

/// Draw the "Global" options section (audio, background, font, UI scale, debug, etc.).
pub fn draw_global_options_section(ui: &mut egui::Ui, global: &mut GlobalOptions) {
    // === AUDIO ===
    ui.heading("Audio");
    ui.add_space(8.0);

    // Sound enable checkbox; bump version if it changes.
    let prev_enabled = global.sound_enabled;
    ui.checkbox(&mut global.sound_enabled, "Enable sound effects");
    if global.sound_enabled != prev_enabled {
        global.sound_version = global.sound_version.wrapping_add(1);
    }

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label("Master volume:");
        ui.add(
            egui::Slider::new(&mut global.master_volume, 0.0..=1.0)
                .show_value(true),
        );
    })
    .response
    .on_hover_text("Adjust how loud all sound effects are.");

    ui.add_space(8.0);

    // Individual sound slots.
    draw_sound_slot(
        ui,
        "Correct answer sound",
        &mut global.sound_correct,
        &mut global.known_custom_sfx,
        &mut global.sound_version,
    );
    ui.add_space(6.0);

    draw_sound_slot(
        ui,
        "Incorrect answer sound",
        &mut global.sound_incorrect,
        &mut global.known_custom_sfx,
        &mut global.sound_version,
    );
    ui.add_space(6.0);

    draw_sound_slot(
        ui,
        "Completion sound (when set is finished)",
        &mut global.sound_complete,
        &mut global.known_custom_sfx,
        &mut global.sound_version,
    );
    ui.add_space(6.0);

    draw_sound_slot(
        ui,
        "UI select sound",
        &mut global.sound_ui_select,
        &mut global.known_custom_sfx,
        &mut global.sound_version,
    );

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(16.0);

    // === BACKGROUND ===
    ui.heading("Background");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        if square_choice(
            ui,
            matches!(global.background_choice, BackgroundChoice::BuiltIn),
            "Built-in paper texture",
        ) {
            global.background_choice = BackgroundChoice::BuiltIn;
        }
        if square_choice(
            ui,
            matches!(global.background_choice, BackgroundChoice::Custom),
            "Custom tiling background",
        ) {
            global.background_choice = BackgroundChoice::Custom;
        }
    });

    if matches!(global.background_choice, BackgroundChoice::Custom) {
        ui.add_space(8.0);

        // List known custom backgrounds.
        if !global.known_custom_backgrounds.is_empty() {
            for path in &global.known_custom_backgrounds {
                let name = Path::new(path)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(path);

                let is_current = global
                    .custom_bg_path
                    .as_deref()
                    .map(|p| p == path.as_str())
                    .unwrap_or(false);

                if square_choice(ui, is_current, name) {
                    global.custom_bg_path = Some(path.clone());
                }
            }
        }

        if ui.button("Import background…").clicked() {
            if let Some(dest_str) = copy_chosen_file(
                "Images",
                &["png", "jpg", "jpeg"],
                "assets/backgrounds",
                "custom_background.png",
            ) {
                if !global
                    .known_custom_backgrounds
                    .iter()
                    .any(|p| p == &dest_str)
                {
                    global.known_custom_backgrounds.push(dest_str.clone());
                    save_known_custom_backgrounds(&global.known_custom_backgrounds);
                }

                global.background_choice = BackgroundChoice::Custom;
                global.custom_bg_path = Some(dest_str);
            }
        }

        ui.label("Tip: use a seamless / tiling image for best results.");
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(16.0);

    // === FONT ===
    ui.heading("Font");
    ui.add_space(8.0);

    for (variant, label) in [
        (FontChoice::MorflashSerif, "MorFlash serif (Cormorant)"),
        (FontChoice::Pixel, "Pixel font (PublicPixel)"),
        (FontChoice::System, "System / default font"),
        (FontChoice::Custom, "Custom font (file)"),
    ] {
        let selected = global.font_choice == variant;
        if square_choice(ui, selected, label) {
            global.font_choice = variant;
        }
    }

    if matches!(global.font_choice, FontChoice::Custom) {
        ui.add_space(8.0);

        let path_buf = global.custom_font_path.get_or_insert_with(String::new);

        ui.horizontal(|ui| {
            ui.label("Font file:");
            ui.text_edit_singleline(path_buf);

            if ui.button("Browse…").clicked() {
                if let Some(dest_str) = copy_chosen_file(
                    "Fonts",
                    &["ttf", "otf"],
                    "assets/fonts",
                    "custom_font.ttf",
                ) {
                    *path_buf = dest_str.clone();

                    if !global.known_custom_fonts.iter().any(|p| p == &dest_str) {
                        global.known_custom_fonts.push(dest_str);
                        save_known_custom_fonts(&global.known_custom_fonts);
                    }

                    global.font_choice = FontChoice::Custom;
                }
            }
        });

        ui.label("Choose a .ttf or .otf font file.");

        if !global.known_custom_fonts.is_empty() {
            ui.add_space(8.0);
            for font_path in &global.known_custom_fonts {
                let name = Path::new(font_path)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or(font_path);

                let is_current = global
                    .custom_font_path
                    .as_deref()
                    .map(|p| p == font_path)
                    .unwrap_or(false);

                if square_choice(ui, is_current, name) {
                    global.custom_font_path = Some(font_path.clone());
                    global.font_choice = FontChoice::Custom;
                }
            }
        }
    }

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(16.0);

    // === INTERFACE / DEBUG ===
    ui.heading("Interface");
    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("UI scale:");
        ui.add(
            egui::Slider::new(&mut global.ui_scale, 0.75..=1.5)
                .step_by(0.01)
                .show_value(true),
        );
    })
    .response
    .on_hover_text("Increase or decrease the overall size of all UI elements.");

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(16.0);

    ui.heading("Debug");
    ui.add_space(8.0);

    ui.checkbox(&mut global.debug_enabled, "Enable debug overlay");
    ui.label(
        "Shows extra diagnostics in the UI (card IDs, raw SRS state, \
         and other nerdy goodness).",
    );
}
