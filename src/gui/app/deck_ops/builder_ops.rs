// src/gui/app/deck_ops/builder_ops.rs

use std::path::{Path, PathBuf};

use crate::gui::app::screens::deck_builder_screen::DeckBuilderState;
use crate::gui::app::MorflashGui;
use crate::model::{Deck, Card};
use crate::srs::mflash::save_mflash_deck;

/// Glue between DeckBuilderState and real `.mflash` deck files.
///
/// This converts the in-memory builder state into a Deck,
/// saves it as a `.mflash` file under `decks/`, and refreshes
/// the deck list so it appears in the UI.
impl MorflashGui {
    /// Convert the current DeckBuilderState into a deck file under `decks/`
    /// and return the path to the saved `.mflash` file.
    pub(crate) fn save_builder_state_as_deck(&mut self) -> anyhow::Result<PathBuf> {
        // Borrow once immutably.
        let state: &DeckBuilderState = &self.deck_builder_state;

        // 1. Ensure `decks/` exists.
        let decks_dir = Path::new("decks");
        std::fs::create_dir_all(decks_dir)?;

        // 2. Derive a safe base name from the builder's file_name field.
        let raw = state.file_name.trim();
        let base = if raw.is_empty() { "new_deck" } else { raw };

        let safe: String = base
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        let path = decks_dir.join(format!("{safe}.mflash"));

        // 3. Convert DeckBuilderState -> Deck.
        let cards: Vec<Card> = state
            .cards
            .iter()
            .enumerate()
            .map(|(i, c)| Card {
                id: (i as u64) + 1,
                term: c.term.clone(),
                definition: c.definition.clone(),
            })
            .collect();

        let deck = Deck {
            name: base.to_string(),
            // For now: shove the tags string into description if present.
            description: if state.tags.trim().is_empty() {
                None
            } else {
                Some(state.tags.trim().to_string())
            },
            cards,
        };

        // 4. Save as .mflash using the helper.
        save_mflash_deck(&path, &deck)?;

        // 5. Refresh deck list so it appears in the UI.
        self.deck_paths = Self::load_all_deck_paths("decks").unwrap_or_default();

        Ok(path)
    }
}
