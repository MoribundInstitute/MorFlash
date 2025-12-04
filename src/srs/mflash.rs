// src/srs/mflash.rs
//
// .mflash support: a JSON-based deck format on disk using the `.mflash`
// extension, aligned with the mflash v1 specification.
//
// v1 is a single UTF-8 JSON object with the following high-level
// structure:
//
// MflashDeck {
//     format: "mflash",
//     version: 1,
//     title: String,
//     description: Option<String>,
//     snippet: Option<String>,
//     default_term_lang: Option<String>,
//     default_def_lang: Option<String>,
//     deck_tags: Vec<String>,
//     cover_media: Option<String>,
//     cards: Vec<MflashCard>,
// }
//
// MflashCard {
//     term: String,
//     definition: String,
//     term_lang: Option<String>,
//     def_lang: Option<String>,
//     hyperlink: Option<String>,
//     media: Option<String>,
//     tags: Vec<String>,
//     examples: Vec<String>,
//     notes: Option<String>,
// }

use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::model::{Card, Deck};

/// Language code for cards/decks (e.g. "en", "fr", "ja-JP", "zh-CN").
pub type LangCode = String;

/// A single card in a `.mflash` deck.
#[derive(Debug, Serialize, Deserialize)]
pub struct MflashCard {
    /// Prompt side (Unicode).
    pub term: String,

    /// Answer side (Unicode).
    pub definition: String,

    /// Language of `term` (overrides deck default if present).
    pub term_lang: Option<LangCode>,

    /// Language of `definition` (overrides deck default if present).
    pub def_lang: Option<LangCode>,

    /// Optional external URL associated with the card.
    pub hyperlink: Option<String>,

    /// Optional relative media path (image / gif / video / audio).
    pub media: Option<String>,

    /// Per-card tags (topics, difficulty, etc.).
    #[serde(default)]
    pub tags: Vec<String>,

    /// Example sentences or usage notes.
    #[serde(default)]
    pub examples: Vec<String>,
}

/// Top-level `.mflash` deck object.
#[derive(Debug, Serialize, Deserialize)]
pub struct MflashDeck {
    /// Must be the literal string `"mflash"`.
    pub format: String,

    /// Format version. This implementation currently supports version 1.
    pub version: u16,

    /// Human-readable deck title.
    pub title: String,

    /// Optional longer deck description.
    pub description: Option<String>,

    /// Short preview text for UIs (e.g. file pickers).
    pub snippet: Option<String>,

    /// Default language for `term` when a card does not set `term_lang`.
    pub default_term_lang: Option<LangCode>,

    /// Default language for `definition` when a card does not set `def_lang`.
    pub default_def_lang: Option<LangCode>,

    /// Deck-level tags (Dewey codes, subject, difficulty, etc.).
    #[serde(default)]
    pub deck_tags: Vec<String>,

    /// Optional relative path to a cover/thumbnail image.
    pub cover_media: Option<String>,

    /// All cards in this deck.
    #[serde(default)]
    pub cards: Vec<MflashCard>,
}

impl MflashDeck {
    pub fn effective_term_lang<'a>(&'a self, card: &'a MflashCard) -> Option<&'a str> {
        card.term_lang
            .as_deref()
            .or(self.default_term_lang.as_deref())
    }

    pub fn effective_def_lang<'a>(&'a self, card: &'a MflashCard) -> Option<&'a str> {
        card.def_lang
            .as_deref()
            .or(self.default_def_lang.as_deref())
    }
}

/// Convert an in-memory `Deck` into an `.mflash` deck payload.
///
/// Note: the current `Deck` / `Card` model does not track language metadata,
/// hyperlinks, examples, etc., so those fields are left empty/None.
impl From<&Deck> for MflashDeck {
    fn from(deck: &Deck) -> Self {
        let cards = deck
            .cards
            .iter()
            .map(|c| MflashCard {
                term: c.term.clone(),
                definition: c.definition.clone(),
                term_lang: None,
                def_lang: None,
                hyperlink: None,
                media: None,
                tags: Vec::new(),
                examples: Vec::new(),
            })
            .collect();

        Self {
            format: "mflash".to_string(),
            version: 1,
            title: deck.name.clone(),
            description: deck.description.clone(),
            snippet: None,
            default_term_lang: None,
            default_def_lang: None,
            deck_tags: Vec::new(),
            cover_media: None,
            cards,
        }
    }
}

/// Convert an `.mflash` deck back into the in-memory `Deck` type.
///
/// Extra metadata (languages, tags, examples, notes, media, etc.) is currently
/// not represented in `Deck` and is therefore ignored on import.
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
///
/// This performs basic validation of the `format` and `version` fields to make
/// sure we’re really looking at a supported `.mflash` deck.
pub fn load_mflash_deck(path: &Path) -> anyhow::Result<Deck> {
    let bytes = fs::read(path)?;
    let payload: MflashDeck = serde_json::from_slice(&bytes)?;

    if payload.format != "mflash" {
        return Err(anyhow::anyhow!(
            "Invalid .mflash deck: expected format \"mflash\", got \"{}\"",
            payload.format
        ));
    }

    if payload.version != 1 {
        return Err(anyhow::anyhow!(
            "Unsupported .mflash version {} (expected 1)",
            payload.version
        ));
    }

    Ok(payload.into())
}
