// src/import/json.rs

use crate::model::Deck;

/// Expect full `Deck` JSON (with name, description, cards).
pub fn deck_from_json_deck(raw: &str) -> anyhow::Result<Deck> {
    let deck: Deck = serde_json::from_str(raw)?;
    Ok(deck)
}
