// src/gui/app/mod.rs
use eframe::{
    egui::{self, ColorImage, TextureHandle, TextureOptions},
    App,
};
use std::{collections::HashMap, path::PathBuf};

mod deck_ops;
mod review_ops;
pub mod screens;

use screens::{main_menu_screen, options_screen, study_screen};

use crate::gui::{sound::SoundManager, theme::Theme};
use crate::model::{Card, ReviewState};

#[derive(PartialEq)]
pub enum Screen {
    DeckList,
    Study,
    Options,
}

pub struct MorflashGui {
    // Navigation / main menu
    pub(crate) screen: Screen,
    pub(crate) critter_tex: Option<TextureHandle>,
    pub(crate) container_tex: Option<TextureHandle>,
    pub main_menu_focus: usize,

    // Decks
    pub(crate) deck_paths: Vec<PathBuf>,
    pub(crate) selected_deck_name: Option<String>,

    // SRS state
    pub(crate) cards: Vec<Card>,
    pub(crate) states: HashMap<u64, ReviewState>,
    pub(crate) current_card_id: Option<u64>,

    // Multiple choice options & feedback
    pub(crate) options: Vec<Card>,
    pub(crate) feedback: String,
    pub(crate) last_answer_correct: Option<bool>,
    pub(crate) correct_term: Option<String>,
    pub(crate) wrong_term: Option<String>,

    // Progress / auto-advance
    pub(crate) total_cards: usize,
    pub(crate) reviewed_count: usize,
    pub(crate) pending_advance: bool,
    pub(crate) last_answer_time: Option<chrono::DateTime<chrono::Utc>>,

    // Visuals (tiled PC-98 background + zoom)
    bg_texture: Option<TextureHandle>,
    last_bg_key: Option<String>,
    pub(crate) zoom: f32,

    // Options + sound
    pub options_state: options_screen::OptionsState,
    pub(crate) sound: Option<SoundManager>,
    last_applied_sound_version: u64,
    celebration_played: bool,

    // UI textures
    pub(crate) mor_button_tex: Option<TextureHandle>,
}

impl MorflashGui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let deck_paths = Self::load_all_deck_paths("decks").unwrap_or_default();
        let options_state = options_screen::OptionsState::default();
        let sound = SoundManager::new();

        let mut app = Self {
            screen: Screen::DeckList,
            critter_tex: None,
            container_tex: None,
            main_menu_focus: 0,

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

            bg_texture: None,
            last_bg_key: None,
            zoom: 1.0,

            options_state,
            sound,
            last_applied_sound_version: 0,
            celebration_played: false,

            mor_button_tex: None,
        };

        app.load_mor_button_texture(&cc.egui_ctx);
        app.load_critter_texture(&cc.egui_ctx);
        app.load_container_texture(&cc.egui_ctx);

        app.configure_sounds_from_options();
        app.last_applied_sound_version = app.options_state.sound_version;
        app
    }

    fn load_mor_button_texture(&mut self, ctx: &egui::Context) {
        use egui_extras::image::load_svg_bytes_with_size;

        let path = "assets/ui/buttons/MorButton.svg";
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(err) => {
                eprintln!("MorFlash: failed to read {path}: {err}");
                return;
            }
        };

        let base_size = egui::vec2(320.0, 64.0);

        match load_svg_bytes_with_size(&bytes, Some(base_size.into())) {
            Ok(color_image) => {
                self.mor_button_tex = Some(ctx.load_texture(
                    "morflash_morbutton",
                    color_image,
                    TextureOptions::LINEAR,
                ));
            }
            Err(err) => eprintln!("MorFlash: failed to decode {path}: {err}"),
        }
    }

    fn load_png_texture(
        ctx: &egui::Context,
        path: &str,
        id: &str,
    ) -> Option<TextureHandle> {
        let bytes = std::fs::read(path).ok()?;
        let img = image::load_from_memory(&bytes).ok()?.to_rgba8();
        let (w, h) = img.dimensions();
        let pixels = img.into_raw();
        let color_image =
            ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &pixels);
        Some(ctx.load_texture(id, color_image, TextureOptions::LINEAR))
    }

    fn load_critter_texture(&mut self, ctx: &egui::Context) {
        self.critter_tex =
            Self::load_png_texture(ctx, "assets/ui/mor_critter.png", "morflash_critter");
    }

    fn load_container_texture(&mut self, ctx: &egui::Context) {
        self.container_tex =
            Self::load_png_texture(ctx, "assets/ui/menu_container.png", "menu_container");
    }

    fn configure_sounds_from_options(&mut self) {
        let Some(sm) = self.sound.as_mut() else { return };

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

        let correct = resolve(
            &self.options_state.sound_correct,
            "assets/sfx/Correct-Tone-Default.ogg",
        );
        let incorrect = resolve(
            &self.options_state.sound_incorrect,
            "assets/sfx/Incorrect-Sound-Default.ogg",
        );

        let complete = match self.options_state.sound_complete.source {
            options_screen::SoundSource::BuiltIn => {
                Some(PathBuf::from("assets/sfx/Celebration-Noise-Default.ogg"))
            }
            options_screen::SoundSource::Custom => self
                .options_state
                .sound_complete
                .custom_path
                .as_ref()
                .map(PathBuf::from)
                .or_else(|| Some(PathBuf::from("assets/sfx/Celebration-Noise-Default.ogg"))),
        };

        sm.load_core_sounds(&correct, &incorrect, complete.as_deref());
    }

    fn ensure_background_texture(&mut self, ctx: &egui::Context) {
        use std::path::Path;

        let (bg_path, bg_key) = match (
            self.options_state.background_choice,
            self.options_state.custom_bg_path.as_ref(),
        ) {
            (options_screen::BackgroundChoice::Custom, Some(path)) => {
                (Path::new(path), path.clone())
            }
            _ => (
                Path::new("assets/backgrounds/Rectangular Cobble.png"),
                "builtin_bg".to_string(),
            ),
        };

        if self
            .last_bg_key
            .as_ref()
            .is_some_and(|prev| prev == &bg_key && self.bg_texture.is_some())
        {
            return;
        }

        if let Ok(bytes) = std::fs::read(bg_path) {
            if let Ok(img) = image::load_from_memory(&bytes) {
                let img = img.to_rgba8();
                let (w, h) = img.dimensions();
                let pixels = img.into_raw();
                let color_image =
                    ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &pixels);

                let tex = ctx.load_texture(
                    "bg_texture_dynamic",
                    color_image,
                    TextureOptions::LINEAR,
                );
                self.bg_texture = Some(tex);
                self.last_bg_key = Some(bg_key);
            }
        } else {
            eprintln!("MorFlash: failed to load background image at {:?}", bg_path);
        }
    }

    fn trigger_main_menu_enter(&mut self) {
        let deck_count = self.deck_paths.len().min(3);
        let options_index = deck_count;

        if deck_count == 0 {
            self.screen = Screen::Options;
            self.main_menu_focus = 0;
            return;
        }

        if self.main_menu_focus < deck_count {
            if let Some(path) = self.deck_paths.get(self.main_menu_focus).cloned() {
                self.celebration_played = false;
                self.load_deck(&path);
                self.screen = Screen::Study;
            }
        } else if self.main_menu_focus == options_index {
            self.screen = Screen::Options;
        }
    }

    // load_all_deck_paths, pick_next_card, handle_answer live in deck_ops / review_ops
}

impl App for MorflashGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Reconfigure sounds if options changed
        if self.options_state.sound_version != self.last_applied_sound_version {
            self.configure_sounds_from_options();
            self.last_applied_sound_version = self.options_state.sound_version;
        }

        // Zoom controls
        ctx.input(|i| {
            let step = 0.1;
            if i.key_pressed(egui::Key::Equals) {
                self.zoom = (self.zoom + step).min(2.5);
            }
            if i.key_pressed(egui::Key::Minus) {
                self.zoom = (self.zoom - step).max(0.5);
            }
            if i.key_pressed(egui::Key::Num0) {
                self.zoom = 1.0;
            }
        });
        ctx.set_pixels_per_point(self.zoom);

        // Keyboard navigation for main menu
        if self.screen == Screen::DeckList {
            let deck_count = self.deck_paths.len().min(3);
            let max_index = (deck_count + 1).max(1);

            ctx.input(|i| {
                if i.key_pressed(egui::Key::ArrowUp) {
                    self.main_menu_focus = self.main_menu_focus.saturating_sub(1);
                }
                if i.key_pressed(egui::Key::ArrowDown) {
                    self.main_menu_focus = (self.main_menu_focus + 1).min(max_index - 1);
                }
                if i.key_pressed(egui::Key::Enter) {
                    self.trigger_main_menu_enter();
                }
            });
        }

        // Fonts + visuals
        Theme::apply_to_ctx(
            ctx,
            self.options_state.font_choice,
            self.options_state.custom_font_path.as_deref(),
        );

        // Background tiling
        self.ensure_background_texture(ctx);
        if let Some(bg) = &self.bg_texture {
            let painter = ctx.layer_painter(egui::LayerId::background());
            let screen = ctx.screen_rect();
            let tile = bg.size_vec2();

            let mut y = screen.top();
            while y < screen.bottom() {
                let mut x = screen.left();
                while x < screen.right() {
                    let rect = egui::Rect::from_min_size(egui::pos2(x, y), tile);
                    painter.image(
                        bg.id(),
                        rect,
                        egui::Rect::from_min_max(
                            egui::pos2(0.0, 0.0),
                            egui::pos2(1.0, 1.0),
                        ),
                        egui::Color32::WHITE,
                    );
                    x += tile.x;
                }
                y += tile.y;
            }
        }

        // Auto-advance
        if self.pending_advance {
            if let Some(t) = self.last_answer_time {
                let now = chrono::Utc::now();
                if now - t > chrono::Duration::milliseconds(700) {
                    self.pending_advance = false;
                    self.pick_next_card(now);
                }
            }
        }

        // Main UI
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| {
                match self.screen {
                    Screen::DeckList => {
                        match main_menu_screen::draw_main_menu(
                            ui,
                            &self.deck_paths,
                            self.main_menu_focus,
                            self.mor_button_tex.as_ref(),
                            self.critter_tex.as_ref(),
                        ) {
                            main_menu_screen::MainMenuAction::OpenDeck(path) => {
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

                    Screen::Options => {
                        if options_screen::draw_options(
                            ui,
                            &mut self.options_state,
                            self.mor_button_tex.as_ref(),
                        ) {
                            self.screen = Screen::DeckList;
                            self.main_menu_focus = 0;
                        }
                    }

                    Screen::Study => {
                        let current_card = self
                            .current_card_id
                            .and_then(|id| self.cards.iter().find(|c| c.id == id));

                        // Celebration sound when set is finished
                        if current_card.is_none()
                            && self.total_cards > 0
                            && !self.celebration_played
                        {
                            if let Some(ref sm) = self.sound {
                                if self.options_state.sound_enabled {
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
                                            sm.play(if correct { "correct" } else { "wrong" });
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
