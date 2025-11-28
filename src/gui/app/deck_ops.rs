// src/gui/app/deck_ops.rs
use super::MorflashGui;
use crate::import;
use crate::model::Deck;
use std::fs;
use std::path::{Path, PathBuf};

impl MorflashGui {
    pub(crate) fn refresh_decks(&mut self) {
        self.deck_paths = Self::load_all_deck_paths("decks").unwrap_or_default();
    }

    pub(crate) fn load_deck(&mut self, path: &Path) {
        // ⬇ copy the EXACT body of your old `fn load_deck(&mut self, path: &Path)` here
        if let Ok(deck) = Deck::from_json_file(path) {
            let cards = deck.cards;
            let now = chrono::Utc::now();
            let mut state_map = std::collections::HashMap::new();
            for card in &cards {
                state_map.insert(card.id, crate::model::ReviewState::new(card.id, now));
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

            self.screen = super::Screen::Study;
            self.pick_next_card(now);
        }
    }

    pub(crate) fn import_deck(&mut self) {
        // ⬇ copy the EXACT body of your old `fn import_deck(&mut self)` here
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(
                "Deck files",
                &["json", "csv", "txt", "md", "markdown", "xml"],
            )
            .pick_file()
        {
            let decks_dir = Path::new("decks");
            if let Err(e) = fs::create_dir_all(decks_dir) {
                eprintln!("Failed to create decks dir: {e}");
                return;
            }

            match import::import_deck_file(&path) {
                Ok(deck) => {
                    let safe_name = deck.name.replace('/', "_");
                    let dest = decks_dir.join(format!("{safe_name}.json"));

                    if let Err(e) = fs::write(&dest, serde_json::to_string_pretty(&deck).unwrap()) {
                        eprintln!("Failed to write deck JSON: {e}");
                    } else {
                        self.refresh_decks();
                    }
                }
                Err(e) => {
                    eprintln!("Failed to import deck: {e}");
                }
            }
        }
    }

    pub(super) fn load_all_deck_paths(dir: &str) -> anyhow::Result<Vec<PathBuf>> {
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
