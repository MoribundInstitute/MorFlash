// src/gui/app/deck_ops/builder_ops.rs

use std::fs;
use std::path::{Path, PathBuf};

use crate::gui::app::screens::deck_builder_screen::DeckBuilderState;
use crate::gui::app::MorflashGui;
use crate::srs::mflash::{MflashCard, MflashDeck};

/// Glue between DeckBuilderState and real `.mflash` deck files.
///
/// This converts the in-memory builder state into an `MflashDeck`
/// (the on-disk spec format), saves it as a `.mflash` file under
/// `decks/`, and refreshes the deck list so it appears in the UI.
impl MorflashGui {
    /// Convert the current DeckBuilderState into a deck file under `decks/`
    /// and return the path to the saved `.mflash` file.
    pub(crate) fn save_builder_state_as_deck(&mut self) -> anyhow::Result<PathBuf> {
        let state: &DeckBuilderState = &self.deck_builder_state;

        // ============================================================
        // 1. Ensure `decks/` dir exists.
        // ============================================================
        let decks_dir = Path::new("decks");
        fs::create_dir_all(decks_dir)?;

        // ============================================================
        // 2. Derive a safe base name from the builder's file_name field.
        // ============================================================
        let raw_name = state.file_name.trim();
        let base_name = if raw_name.is_empty() { "new_deck" } else { raw_name };

        let safe_name: String = base_name
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        let path = decks_dir.join(format!("{safe_name}.mflash"));

       // ============================================================
// 3. Build MflashCard list from the builder's cards, wiring up
//    all known metadata fields.
// ============================================================
let mut default_term_lang: Option<String> = None;
let mut default_def_lang: Option<String> = None;

let mut cards: Vec<MflashCard> = Vec::with_capacity(state.cards.len());

for c in &state.cards {
    // Optional languages: store only if non-empty.
    let term_lang_opt = if !c.term_lang.trim().is_empty() {
        // Also use the first non-empty as deck default.
        if default_term_lang.is_none() {
            default_term_lang = Some(c.term_lang.trim().to_string());
        }
        Some(c.term_lang.trim().to_string())
    } else {
        None
    };

    let def_lang_opt = if !c.def_lang.trim().is_empty() {
        if default_def_lang.is_none() {
            default_def_lang = Some(c.def_lang.trim().to_string());
        }
        Some(c.def_lang.trim().to_string())
    } else {
        None
    };

    // Optional hyperlink.
    let hyperlink_opt = if !c.hyperlink.trim().is_empty() {
        Some(c.hyperlink.trim().to_string())
    } else {
        None
    };

    // Optional media path.
    let media_opt = if !c.media_path.trim().is_empty() {
        Some(c.media_path.trim().to_string())
    } else {
        None
    };

    let card = MflashCard {
        term: c.term.clone(),
        definition: c.definition.clone(),
        term_lang: term_lang_opt,
        def_lang: def_lang_opt,
        hyperlink: hyperlink_opt,
        media: media_opt,
        tags: c.tags.clone(),
        examples: c.examples.clone(),
    };

    cards.push(card);
}


        // ============================================================
        // 4. Deck-level metadata from the builder.
        //
        // Right now we treat `state.tags` as a deck-level tag/description
        // field: split it into deck_tags and also use it as description
        // if non-empty. You can later replace this with explicit
        // deck-level fields like `state.deck_tags`, `state.description`,
        // etc., and wire them here.
        // ============================================================
        let deck_tags: Vec<String> = state
            .tags
            .split(|ch: char| ch == ',' || ch.is_whitespace())
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let description = if state.tags.trim().is_empty() {
            None
        } else {
            Some(state.tags.trim().to_string())
        };

        // TODO (future): once DeckBuilderState has explicit fields for:
        //   - snippet (short blurb)
        //   - cover_media (cover image path)
        //   - deck_tags as Vec<String>
        // you can replace the above heuristic with those fields directly.

        let payload = MflashDeck {
            format: "mflash".to_string(),
            version: 1,
            title: base_name.to_string(),
            description,
            snippet: None,                // TODO: wire from DeckBuilderState when available
            default_term_lang,            // inferred from first non-empty card term_lang
            default_def_lang,             // inferred from first non-empty card def_lang
            deck_tags,
            cover_media: None,            // TODO: wire from DeckBuilderState when available
            cards,
        };

        // ============================================================
        // 5. Save as JSON `.mflash`.
        // ============================================================
        let bytes = serde_json::to_vec_pretty(&payload)?;
        fs::write(&path, bytes)?;

        // ============================================================
        // 6. Refresh deck list so it appears in the UI.
        // ============================================================
        self.deck_paths = Self::load_all_deck_paths("decks").unwrap_or_default();

        Ok(path)
    }
}
