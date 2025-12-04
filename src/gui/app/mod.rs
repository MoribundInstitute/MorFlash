// src/gui/app/mod.rs
use eframe::{
    egui::{self, ColorImage, TextureHandle, TextureOptions},
    App,
};
use rfd::FileDialog;
use std::time::Instant;
use std::{collections::HashMap, path::PathBuf};
mod deck_ops;
pub mod screens;

use screens::{
    completion_screen, deck_builder_screen, main_menu_screen, options_screen, study_screen,
};

use crate::gui::{sound::SoundManager, theme:: Theme};
use crate::model::{Card, ReviewState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenMode {
    Wide,
    Medium,
    Narrow,
    UltraNarrow,
}

#[derive(PartialEq, Debug)]
pub enum Screen {
    MainMenu,
    DeckList,
    Study,
    Options,
    Completion,
    DeckBuilder,
}

// Small toast-style notification used for save status, etc.
struct SaveNotice {
    message: String,
    is_error: bool,
    created_at: Instant,
}

pub struct MorflashGui {
    // Navigation / main menu
    pub(crate) screen: Screen,
    pub(crate) critter_tex: Option<TextureHandle>,
    pub(crate) container_tex: Option<TextureHandle>,
    pub main_menu_focus: usize,
    pub last_main_menu_focus: usize,

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
    pub(crate) pending_advance: bool, // FIXED: comma, not semicolon
    pub(crate) last_answer_time: Option<chrono::DateTime<chrono::Utc>>,

    // Visuals (tiled PC-98 background + zoom)
    bg_texture: Option<TextureHandle>,
    last_bg_key: Option<String>,
    pub(crate) zoom: f32,

    // Responsive UI mode (wide/medium/narrow/tiny)
    pub screen_mode: ScreenMode,

    // Options + sound
    pub options_state: options_screen::OptionsState,
    pub(crate) sound: Option<SoundManager>,
    pub(crate) last_applied_sound_version: u64,
    pub(crate) celebration_played: bool,

    // Small notification ("Saved deck", errors, etc.)
    pub(crate) save_notice: Option<SaveNotice>,

    // UI textures
    pub(crate) mor_button_tex: Option<TextureHandle>,

    // Screen-specific state
    pub(crate) deck_builder_state: deck_builder_screen::DeckBuilderState,
    pub(crate) completion_state: completion_screen::CompletionState,
}

// =======================================
// Core construction + high-level helpers
// =======================================
impl MorflashGui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let deck_paths = Self::load_all_deck_paths("decks").unwrap_or_default();
        let options_state = options_screen::OptionsState::default();
        let sound = SoundManager::new(); // ✅ no Some(...)


      let mut app = Self {
    // navigation
    screen: Screen::DeckList,
    critter_tex: None,
    container_tex: None,
    main_menu_focus: 0,
    last_main_menu_focus: 0,

    // decks
    deck_paths,
    selected_deck_name: None,

    // SRS
    cards: Vec::new(),
    states: HashMap::new(),
    current_card_id: None,

    // multiple choice
    options: Vec::new(),
    feedback: String::new(),
    last_answer_correct: None,
    correct_term: None,
    wrong_term: None,

    // progress / auto-advance
    total_cards: 0,
    reviewed_count: 0,
    pending_advance: false,
    last_answer_time: None,

    // visuals
    bg_texture: None,
    last_bg_key: None,
    zoom: 1.0,
    screen_mode: ScreenMode::Wide,

    // options + sound
    options_state,
    sound,
    last_applied_sound_version: 0,
    celebration_played: false,

    // transient UI notification ("Saved deck", errors, etc.)
    save_notice: None,

    // textures
    mor_button_tex: None,

    // screen-specific state
    deck_builder_state: deck_builder_screen::DeckBuilderState::default(),
    completion_state: completion_screen::CompletionState::default(),
};

        app.load_mor_button_texture(&cc.egui_ctx);
        app.load_critter_texture(&cc.egui_ctx);
        app.load_container_texture(&cc.egui_ctx);

        app.configure_sounds_from_options();
        app.last_applied_sound_version = app.options_state.global.sound_version;
        app
    }

    /// Play a navigation sound when the main menu focus changes.
fn play_main_menu_nav_sound(&self) {
    if let Some(ref sm) = self.sound {
        if self.options_state.global.sound_enabled {
            // Use the dedicated UI select sound instead of the quiz "correct" sound.
            sm.play("ui_select");
        }
    }
}

    fn trigger_main_menu_enter(&mut self) {
        match self.main_menu_focus {
            // 0: Choose Deck – open file picker
            0 => {
                if let Some(path) = FileDialog::new()
                    .add_filter("MorFlash decks", &["json", "mflash"])
                    .set_directory("decks")
                    .pick_file()
                {
                    self.celebration_played = false;
                    self.load_deck(path.as_path());
                    self.screen = Screen::Study;
                }
            }

            // 1: Deck Builder
            1 => {
                self.screen = Screen::DeckBuilder;
                self.main_menu_focus = 0;
                self.last_main_menu_focus = 0;
            }

            // 2: Options (and any other index)
            _ => {
                self.screen = Screen::Options;
                self.main_menu_focus = 0;
                self.last_main_menu_focus = 0;
            }
        }
    }
}
// =====================
// Textures / assets
// =====================
impl MorflashGui {
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

    fn load_png_texture(ctx: &egui::Context, path: &str, id: &str) -> Option<TextureHandle> {
        let bytes = std::fs::read(path).ok()?;
        let img = image::load_from_memory(&bytes).ok()?.to_rgba8();
        let (w, h) = img.dimensions();
        let pixels = img.into_raw();
        let color_image = ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &pixels);
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

   fn ensure_background_texture(&mut self, ctx: &egui::Context) {
    use std::path::Path;

    let (bg_path, bg_key) = match (
        self.options_state.global.background_choice,
        self.options_state.global.custom_bg_path.as_ref(),
    ) {
        (options_screen::BackgroundChoice::Custom, Some(path)) => {
            (Path::new(path.as_str()), path.clone())
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

                let tex =
                    ctx.load_texture("bg_texture_dynamic", color_image, TextureOptions::LINEAR);
                self.bg_texture = Some(tex);
                self.last_bg_key = Some(bg_key);
            }
        } else {
            eprintln!("MorFlash: failed to load background image at {:?}", bg_path);
        }
    }
}
// =====================
// Sound / options
// =====================
impl MorflashGui {
    fn configure_sounds_from_options(&mut self) {
        use std::path::PathBuf;

        let Some(sm) = self.sound.as_mut() else {
            return;
        };

        sm.set_enabled(self.options_state.global.sound_enabled);

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

        // Quiz sounds
        let correct = resolve(
            &self.options_state.global.sound_correct,
            "assets/sfx/Correct-Tone-Default.ogg",
        );

        let incorrect = resolve(
            &self.options_state.global.sound_incorrect,
            "assets/sfx/Incorrect-Sound-Default.ogg",
        );

        // Completion sound
        let complete = match self.options_state.global.sound_complete.source {
            options_screen::SoundSource::BuiltIn => {
                Some(PathBuf::from("assets/sfx/Celebration-Noise-Default.ogg"))
            }
            options_screen::SoundSource::Custom => self
                .options_state
                .global
                .sound_complete
                .custom_path
                .as_ref()
                .map(PathBuf::from)
                .or_else(|| Some(PathBuf::from("assets/sfx/Celebration-Noise-Default.ogg"))),
        };

        // UI select sound (main menu nav, etc.)
        let ui_select = resolve(
            &self.options_state.global.sound_ui_select,
            "assets/sfx/ui_select.ogg",
        );

        // ✅ Matches SoundManager::load_core_sounds(correct, incorrect, complete, ui_select)
        sm.load_core_sounds(&correct, &incorrect, complete.as_deref(), &ui_select);
    }

    fn hot_reload_sound(&mut self) {
        if self.options_state.global.sound_version != self.last_applied_sound_version {
            self.configure_sounds_from_options();
            self.last_applied_sound_version = self.options_state.global.sound_version;
        }
    }
}

// =====================
// Layout / controls
// =====================
impl MorflashGui {
    fn update_screen_mode(&mut self, ctx: &egui::Context) {
        let width = ctx.input(|i| i.screen_rect.width());

        self.screen_mode = if width >= 800.0 {
            ScreenMode::Wide
        } else if width >= 500.0 {
            ScreenMode::Medium
        } else if width >= 350.0 {
            ScreenMode::Narrow
        } else {
            ScreenMode::UltraNarrow
        };
    }

    fn handle_zoom_controls(&mut self, ctx: &egui::Context) {
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
    }

    fn handle_main_menu_keyboard_nav(&mut self, ctx: &egui::Context) {
        // Only on main menu / deck list screens
        if self.screen != Screen::DeckList && self.screen != Screen::MainMenu {
            return;
        }

        // We have 3 items: 0 = Choose Deck, 1 = Deck Builder, 2 = Options
        const MENU_ITEMS: usize = 3;
        let max_index = MENU_ITEMS.saturating_sub(1);

        ctx.input(|i| {
            // Keyboard up/down
            if i.key_pressed(egui::Key::ArrowUp) {
                self.main_menu_focus = self.main_menu_focus.saturating_sub(1);
            }
            if i.key_pressed(egui::Key::ArrowDown) {
                self.main_menu_focus = (self.main_menu_focus + 1).min(max_index);
            }

            // Scroll wheel: scroll up = move up, scroll down = move down
            let scroll = i.raw_scroll_delta.y;
            if scroll > 0.0 {
                // wheel up
                self.main_menu_focus = self.main_menu_focus.saturating_sub(1);
            } else if scroll < 0.0 {
                // wheel down
                self.main_menu_focus = (self.main_menu_focus + 1).min(max_index);
            }

            // Enter = activate current choice
            if i.key_pressed(egui::Key::Enter) {
                self.trigger_main_menu_enter();
            }
        });

        // If focus changed this frame, play nav sound once
        if self.main_menu_focus != self.last_main_menu_focus {
            self.play_main_menu_nav_sound();
            self.last_main_menu_focus = self.main_menu_focus;
        }
    }

    fn apply_global_theme(&mut self, ctx: &egui::Context) {
    Theme::apply_to_ctx(
        ctx,
        self.options_state.global.font_choice,
        self.options_state.global.custom_font_path.as_deref(),
    );
}

    fn draw_tiled_background(&mut self, ctx: &egui::Context) {
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
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                    x += tile.x;
                }
                y += tile.y;
            }
        }
    }

    fn handle_auto_advance(&mut self) {
        if !self.pending_advance {
            return;
        }

        if let Some(t) = self.last_answer_time {
            let now = chrono::Utc::now();
            if now - t > chrono::Duration::milliseconds(700) {
                self.pending_advance = false;
                self.pick_next_card(now);
            }
        }
    }
}
impl MorflashGui {
    fn show_save_notice(&mut self, ctx: &egui::Context) {
        use std::time::Duration;

        let Some(notice) = &self.save_notice else {
            return;
        };

        // Auto-hide after 3 seconds
        if notice.created_at.elapsed() > Duration::from_secs(3) {
            self.save_notice = None;
            return;
        }

        egui::Area::new("save_notice".into())
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-16.0, 16.0))
            .show(ctx, |ui| {
                let bg = if notice.is_error {
                    egui::Color32::from_rgb(120, 30, 30)
                } else {
                    egui::Color32::from_rgb(30, 140, 80)
                };

                let text_color = egui::Color32::WHITE;

                egui::Frame::none()
                    .fill(bg)
                    .rounding(egui::Rounding::same(10.0))
                    .inner_margin(egui::Margin::symmetric(12.0, 8.0))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&notice.message)
                                .color(text_color)
                                .strong(),
                        );
                    });
            });
    }
}

// =====================
// Main UI dispatcher
// =====================
impl MorflashGui {
    fn draw_main_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| {
                match self.screen {
                    // =========================
                    // MAIN MENU / DECK LIST
                    // =========================
                    Screen::MainMenu | Screen::DeckList => {
                        match main_menu_screen::draw_main_menu(
                            ui,
                            self.main_menu_focus,
                            self.mor_button_tex.as_ref(),
                            self.critter_tex.as_ref(),
                            &self.options_state.main_menu,
                        ) {
                            main_menu_screen::MainMenuAction::ChooseDeck => {
                                // Prefer a local "decks" directory if it exists
                                let decks_dir = std::path::Path::new("decks");

                                let mut dialog = FileDialog::new()
                                    .add_filter("MorFlash decks", &["json", "mflash"]);

                                if decks_dir.exists() {
                                    dialog = dialog.set_directory(decks_dir);
                                }

                                if let Some(path) = dialog.pick_file() {
                                    self.celebration_played = false;
                                    self.load_deck(path.as_path());
                                    self.screen = Screen::Study;
                                }
                            }
                            main_menu_screen::MainMenuAction::OpenDeckBuilder => {
                                self.screen = Screen::DeckBuilder;
                                self.main_menu_focus = 0;
                                self.last_main_menu_focus = 0;
                            }
                            main_menu_screen::MainMenuAction::OpenOptions => {
                                self.screen = Screen::Options;
                                self.main_menu_focus = 0;
                                self.last_main_menu_focus = 0;
                            }
                            main_menu_screen::MainMenuAction::None => {}
                        }
                    }

                    // =========================
                    // OPTIONS
                    // =========================
                    Screen::Options => {
                        // Draw the options UI (mutates self.options_state in-place).
                        let save_and_exit = options_screen::draw_options(
                            ui,
                            &mut self.options_state,
                            self.mor_button_tex.as_ref(),
                        );

                        // Esc should also act like "Save & Exit".
                        let esc_pressed = ctx.input(|i| i.key_pressed(egui::Key::Escape));

                        if save_and_exit || esc_pressed {
                            // Go back to main menu.
                            self.screen = Screen::MainMenu;
                            self.main_menu_focus = 0;
                            self.last_main_menu_focus = 0;
                        }
                    }

                    // =========================
                    // STUDY
                    // =========================
                    Screen::Study => {
                        let current_card = self
                            .current_card_id
                            .and_then(|id| self.cards.iter().find(|c| c.id == id));

                        // Completion transition
                        if current_card.is_none()
                            && self.total_cards > 0
                            && !self.celebration_played
                        {
                            if let Some(ref sm) = self.sound {
                                if self.options_state.global.sound_enabled {
                                    sm.play("complete");
                                }
                            }
                            self.celebration_played = true;
                            self.screen = Screen::Completion;
                            return;
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

                        let card_fill = match self.options_state.study.card_color_mode {
                            options_screen::CardColorMode::BuiltIn => Theme::CARD_BG,
                            options_screen::CardColorMode::Custom => {
                                self.options_state.study.card_color
                            }
                        };

                        // ---- Window geometry: centered, good default size ----
                        let screen_rect = ctx.screen_rect();
                        let screen_w = screen_rect.width();
                        let screen_h = screen_rect.height();

                        // About 75% of the width, clamped to a nice range.
                        let default_w = (screen_w * 0.75).clamp(800.0, 1200.0);
                        // About 60% of the height, also clamped.
                        let default_h = (screen_h * 0.6).clamp(450.0, 800.0);

                        let default_rect = egui::Rect::from_center_size(
                            screen_rect.center(),
                            egui::vec2(default_w, default_h),
                        );

                        // Draggable, resizable study window that remembers size/position
                        egui::Window::new("StudyCard")
                            .title_bar(false)
                            .resizable(true)
                            .collapsible(false)
                            .movable(true)
                            .default_rect(default_rect)
                            .frame(
                                egui::Frame::none()
                                    .fill(card_fill)
                                    .rounding(egui::Rounding::same(Theme::CARD_ROUNDING))
                                    .inner_margin(egui::Margin::symmetric(
                                        Theme::CARD_MARGIN,
                                        Theme::CARD_MARGIN - 4.0,
                                    )),
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
                                    &self.options_state.study,
                                );

                                clicked_term = ct;
                                back_to_list = back;
                            });

                        // Back to deck list
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

                        // Handle answer click + sound
                        if let Some(term) = clicked_term {
                            if !self.pending_advance {
                                self.handle_answer(&term);
                                if let Some(ref sm) = self.sound {
                                    if self.options_state.global.sound_enabled {
                                        if let Some(correct) = self.last_answer_correct {
                                            sm.play(if correct { "correct" } else { "wrong" });
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // =========================
                    // COMPLETION
                    // =========================
                    Screen::Completion => {
                        let back_to_deck: bool = completion_screen::draw_completion_screen(
                            ui,
                            &mut self.completion_state,
                            &self.options_state.completion,
                            self.bg_texture.as_ref(),
                            || {
                                if let Some(sm) = self.sound.as_ref() {
                                    sm.play("finish"); // <- use the "finish" sound id
                                }
                            },
                        );

                        if back_to_deck {
                            self.screen = Screen::DeckList;
                            self.main_menu_focus = 0;
                            self.celebration_played = false;
                            self.completion_state.celebration_played = false;
                            self.current_card_id = None;
                            self.feedback.clear();
                            self.last_answer_correct = None;
                            self.correct_term = None;
                            self.wrong_term = None;
                            self.pending_advance = false;
                            self.last_answer_time = None;
                        }
                    }

                    // =========================
                    // DECK BUILDER
                    // =========================
                    Screen::DeckBuilder => {
                        let done = deck_builder_screen::draw_deck_builder_screen(
                            ctx,
                            &mut self.deck_builder_state,
                            &self.options_state.deck_builder,
                        );

                        if done {
                            match self.save_builder_state_as_deck() {
                                Err(err) => {
                                    eprintln!(
                                        "MorFlash: failed to save deck from builder: {err:?}"
                                    );
                                    self.save_notice = Some(SaveNotice {
                                        message: format!("Failed to save deck: {err}"),
                                        is_error: true,
                                        created_at: Instant::now(),
                                    });
                                    // Keep user on the builder screen so they can fix it.
                                }
                                Ok(path) => {
                                    self.save_notice = Some(SaveNotice {
                                        message: format!("Saved deck to {}", path.display()),
                                        is_error: false,
                                        created_at: Instant::now(),
                                    });
                                    // Successfully saved as .mflash; go back to deck list.
                                    self.screen = Screen::DeckList;
                                }
                            }
                        }
                    }
                }
            });

        // Draw any active save / error notice as a floating toast.
        self.show_save_notice(ctx);
    }
}

// =====================
// eframe::App impl
// =====================
impl App for MorflashGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.hot_reload_sound();
        self.update_screen_mode(ctx);
        self.handle_zoom_controls(ctx);
        self.handle_main_menu_keyboard_nav(ctx);
        self.apply_global_theme(ctx);
        self.draw_tiled_background(ctx);
        self.handle_auto_advance();
        self.draw_main_ui(ctx);
    }
}
