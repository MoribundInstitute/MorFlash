// src/import/xml.rs

use crate::model::Deck;

/// Placeholder: XML import not implemented yet.
///
/// Later, we can support things like Anki's .apkg-exported XML,
/// QTI, or your own XML vocab format.
///
/// For now this just returns an error so the rest of the app compiles.
pub fn deck_from_xml(_raw: &str) -> anyhow::Result<Deck> {
    anyhow::bail!("XML import not implemented yet")
}
