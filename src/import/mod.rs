// src/import/mod.rs

mod txt;
mod json;
mod csv;
mod markdown;
mod xml;

pub use txt::deck_from_paste;
pub use json::deck_from_json_deck;
pub use csv::deck_from_csv;
pub use markdown::deck_from_markdown;
pub use xml::deck_from_xml;

use std::path::Path;
use crate::model::Deck;

/// High-level entry point: pick importer based on file extension.
pub fn import_deck_file(path: &Path) -> anyhow::Result<Deck> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let content = std::fs::read_to_string(path)?;

    match ext.as_str() {
        "json" => deck_from_json_deck(&content),
        "csv" => deck_from_csv(&content),
        "md" | "markdown" => deck_from_markdown(&content),
        "xml" => deck_from_xml(&content),
        "txt" | _ => {
            // default: treat as "Term - Definition" per line
            Ok(deck_from_paste(
                path.file_stem().and_then(|s| s.to_str()).unwrap_or("Imported deck"),
                None,
                &content,
            ))
        }
    }
}
