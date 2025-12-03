// src/gui/app/deck_ops/builder_ops.rs

use crate::gui::app::screens::deck_builder_screen::DeckBuilderState;
use crate::gui::app::MorflashGui;

/// Future: glue between DeckBuilderState and real `Deck` files.
///
/// Right now this is just a placeholder so the architecture is obvious.
/// Later you'll implement:
///   - DeckBuilderState -> Deck model
///   - serialize Deck as JSON in `decks/`
///   - refresh deck list, maybe auto-open the new deck, etc.
impl MorflashGui {
    /// Convert a DeckBuilderState into a deck file under `decks/`.
    /// For now this is a no-op stub to be implemented later.
    pub(crate) fn save_builder_state_as_deck(
        &mut self,
        _state: &DeckBuilderState,
    ) -> anyhow::Result<()> {
        // TODO:
        // 1. Map DeckBuilderState -> Deck
        // 2. Write Deck as JSON into `decks/`
        // 3. self.refresh_decks();
        Ok(())
    }
}
