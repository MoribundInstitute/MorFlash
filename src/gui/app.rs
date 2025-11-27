// src/gui/app.rs
use eframe::{egui, App};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::gui::sound::play_sound;
use crate::import;
use crate::model::{Card, Deck, ReviewState};
use crate::srs::{is_due, update_review_state};

use super::{deck_list_screen, study_screen};

#[derive(PartialEq)]
enum Screen {
    DeckList,
    Study,
}

pub struct MorflashGui {
    screen: Screen,
    deck_paths: Vec<PathBuf>,
    selected_deck_name: Option<String>,

    cards: Vec<Card>,
    states: HashMap<u64, ReviewState>,
    current_card_id: Option<u64>,
    options: Vec<Card>,
    feedback: String,

    // highlight info
    last_answer_correct: Option<bool>,
    correct_term: Option<String>,
    wrong_term: Option<String>,

    // progress
    total_cards: usize,
    reviewed_count: usize,

    // for auto-advance after a short delay
    pending_advance: bool,
    last_answer_time: Option<chrono::DateTime<chrono::Utc>>,
}

impl MorflashGui {
    /// Create a new GUI app, loading all decks from ./decks
    pub fn new() -> Self {
        let deck_paths = load_all_deck_paths("decks").unwrap_or_default();

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
        }
    }

    fn refresh_decks(&mut self) {
        self.deck_paths = load_all_deck_paths("decks").unwrap_or_default();
    }

    fn load_deck(&mut self, path: &Path) {
        if let Ok(deck) = Deck::from_json_file(path) {
            let cards = deck.cards;
            let now = chrono::Utc::now();
            let mut state_map = HashMap::new();
            for card in &cards {
                state_map.insert(card.id, ReviewState::new(card.id, now));
            }

            self.selected_deck_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .map(|s| s.to_string());

            self.cards = cards;
            self.states = state_map;
            self.feedback.clear();
            self.current_card_id = None;
            self.options.clear();
            self.last_answer_correct = None;
            self.correct_term = None;
            self.wrong_term = None;
            self.pending_advance = false;
            self.last_answer_time = None;

            self.total_cards = self.cards.len();
            self.reviewed_count = 0;

            self.screen = Screen::Study;
            self.pick_next_card(now);
        }
    }

    fn import_deck(&mut self) {
        // 1. file picker allowing several deck-like formats
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Deck files", &["json", "csv", "txt", "md", "markdown", "xml"])
            .pick_file()
        {
            let decks_dir = Path::new("decks");
            if let Err(e) = fs::create_dir_all(decks_dir) {
                eprintln!("Failed to create decks dir: {e}");
                return;
            }

            // 2. turn file into a Deck using central importer
            match import::import_deck_file(&path) {
                Ok(deck) => {
                    // 3. safe filename
                    let safe_name = deck.name.replace('/', "_");
                    let dest = decks_dir.join(format!("{safe_name}.json"));

                    // 4. save canonical JSON into ./decks
                    if let Err(e) =
                        fs::write(&dest, serde_json::to_string_pretty(&deck).unwrap())
                    {
                        eprintln!("Failed to write deck JSON: {e}");
                    } else {
                        // 5. refresh deck list
                        self.refresh_decks();
                    }
                }
                Err(e) => {
                    eprintln!("Failed to import deck: {e}");
                }
            }
        }
    }

    fn pick_next_card(&mut self, now: chrono::DateTime<chrono::Utc>) {
        if let Some(card) = self
            .cards
            .iter()
            .find(|c| is_due(self.states.get(&c.id).unwrap(), now))
        {
            self.current_card_id = Some(card.id);
            self.feedback.clear();
            self.last_answer_correct = None;
            self.correct_term = None;
            self.wrong_term = None;
            self.build_options(card.id);
        } else {
            self.current_card_id = None;
            self.options.clear();
            self.feedback = "No more cards due right now. 🎉".to_string();
        }
    }

    fn build_options(&mut self, correct_id: u64) {
        let mut rng = thread_rng();
        let mut opts: Vec<Card> = Vec::new();

        if let Some(c) = self.cards.iter().find(|c| c.id == correct_id) {
            opts.push(c.clone());
        }

        let mut others: Vec<Card> = self
            .cards
            .iter()
            .filter(|c| c.id != correct_id)
            .cloned()
            .collect();

        others.shuffle(&mut rng);
        for c in others.into_iter().take(3) {
            opts.push(c);
        }

        opts.shuffle(&mut rng);
        self.options = opts;
    }

    fn handle_answer(&mut self, chosen_term: &str) {
        if let Some(card_id) = self.current_card_id {
            let card = self.cards.iter().find(|c| c.id == card_id).unwrap();

            let now = chrono::Utc::now();
            let state = self.states.get(&card.id).unwrap().clone();

            let is_correct = chosen_term == card.term;

            // update feedback + glow info
            if is_correct {
                self.feedback = "✓ Correct!".to_string();
                self.last_answer_correct = Some(true);
                self.correct_term = Some(card.term.clone());
                self.wrong_term = None;
                play_sound("assets/sfx/Correct-Tone.wav");
            } else {
                self.feedback = format!("✗ Wrong. Correct answer: {}", card.term);
                self.last_answer_correct = Some(false);
                self.correct_term = Some(card.term.clone());
                self.wrong_term = Some(chosen_term.to_string());
                play_sound("assets/sfx/Incorrect-Tone.wav");
            }

            // update SRS state
            let rating = if is_correct { 3 } else { 0 };
            let new_state = update_review_state(state, rating, now);
            self.states.insert(card.id, new_state);

            // progress
            if self.total_cards > 0 {
                self.reviewed_count =
                    (self.reviewed_count + 1).min(self.total_cards);
            }

            // schedule auto-advance (short delay so glow is visible)
            self.pending_advance = true;
            self.last_answer_time = Some(now);
        }
    }
}

impl App for MorflashGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // handle auto-advance if we’re waiting
        if self.pending_advance {
            if let Some(t) = self.last_answer_time {
                let now = chrono::Utc::now();
                if now - t > chrono::Duration::milliseconds(700) {
                    self.pending_advance = false;
                    self.pick_next_card(now);
                }
            } else {
                // safety fallback
                let now = chrono::Utc::now();
                self.pending_advance = false;
                self.pick_next_card(now);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // dark outer background
            ui.painter().rect_filled(
                ui.max_rect(),
                0.0,
                egui::Color32::from_rgb(8, 8, 24),
            );

            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(24.0);

                match self.screen {
                    Screen::DeckList => {
                        let clicked_deck =
                            deck_list_screen::draw_deck_list(ui, &self.deck_paths);

                        if let Some(path) = clicked_deck {
                            self.load_deck(&path);
                        }

                        ui.add_space(20.0);
                        if ui.button("Import deck file...").clicked() {
                            self.import_deck();
                        }
                    }

                    Screen::Study => {
                        let current_card = self
                            .current_card_id
                            .and_then(|id| self.cards.iter().find(|c| c.id == id));

                        // which terms to highlight
                        let correct_term = self.correct_term.as_deref();
                        let wrong_term = self.wrong_term.as_deref();

                        let progress = if self.total_cards == 0 {
                            0.0
                        } else {
                            self.reviewed_count as f32 / self.total_cards as f32
                        };

                        let (clicked_term, back_to_list) =
                            study_screen::draw_study_screen(
                                ui,
                                current_card,
                                &self.options,
                                correct_term,
                                wrong_term,
                                &self.feedback,
                                progress,
                                self.reviewed_count,
                                self.total_cards,
                            );

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
                            // don’t accept another answer while waiting to advance
                            if !self.pending_advance {
                                self.handle_answer(&term);
                            }
                        }
                    }
                }
            });
        });
    }
}

fn load_all_deck_paths(dir: &str) -> anyhow::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    let base = Path::new(dir);

    if !base.exists() {
        return Ok(out);
    }

    for entry in fs::read_dir(base)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "json" {
                    out.push(path);
                }
            }
        }
    }

    Ok(out)
}
