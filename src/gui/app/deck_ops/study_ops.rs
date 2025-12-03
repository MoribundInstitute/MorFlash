// src/gui/app/deck_ops/study_ops.rs

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;

use crate::gui::app::{MorflashGui, Screen};
use crate::model::{Deck, ReviewState};

impl MorflashGui {
    /// Rescan the `decks/` directory and refresh the in-memory list.
    pub(crate) fn refresh_decks(&mut self) {
        self.deck_paths = Self::load_all_deck_paths("decks").unwrap_or_default();
    }

    /// Load a deck file from disk and initialize SRS state for studying.
    pub(crate) fn load_deck(&mut self, path: &Path) {
        if let Ok(deck) = Deck::from_json_file(path) {
            let cards = deck.cards;
            let now = Utc::now();
            let mut state_map: HashMap<u64, ReviewState> = HashMap::new();

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

    /// Find all `.json` deck files in the given directory.
    pub(crate) fn load_all_deck_paths(dir: &str) -> anyhow::Result<Vec<PathBuf>> {
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
}
