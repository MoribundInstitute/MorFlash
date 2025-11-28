use eframe::{egui, App};
use egui::{ColorImage, TextureOptions};
use std::collections::HashMap;
use std::path::PathBuf;

mod deck_ops;
mod review_ops;
mod screens;

use screens::{deck_list_screen, options_screen, study_screen};

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

    // ===== Global zoom factor (1.0 = 100%) =====
    pub(crate) zoom: f32,

    // ===== Options screen state =====
    pub options_state: options_screen::OptionsState,
}

impl MorflashGui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let deck_paths = MorflashGui::load_all_deck_paths("decks").unwrap_or_default();

        Self {
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

            bg_texture: None,
            zoom: 1.0,

            options_state: options_screen::OptionsState {
                sound_enabled: true,
                font_choice: options_screen::FontChoice::Pixel,
                custom_font_path: None,
            },
        }
    }

    /// Lazily load the background texture once.
    fn ensure_background_texture(&mut self, ctx: &egui::Context) {
        if self.bg_texture.is_some() {
            return;
        }

        let bytes = include_bytes!("../../../assets/bg_tile.png");

        if let Ok(img) = image::load_from_memory(bytes) {
            let img = img.to_rgba8();
            let (w, h) = img.dimensions();
            let pixels = img.into_raw();

            let color_image = ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &pixels);

            let tex = ctx.load_texture("bg_tile", color_image, TextureOptions::LINEAR);
            self.bg_texture = Some(tex);
        }
    }
}

// This is what eframe calls each frame.
impl App for MorflashGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ----- Zoom controls -----
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

        // Global visuals (colors + font) – driven by options_state.font_choice
        let use_pixel_font = matches!(
            self.options_state.font_choice,
            options_screen::FontChoice::Pixel
        );
        Theme::apply_to_ctx(ctx, use_pixel_font);

        // Ensure the background texture is loaded
        self.ensure_background_texture(ctx);

        // Draw tiled background
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

        // Auto-advance
        if self.pending_advance {
            if let Some(t) = self.last_answer_time {
                if chrono::Utc::now() - t > chrono::Duration::milliseconds(700) {
                    self.pending_advance = false;
                    self.pick_next_card(chrono::Utc::now());
                }
            }
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| match self.screen {
                Screen::DeckList => match deck_list_screen::draw_main_menu(ui, &self.deck_paths) {
                    deck_list_screen::MainMenuAction::OpenDeck(path) => {
                        self.load_deck(&path);
                    }
                    deck_list_screen::MainMenuAction::OpenOptions => {
                        self.screen = Screen::Options;
                    }
                    deck_list_screen::MainMenuAction::None => {}
                },

                Screen::Options => {
                    let back = options_screen::draw_options(ui, &mut self.options_state);
                    if back {
                        self.screen = Screen::DeckList;
                    }
                }

                Screen::Study => {
                    let current_card = self
                        .current_card_id
                        .and_then(|id| self.cards.iter().find(|c| c.id == id));

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
                    }

                    if let Some(term) = clicked_term {
                        if !self.pending_advance {
                            self.handle_answer(&term);
                        }
                    }
                }
            });
    }
}
