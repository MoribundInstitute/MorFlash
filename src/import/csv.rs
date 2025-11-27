// src/import/csv.rs

use crate::model::{Card, Deck};

/// Very simple CSV: "term,definition" per line (no headers).
pub fn deck_from_csv(raw: &str) -> anyhow::Result<Deck> {
    let mut cards = Vec::new();
    let mut next_id = 1u64;

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let mut parts = trimmed.splitn(2, ',');
        let term = parts.next().unwrap().trim();
        let definition = parts.next().unwrap_or("").trim();

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

    Ok(Deck {
        name: "CSV Deck".to_string(),
        description: None,
        cards,
    })
}
