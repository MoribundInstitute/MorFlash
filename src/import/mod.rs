// src/import/mod.rs

mod csv;
mod json;
mod markdown;
mod txt;
mod xml;
mod apkg;

pub use csv::deck_from_csv;
pub use json::{deck_from_any_json, deck_from_json_deck};
pub use markdown::deck_from_markdown;
pub use txt::{deck_from_paste, deck_from_txt};
pub use xml::deck_from_xml;
pub use apkg::deck_from_apkg;

use crate::model::Deck;
use std::path::Path;

/// High-level entry point: choose importer based on file extension or type.
///
/// - Directory               → treated as an *unzipped APKG* (collection.anki2 / anki21*)
/// - `.apkg`                 → binary SQLite/ZIP importer
/// - `.json`                 → JSON importer
/// - `.csv`                  → CSV importer
/// - `.md` / `.markdown`     → Markdown importer
/// - `.xml`                  → XML importer
/// - `.txt` / unknown        → text importer
pub fn import_deck_file(path: &Path) -> anyhow::Result<Deck> {
    // Special case: directory → assume unzipped APKG (like `/tmp/apkg_test`).
    if path.is_dir() {
        return deck_from_apkg(path);
    }

    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    let deck_name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Imported deck");

    // Binary format: APKG (ZIP + SQLite) — do *not* read as text.
    if ext == "apkg" {
        return deck_from_apkg(path);
    }

    // Everything else is text-based: read once, then dispatch.
    let content = std::fs::read_to_string(path)?;

    let deck: Deck = match ext.as_str() {
        // -------------------------
        // JSON, CSV, MD, XML (text-based)
        // -------------------------
        "json" => deck_from_any_json(&content)?,
        "csv" => deck_from_csv(&content)?,
        "md" | "markdown" => deck_from_markdown(&content)?,
        "xml" => deck_from_xml(&content)?,

        // -------------------------
        // TXT or unknown: treat as plain text
        // -------------------------
        "txt" | "" => deck_from_txt(deck_name, None, &content),
        _ => deck_from_txt(deck_name, None, &content),
    };

    Ok(deck)
}
