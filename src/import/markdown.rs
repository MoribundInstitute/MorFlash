// src/import/markdown.rs

use crate::model::Deck;

pub fn deck_from_markdown(_raw: &str) -> anyhow::Result<Deck> {
    anyhow::bail!("Markdown import not implemented yet");
}
