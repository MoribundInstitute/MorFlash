// src/srs/mflash.rs
//
// Minimal .mflash support: a custom deck container that is just
// JSON on disk but uses the .mflash extension so you can evolve
// it later without breaking imports.

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::model::{Card, Deck};

#[derive(Debug, Serialize, Deserialize)]
pub struct MflashCard {
    pub term: String,
    pub definition: String,
    pub media: Option<String>, // optional per-card media path/URL
    pub tags: Vec<String>,     // future-proof field for tags
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MflashDeck {
    pub version: u16, // for future migrations
    pub title: String,
    pub description: Option<String>,
    pub cards: Vec<MflashCard>,
}

impl From<&Deck> for MflashDeck {
    fn from(deck: &Deck) -> Self {
        let cards = deck
            .cards
            .iter()
            .map(|c| MflashCard {
                term: c.term.clone(),
                definition: c.definition.clone(),
                media: None,
                tags: Vec::new(),
            })
            .collect();

        Self {
            version: 1,
            title: deck.name.clone(),
            description: deck.description.clone(),
            cards,
        }
    }
}

/// Convert an `.mflash` deck back into the in-memory `Deck` type.
impl From<MflashDeck> for Deck {
    fn from(m: MflashDeck) -> Self {
        let cards = m
            .cards
            .into_iter()
            .enumerate()
            .map(|(i, c)| Card {
                id: (i as u64) + 1,
                term: c.term,
                definition: c.definition,
            })
            .collect();

        Deck {
            name: m.title,
            description: m.description,
            cards,
        }
    }
}

/// Save a `Deck` as a `.mflash` file (JSON payload).
pub fn save_mflash_deck(path: &Path, deck: &Deck) -> anyhow::Result<()> {
    let payload = MflashDeck::from(deck);
    let bytes = serde_json::to_vec_pretty(&payload)?;
    fs::write(path, bytes)?;
    Ok(())
}

/// Load a `.mflash` file into a `Deck`.
pub fn load_mflash_deck(path: &Path) -> anyhow::Result<Deck> {
    let bytes = fs::read(path)?;
    let payload: MflashDeck = serde_json::from_slice(&bytes)?;
    Ok(payload.into())
}
