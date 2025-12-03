// src/import/json.rs

use crate::model::{Card, Deck};
use serde_json::Value;

/// Expect full `Deck` JSON (with name, description, cards).
pub fn deck_from_json_deck(raw: &str) -> anyhow::Result<Deck> {
    let deck: Deck = serde_json::from_str(raw)?;
    Ok(deck)
}

/// Universal importer that tries all JSON formats.
pub fn deck_from_any_json(raw: &str) -> anyhow::Result<Deck> {
    // Try full deck structure
    if let Ok(deck) = deck_from_json_deck(raw) {
        return Ok(deck);
    }

    // Try cards array
    if let Ok(deck) = deck_from_json_cards_array(raw) {
        return Ok(deck);
    }

    // Try dictionary map
    if let Ok(deck) = deck_from_json_map(raw) {
        return Ok(deck);
    }

    // Try string list (terms only)
    if let Ok(deck) = deck_from_json_string_array(raw) {
        return Ok(deck);
    }

    // Try pairs [term, definition]
    if let Ok(deck) = deck_from_json_pairs(raw) {
        return Ok(deck);
    }

    // Try category → pairs map
    if let Ok(deck) = deck_from_json_category_map(raw) {
        return Ok(deck);
    }

    anyhow::bail!("Unsupported JSON deck format")
}

//
// ────────────────────────────────────────────────────────────────
//   BELOW ARE TEMPORARY STUBS — THESE LET THE FILE COMPILE
//   Replace them later with real implementations
// ────────────────────────────────────────────────────────────────
//

/// JSON: [{"term": "...", "definition": "..."}]
pub fn deck_from_json_cards_array(raw: &str) -> anyhow::Result<Deck> {
    let value: Value = serde_json::from_str(raw)?;
    let array = value.as_array().ok_or(anyhow::anyhow!("not array"))?;

    let mut cards = Vec::new();
    let mut next_id = 1;

    for item in array {
        let term = item.get("term").and_then(|v| v.as_str());
        let definition = item.get("definition").and_then(|v| v.as_str());
        if let (Some(t), Some(d)) = (term, definition) {
            cards.push(Card {
                id: next_id,
                term: t.to_string(),
                definition: d.to_string(),
            });
            next_id += 1;
        }
    }

    if cards.is_empty() {
        anyhow::bail!("no cards found");
    }

    Ok(Deck {
        name: "JSON Cards Deck".to_string(),
        description: None,
        cards,
    })
}

/// JSON: {"term": "definition", ...}
pub fn deck_from_json_map(raw: &str) -> anyhow::Result<Deck> {
    let value: Value = serde_json::from_str(raw)?;
    let obj = value.as_object().ok_or(anyhow::anyhow!("not object"))?;

    let mut cards = Vec::new();
    let mut next_id = 1;

    for (term, def_value) in obj {
        if let Some(def) = def_value.as_str() {
            cards.push(Card {
                id: next_id,
                term: term.to_string(),
                definition: def.to_string(),
            });
            next_id += 1;
        }
    }

    if cards.is_empty() {
        anyhow::bail!("JSON map had no string definitions");
    }

    Ok(Deck {
        name: "JSON Dictionary Deck".to_string(),
        description: None,
        cards,
    })
}

/// JSON: ["word1", "word2", "word3"]
pub fn deck_from_json_string_array(raw: &str) -> anyhow::Result<Deck> {
    let value: Value = serde_json::from_str(raw)?;
    let arr = value.as_array().ok_or(anyhow::anyhow!("not array"))?;

    let mut cards = Vec::new();
    let mut next_id = 1;

    for v in arr {
        if let Some(term) = v.as_str() {
            cards.push(Card {
                id: next_id,
                term: term.to_string(),
                definition: "?".to_string(),
            });
            next_id += 1;
        }
    }

    if cards.is_empty() {
        anyhow::bail!("string array contained no strings");
    }

    Ok(Deck {
        name: "Term List Deck".to_string(),
        description: None,
        cards,
    })
}

/// JSON: [["term", "definition"], ...]
pub fn deck_from_json_pairs(raw: &str) -> anyhow::Result<Deck> {
    let value: Value = serde_json::from_str(raw)?;
    let arr = value.as_array().ok_or(anyhow::anyhow!("not array"))?;

    let mut cards = Vec::new();
    let mut next_id = 1;

    for v in arr {
        if let Some(list) = v.as_array() {
            if list.len() == 2 {
                if let (Some(t), Some(d)) = (list[0].as_str(), list[1].as_str()) {
                    cards.push(Card {
                        id: next_id,
                        term: t.to_string(),
                        definition: d.to_string(),
                    });
                    next_id += 1;
                    continue;
                }
            }
        }
    }

    if cards.is_empty() {
        anyhow::bail!("no valid term/definition pairs");
    }

    Ok(Deck {
        name: "JSON Pairs Deck".to_string(),
        description: None,
        cards,
    })
}

/// JSON: { "Category": [["term","def"], ...], ... }
pub fn deck_from_json_category_map(raw: &str) -> anyhow::Result<Deck> {
    let value: Value = serde_json::from_str(raw)?;
    let obj = value.as_object().ok_or(anyhow::anyhow!("not object"))?;

    let mut cards = Vec::new();
    let mut next_id = 1;

    for (_cat, arr_value) in obj {
        if let Some(arr) = arr_value.as_array() {
            for v in arr {
                if let Some(pair) = v.as_array() {
                    if pair.len() == 2 {
                        if let (Some(t), Some(d)) = (pair[0].as_str(), pair[1].as_str()) {
                            cards.push(Card {
                                id: next_id,
                                term: t.to_string(),
                                definition: d.to_string(),
                            });
                            next_id += 1;
                        }
                    }
                }
            }
        }
    }

    if cards.is_empty() {
        anyhow::bail!("no cards in category map");
    }

    Ok(Deck {
        name: "JSON Category Deck".to_string(),
        description: None,
        cards,
    })
}
