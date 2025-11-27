// src/import/xml.rs

use crate::model::Deck;

pub fn deck_from_xml(_raw: &str) -> anyhow::Result<Deck> {
    anyhow::bail!("XML import not implemented yet");
}
