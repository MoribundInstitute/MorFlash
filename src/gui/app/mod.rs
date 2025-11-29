use eframe::{egui, App};
use egui::{ColorImage, TextureOptions};
use std::collections::HashMap;
use std::path::PathBuf;

mod deck_ops;
mod review_ops;
pub mod screens;

use screens::{completion_screen, main_menu_screen, options_screen, study_screen};

use crate::gui::sound::SoundManager;
use crate::gui::theme::Theme;
use crate::model::{Card, ReviewState};

#[derive(PartialEq)]
pub enum Screen {
    DeckList,
    Study,
    Options,
}

pub struct MorflashGui {
    // ===== Navigation =====
    pub(crate) screen: Screen,

    // ===== Keyboard navigation for main menu =====
    pub main_menu_focus: usize,

    // ===== Decks =====
    pub(crate) deck_paths: Vec<PathBuf>,
    pub(crate) selected_deck_name: Option<String>,

    // ===== SRS state =====
    pub(crate) cards: Vec<Card>,
    pub(crate) states: HashMap<u64, ReviewState>,
    pub(crate) current_card_id: Option<u64>,

    // ===== Multiple choice options & feedback =====
    pub(crate) options: Vec<Card>,
    pub(crate) feedback: String,
    pub(crate) last_answer_correct: Option<bool>,
    pub(crate) correct_term: Option<String>,
    pub(crate) wrong_term: Option<String>,

    // ===== Progress =====
    pub(crate) total_cards: usize,
    pub(crate) reviewed_count: usize,

    // ===== Auto-advance delay =====
    pub(crate) pending_advance: bool,
    pub(crate) last_answer_time: Option<chrono::DateTime<chrono::Utc>>,

    // ===== Background texture (tiled PC-98 style) =====
    bg_texture: Option<egui::TextureHandle>,
    last_bg_key: Option<String>,

    // ===== Global zoom factor (1.0 = 100%) =====
    pub(crate) zoom: f32,

    // ===== Options screen state =====
    pub options_state: options_screen::OptionsState,

    // ===== Global sound manager =====
    pub(crate) sound: Option<SoundManager>,

    /// Which audio config we last applied (tracks OptionsState.sound_version).
    last_applied_sound_version: u64,

    /// Have we already played the celebration sound for this deck?
    celebration_played: bool,
}

impl MorflashGui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Uses the load_all_deck_paths method defined in deck_ops.rs (impl MorflashGui)
        let deck_paths = MorflashGui::load_all_deck_paths("decks").unwrap_or_default();

        // Default options: pulled from OptionsState::default() (includes sound slots).
        let options_state = options_screen::OptionsState::default();

        // Set up sound manager (may be None if audio device fails)
        let sound = SoundManager::new();

        let mut app = Self {
            screen: Screen::DeckList,
            deck_paths,
            selected_deck_name: None,

            cards: Vec::new(),
            states: HashMap::new(),
            current_card_id: None,
            options: Vec::new(),
            feedback: String::new(),
            last_answer_correct: None,
            correct_term: None,
            wrong_term: None,

            total_cards: 0,
            reviewed_count: 0,

            pending_advance: false,
            last_answer_time: None,

            // Background
            bg_texture: None,
            last_bg_key: None,
            main_menu_focus: 0,
            zoom: 1.0,

            options_state,
            sound,
            last_applied_sound_version: 0,
            celebration_played: false,
        };

        // Configure sounds once at startup according to default options.
        app.configure_sounds_from_options();
        app.last_applied_sound_version = app.options_state.sound_version;

        app
    }

    /// Reconfigure the SoundManager based on the current OptionsState
    /// (built-in vs custom correct/incorrect/complete sounds).
    fn configure_sounds_from_options(&mut self) {
        use std::path::PathBuf;

        let Some(sm) = self.sound.as_mut() else {
            return;
        };

        sm.set_enabled(self.options_state.sound_enabled);

        fn resolve(slot: &options_screen::SoundSlotConfig, built_in: &str) -> PathBuf {
            match slot.source {
                options_screen::SoundSource::BuiltIn => PathBuf::from(built_in),
                options_screen::SoundSource::Custom => slot
                    .custom_path
                    .as_ref()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from(built_in)),
            }
        }

        // Correct / incorrect paths (built-in or custom, with fallback)
        let correct = resolve(
            &self.options_state.sound_correct,
            "assets/sfx/Correct-Tone.wav",
        );
        let incorrect = resolve(
            &self.options_state.sound_incorrect,
            "assets/sfx/Incorrect-Tone.wav",
        );

        // === Celebration Sound (Built-in or Custom) ===
        let complete: Option<PathBuf> = match self.options_state.sound_complete.source {
            options_screen::SoundSource::BuiltIn => {
                // Built-in celebration noise stored with the other SFX:
                Some(PathBuf::from("assets/sfx/Celebration-Noise.wav"))
            }
            options_screen::SoundSource::Custom => {
                // If user provided a custom path, use it;
                // otherwise fall back to built-in celebration noise.
                self.options_state
                    .sound_complete
                    .custom_path
                    .as_ref()
                    .map(PathBuf::from)
                    .or_else(|| Some(PathBuf::from("assets/sfx/Celebration-Noise.wav")))
            }
        };

        // Finally load all three into the SoundManager.
        sm.load_core_sounds(&correct, &incorrect, complete.as_deref());
    }

    /// Load / reload the background texture based on OptionsState.
    /// Uses:
    ///   - Built-in:  "assets/bg_tile.png"
    ///   - Custom:    whatever `options_state.custom_bg_path` points to
    /// Caches via `last_bg_key` so we only reload when the choice changes.
    fn ensure_background_texture(&mut self, ctx: &egui::Context) {
        use std::path::Path;

        // Decide which file should be used right now.
        let (bg_path, bg_key) = match (
            self.options_state.background_choice,
            self.options_state.custom_bg_path.as_ref(),
        ) {
            (options_screen::BackgroundChoice::Custom, Some(path)) => {
                (Path::new(path), path.clone())
            }
            _ => {
                // Built-in default paper texture.
                (Path::new("assets/bg_tile.png"), "builtin_bg".to_string())
            }
        };

        // If the background selection hasn't changed and we already have a texture, do nothing.
        if let Some(prev) = &self.last_bg_key {
            if prev == &bg_key && self.bg_texture.is_some() {
                return;
            }
        }

        // Try to load the image from disk.
        match std::fs::read(bg_path) {
            Ok(bytes) => {
                if let Ok(img) = image::load_from_memory(&bytes) {
                    let img = img.to_rgba8();
                    let (w, h) = img.dimensions();
                    let pixels = img.into_raw();

                    let color_image =
                        ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &pixels);

                    let tex =
                        ctx.load_texture("bg_texture_dynamic", color_image, TextureOptions::LINEAR);
                    self.bg_texture = Some(tex);
                    self.last_bg_key = Some(bg_key);
                }
            }
            Err(e) => {
                eprintln!(
                    "MorFlash: failed to load background image at {:?}: {e}",
                    bg_path
                );
            }
        }
    }
    fn trigger_main_menu_enter(&mut self) {
        let deck_count = self.deck_paths.len().min(3);
        let options_index = deck_count; // the "Options" button slot

        // If no decks: Enter always goes to Options
        if deck_count == 0 {
            self.screen = Screen::Options;
            self.main_menu_focus = 0;
            return;
        }

        if self.main_menu_focus < deck_count {
            // Open selected deck
            if let Some(path) = self.deck_paths.get(self.main_menu_focus).cloned() {
                self.celebration_played = false;
                self.load_deck(&path);
                self.screen = Screen::Study;
            }
        } else if self.main_menu_focus == options_index {
            // Open options
            self.screen = Screen::Options;
        }
    }

    // NOTE: load_all_deck_paths, pick_next_card, and handle_answer
    // live in deck_ops.rs / review_ops.rs as `impl MorflashGui { ... }`.
}

/// This is what eframe calls each frame.
impl App for MorflashGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ===== Reconfigure sounds if audio options changed =====
        if self.options_state.sound_version != self.last_applied_sound_version {
            self.configure_sounds_from_options();
            self.last_applied_sound_version = self.options_state.sound_version;
        }

        // ===== Zoom controls =====
        let zoom_step = 0.1;
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Equals) {
                self.zoom = (self.zoom + zoom_step).min(2.5);
            }
            if i.key_pressed(egui::Key::Minus) {
                self.zoom = (self.zoom - zoom_step).max(0.5);
            }
            if i.key_pressed(egui::Key::Num0) {
                self.zoom = 1.0;
            }
        });
        ctx.set_pixels_per_point(self.zoom);

        // ===== Keyboard navigation: main menu (DeckList screen) =====
        if self.screen == Screen::DeckList {
            let deck_count = self.deck_paths.len().min(3);
            // number of selectable items = decks (0–3) + 1 "Options" button
            let mut max_index = deck_count + 1; // last index is always "Options"

            if max_index == 0 {
                // No decks at all — just treat it as 1 item ("Options").
                max_index = 1;
            }

            ctx.input(|i| {
                if i.key_pressed(egui::Key::ArrowUp) {
                    if self.main_menu_focus > 0 {
                        self.main_menu_focus -= 1;
                    }
                }

                if i.key_pressed(egui::Key::ArrowDown) {
                    if self.main_menu_focus + 1 < max_index {
                        self.main_menu_focus += 1;
                    }
                }

                if i.key_pressed(egui::Key::Enter) {
                    self.trigger_main_menu_enter();
                }
            });

            // Clamp in case deck list length changed.
            if self.main_menu_focus >= max_index {
                self.main_menu_focus = max_index.saturating_sub(1);
            }
        }

        // ===== Fonts + visuals (Cormorant / Pixel / System / Custom) =====
        Theme::apply_to_ctx(
            ctx,
            self.options_state.font_choice,
            self.options_state.custom_font_path.as_deref(),
        );

        // ===== Background texture =====
        self.ensure_background_texture(ctx);

        if let Some(bg) = &self.bg_texture {
            let painter = ctx.layer_painter(egui::LayerId::background());
            let screen_rect = ctx.screen_rect();
            let tile_size = bg.size_vec2();

            let mut y = screen_rect.top();
            while y < screen_rect.bottom() {
                let mut x = screen_rect.left();
                while x < screen_rect.right() {
                    let tile_rect = egui::Rect::from_min_size(egui::pos2(x, y), tile_size);

                    painter.image(
                        bg.id(),
                        tile_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );

                    x += tile_size.x;
                }
                y += tile_size.y;
            }
        }

        // ===== Auto-advance handling =====
        if self.pending_advance {
            if let Some(t) = self.last_answer_time {
                if chrono::Utc::now() - t > chrono::Duration::milliseconds(700) {
                    self.pending_advance = false;
                    self.pick_next_card(chrono::Utc::now());
                }
            }
        }

        // ===== Main UI root =====
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| {
                match self.screen {
                    // =======================
                    // MAIN MENU
                    // =======================
                    Screen::DeckList => {
                        match main_menu_screen::draw_main_menu(
                            ui,
                            &self.deck_paths,
                            self.main_menu_focus,
                        ) {
                            main_menu_screen::MainMenuAction::OpenDeck(path) => {
                                // Opening a new deck → allow a new celebration later.
                                self.celebration_played = false;
                                self.load_deck(&path);
                                self.screen = Screen::Study;
                            }
                            main_menu_screen::MainMenuAction::OpenOptions => {
                                self.screen = Screen::Options;
                            }
                            main_menu_screen::MainMenuAction::None => {}
                        }
                    }

                    // =======================
                    // OPTIONS
                    // =======================
                    Screen::Options => {
                        let back = options_screen::draw_options(ui, &mut self.options_state);
                        if back {
                            self.screen = Screen::DeckList;
                            self.main_menu_focus = 0; // reset focus to first item
                        }
                    }

                    // =======================
                    // STUDY
                    // =======================
                    Screen::Study => {
                        let current_card = self
                            .current_card_id
                            .and_then(|id| self.cards.iter().find(|c| c.id == id));

                        // === Celebration trigger: no more cards due ===
                        if current_card.is_none()
                            && self.total_cards > 0
                            && !self.celebration_played
                        {
                            if let Some(ref sm) = self.sound {
                                if self.options_state.sound_enabled {
                                    // must match id used in SoundManager::load_core_sounds
                                    sm.play("complete");
                                }
                            }
                            self.celebration_played = true;
                        }

                        let correct_term = self.correct_term.as_deref();
                        let wrong_term = self.wrong_term.as_deref();

                        let progress = if self.total_cards == 0 {
                            0.0
                        } else {
                            self.reviewed_count as f32 / self.total_cards as f32
                        };

                        let mut clicked_term: Option<String> = None;
                        let mut back_to_list = false;

                        egui::Window::new("study-card")
                            .title_bar(false)
                            .resizable(true)
                            .collapsible(false)
                            .constrain(true)
                            .default_size(egui::vec2(900.0, 500.0))
                            .frame(
                                egui::Frame::none()
                                    .fill(Theme::CARD_BG)
                                    .stroke(egui::Stroke::new(1.5, Theme::CARD_STROKE))
                                    .rounding(egui::Rounding::same(Theme::CARD_ROUNDING))
                                    .inner_margin(egui::Margin {
                                        top: 6.0,
                                        bottom: Theme::CARD_MARGIN,
                                        left: Theme::CARD_MARGIN,
                                        right: Theme::CARD_MARGIN,
                                    }),
                            )
                            .show(ctx, |ui_card| {
                                let (ct, back) = study_screen::draw_study_screen(
                                    ui_card,
                                    current_card,
                                    &self.options,
                                    correct_term,
                                    wrong_term,
                                    &self.feedback,
                                    progress,
                                    self.reviewed_count,
                                    self.total_cards,
                                );

                                clicked_term = ct;
                                back_to_list = back;
                            });

                        if back_to_list {
                            self.screen = Screen::DeckList;
                            self.current_card_id = None;
                            self.feedback.clear();
                            self.last_answer_correct = None;
                            self.correct_term = None;
                            self.wrong_term = None;
                            self.pending_advance = false;
                            self.last_answer_time = None;
                            self.celebration_played = false;
                            self.main_menu_focus = 0;
                        }

                        if let Some(term) = clicked_term {
                            if !self.pending_advance {
                                self.handle_answer(&term);

                                if let Some(ref sm) = self.sound {
                                    if self.options_state.sound_enabled {
                                        if let Some(correct) = self.last_answer_correct {
                                            if correct {
                                                sm.play("correct");
                                            } else {
                                                sm.play("wrong");
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
    }
}
