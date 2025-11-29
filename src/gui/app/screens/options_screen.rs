use eframe::egui;
use egui::{Color32, FontId, TextureHandle};
use rfd::FileDialog;
use std::{fs, path::Path};

use crate::gui::menu_theme::MenuTheme;

const CUSTOM_FONT_INDEX: &str = "assets/fonts/custom_fonts.txt";
const CUSTOM_SFX_INDEX: &str = "assets/sfx/custom_sfx.txt";
const CUSTOM_BG_INDEX: &str = "assets/backgrounds/custom_backgrounds.txt";

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BackgroundChoice {
    BuiltIn,
    Custom,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FontChoice {
    MorflashSerif,
    Pixel,
    System,
    Custom,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SoundSource {
    BuiltIn,
    Custom,
}

#[derive(Clone, Debug)]
pub struct SoundSlotConfig {
    pub source: SoundSource,
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

pub struct OptionsState {
    // Audio
    pub sound_enabled: bool,
    pub known_custom_sfx: Vec<String>,
    pub sound_correct: SoundSlotConfig,
    pub sound_incorrect: SoundSlotConfig,
    pub sound_complete: SoundSlotConfig,
    pub sound_version: u64,

    // Background
    pub background_choice: BackgroundChoice,
    pub custom_bg_path: Option<String>,
    pub known_custom_backgrounds: Vec<String>,

    // Fonts
    pub font_choice: FontChoice,
    pub custom_font_path: Option<String>,
    pub known_custom_fonts: Vec<String>,
}

impl Default for OptionsState {
    fn default() -> Self {
        Self {
            sound_enabled: true,
            known_custom_sfx: load_known_custom_sfx(),
            sound_correct: SoundSlotConfig::default(),
            sound_incorrect: SoundSlotConfig::default(),
            sound_complete: SoundSlotConfig::default(),
            sound_version: 0,

            background_choice: BackgroundChoice::BuiltIn,
            custom_bg_path: None,
            known_custom_backgrounds: load_known_custom_backgrounds(),

            font_choice: FontChoice::MorflashSerif,
            custom_font_path: None,
            known_custom_fonts: load_known_custom_fonts(),
        }
    }
}

/* ===== generic helpers for the three index files ===== */

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

fn save_index(path_str: &str, list: &[String]) {
    let path = Path::new(path_str);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let body = list.join("\n");
    let _ = fs::write(path, body);
}

fn load_known_custom_fonts() -> Vec<String> {
    load_index(CUSTOM_FONT_INDEX)
}
fn save_known_custom_fonts(list: &[String]) {
    save_index(CUSTOM_FONT_INDEX, list)
}

fn load_known_custom_sfx() -> Vec<String> {
    load_index(CUSTOM_SFX_INDEX)
}
fn save_known_custom_sfx(list: &[String]) {
    save_index(CUSTOM_SFX_INDEX, list)
}

fn load_known_custom_backgrounds() -> Vec<String> {
    load_index(CUSTOM_BG_INDEX)
}
fn save_known_custom_backgrounds(list: &[String]) {
    save_index(CUSTOM_BG_INDEX, list)
}

/* ===== shared file-import helper ===== */

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

/* ===== tiny PC-98 square toggle ===== */

fn square_choice(ui: &mut egui::Ui, selected: bool, label: &str) -> bool {
    let mut clicked = false;

    ui.horizontal(|ui| {
        let size = egui::vec2(14.0, 14.0);
        let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());
        let painter = ui.painter_at(rect);

        let border = MenuTheme::BUTTON_OUTLINE;
        let bg_off = MenuTheme::PANEL_BG;
        let bg_on = Color32::from_rgba_unmultiplied(0, 200, 255, 40);

        painter.rect_filled(rect, 2.0, if selected { bg_on } else { bg_off });
        painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, border));

        if selected {
            let inner = rect.shrink(3.0);
            painter.rect_filled(inner, 1.0, Color32::from_rgb(0, 200, 255));
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

/* ===== sound slot UI ===== */

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
                matches!(slot.source, SoundSource::BuiltIn),
                "Built-in",
            ) {
                slot.source = SoundSource::BuiltIn;
                *sound_version = sound_version.wrapping_add(1);
            }

            if square_choice(
                ui,
                matches!(slot.source, SoundSource::Custom),
                "Custom",
            ) {
                slot.source = SoundSource::Custom;
                *sound_version = sound_version.wrapping_add(1);
            }
        });

        if matches!(slot.source, SoundSource::Custom) {
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
                if let Some(dest_str) =
                    copy_chosen_file("Sound", &["wav", "ogg"], "assets/sfx", "custom_sfx.ogg")
                {
                    if !known_custom_sfx.iter().any(|p| p == &dest_str) {
                        known_custom_sfx.push(dest_str.clone());
                        save_known_custom_sfx(known_custom_sfx);
                    }

                    slot.source = SoundSource::Custom;
                    slot.custom_path = Some(dest_str);
                    *sound_version = sound_version.wrapping_add(1);
                }
            }
        }
    });
}

/* ===== MorButton wrapper ===== */

fn mor_button(
    ui: &mut egui::Ui,
    label: &str,
    min_width: f32,
    tex_opt: Option<&TextureHandle>,
) -> egui::Response {
    if tex_opt.is_none() {
        return ui.add(
            egui::Button::new(label).min_size(egui::vec2(min_width, 36.0)),
        );
    }

    let tex = tex_opt.unwrap();
    let font_id = FontId::proportional(20.0);

    let galley = ui.fonts(|f| {
        f.layout_no_wrap(label.to_owned(), font_id, Color32::WHITE)
    });
    let sz = galley.size();
    let padding = egui::vec2(20.0, 6.0);

    let mut desired = sz + padding * 2.0;
    desired.x = desired.x.max(min_width);

    let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
    let painter = ui.painter();

    painter.image(
        tex.id(),
        rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );

    let text_pos = rect.center() - sz * 0.5;
    painter.galley(text_pos, galley, Color32::WHITE);

    response
}

/* ===== main Options UI ===== */

/// Draw the options screen. Returns `true` if user pressed "Back".
pub fn draw_options(
    ui: &mut egui::Ui,
    state: &mut OptionsState,
    mor_button_tex: Option<&TextureHandle>,
) -> bool {
    let mut back = false;

    MenuTheme::apply_to_ctx(ui.ctx());

    let avail = ui.available_size();
    let panel_width = (avail.x * 0.7).clamp(600.0, 900.0);

    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading("Options");
        ui.add_space(20.0);

        egui::Frame::none()
            .fill(MenuTheme::PANEL_BG)
            .stroke(egui::Stroke::new(1.5, MenuTheme::BUTTON_OUTLINE))
            .rounding(egui::Rounding::same(18.0))
            .inner_margin(egui::Margin::symmetric(32.0, 24.0))
            .show(ui, |ui| {
                ui.set_width(panel_width);
                ui.vertical_centered(|ui| {
                    // AUDIO
                    ui.label("Audio");
                    if square_choice(ui, state.sound_enabled, "Enable sound effects") {
                        state.sound_enabled = !state.sound_enabled;
                        state.sound_version = state.sound_version.wrapping_add(1);
                    }

                    ui.add_space(8.0);
                    for (label, slot) in [
                        ("Correct answer sound", &mut state.sound_correct),
                        ("Incorrect answer sound", &mut state.sound_incorrect),
                        (
                            "Completion sound (when set is finished)",
                            &mut state.sound_complete,
                        ),
                    ] {
                        draw_sound_slot(
                            ui,
                            label,
                            slot,
                            &mut state.known_custom_sfx,
                            &mut state.sound_version,
                        );
                    }

                    ui.add_space(20.0);

                    // BACKGROUND
                    ui.label("Background");
                    ui.horizontal(|ui| {
                        if square_choice(
                            ui,
                            matches!(state.background_choice, BackgroundChoice::BuiltIn),
                            "Built-in paper texture",
                        ) {
                            state.background_choice = BackgroundChoice::BuiltIn;
                        }
                        if square_choice(
                            ui,
                            matches!(state.background_choice, BackgroundChoice::Custom),
                            "Custom background",
                        ) {
                            state.background_choice = BackgroundChoice::Custom;
                        }
                    });

                    if matches!(state.background_choice, BackgroundChoice::Custom) {
                        ui.add_space(8.0);

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

                            if square_choice(ui, is_current, name) {
                                state.custom_bg_path = Some(path.clone());
                            }
                        }

                        if ui.button("Import background…").clicked() {
                            if let Some(dest_str) = copy_chosen_file(
                                "Images",
                                &["png", "jpg", "jpeg"],
                                "assets/backgrounds",
                                "custom_background.png",
                            ) {
                                if !state
                                    .known_custom_backgrounds
                                    .iter()
                                    .any(|p| p == &dest_str)
                                {
                                    state.known_custom_backgrounds.push(dest_str.clone());
                                    save_known_custom_backgrounds(
                                        &state.known_custom_backgrounds,
                                    );
                                }

                                state.background_choice = BackgroundChoice::Custom;
                                state.custom_bg_path = Some(dest_str);
                            }
                        }

                        ui.label("Tip: use a seamless / tiling image for best results.");
                    }

                    ui.add_space(20.0);

                    // FONT
                    ui.label("Font");
                    for (variant, label) in [
                        (FontChoice::MorflashSerif, "MorFlash serif (Cormorant)"),
                        (FontChoice::Pixel, "Pixel font (PublicPixel)"),
                        (FontChoice::System, "System / default font"),
                        (FontChoice::Custom, "Custom font (file)"),
                    ] {
                        let selected = state.font_choice == variant;
                        if square_choice(ui, selected, label) {
                            state.font_choice = variant;
                        }
                    }

                    if matches!(state.font_choice, FontChoice::Custom) {
                        ui.add_space(8.0);

                        let path_buf = state.custom_font_path.get_or_insert_with(String::new);

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

                                    if !state.known_custom_fonts.iter().any(|p| p == &dest_str)
                                    {
                                        state.known_custom_fonts.push(dest_str);
                                        save_known_custom_fonts(&state.known_custom_fonts);
                                    }

                                    state.font_choice = FontChoice::Custom;
                                }
                            }
                        });

                        ui.label("Choose a .ttf or .otf font file.");

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
                                    .unwrap_or(false);

                                if square_choice(ui, is_current, name) {
                                    state.custom_font_path = Some(font_path.clone());
                                    state.font_choice = FontChoice::Custom;
                                }
                            }
                        }
                    }
                });
            });

        ui.add_space(24.0);
        if mor_button(ui, "⬛ Back", 160.0, mor_button_tex).clicked() {
            back = true;
        }
    });

    back
}
