// src/import/txt.rs

use crate::model::{Card, Deck};

/// Parse "Term - Definition" text into a Deck.
pub fn deck_from_paste(name: &str, description: Option<String>, raw: &str) -> Deck {
    let mut cards = Vec::new();

    let mut next_id: u64 = 1;

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Split only on the *first* '-' so "pot-hole" is fine
        let (term_part, def_part) = match trimmed.split_once('-') {
            Some(pair) => pair,
            None => continue,
        };

        let term = term_part.trim();
        let definition = def_part.trim();

        if term.is_empty() || definition.is_empty() {
            continue;
        }

        cards.push(Card {
            id: next_id,
            term: term.to_string(),
            definition: definition.to_string(),
        });
        next_id += 1;
    }

    Deck {
        name: name.to_string(),
        description,
        cards,
    }
}
