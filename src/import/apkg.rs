// src/import/apkg.rs

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Context, Result};
use rusqlite::Connection;
use zip::ZipArchive;

use crate::model::Deck;
use super::deck_from_txt; // reuse the existing TXT importer

/// Import an Anki `.apkg` file or an *unzipped* APKG folder into a `Deck`.
///
/// Supported inputs:
/// - Path to `something.apkg`  → ZIP is opened, SQLite db extracted, notes read
/// - Path to directory that contains a `collection.anki21*` or `collection.anki2*`
///
/// Strategy for both:
/// - Locate `collection.anki21*` (preferred) or `collection.anki2*`
/// - Open SQLite DB
/// - Read `notes.flds`
/// - Treat field 0 as term, field 1 as definition
/// - Strip simple Anki markup like `[sound:...]` and basic `[anki:tts]` blocks
/// - Convert to a synthetic TXT deck and run `deck_from_txt`
pub fn deck_from_apkg(path: &Path) -> Result<Deck> {
    if path.is_dir() {
        // User has already unzipped the APKG into a folder.
        deck_from_unzipped_apkg_dir(path)
    } else {
        // Normal case: a single .apkg file (ZIP).
        deck_from_apkg_zip(path)
    }
}

/// Handle the "normal" case: a `.apkg` ZIP file.
fn deck_from_apkg_zip(path: &Path) -> Result<Deck> {
    // ----------------------------------------
    // 1. Open `.apkg` as a ZIP archive
    // ----------------------------------------
    let file = File::open(path)
        .with_context(|| format!("Failed to open .apkg file: {}", path.display()))?;
    let mut archive = ZipArchive::new(file)
        .with_context(|| format!("Failed to read .apkg ZIP structure: {}", path.display()))?;

    // ----------------------------------------
    // 2. Decide which DB file to use.
    //
    // We now handle names like:
    //   - "collection.anki21"
    //   - "collection.anki21b"
    //   - "collection.anki2"
    //   - Any path containing those segments.
    // ----------------------------------------
    let mut chosen: Option<String> = None;

    // Prefer any 21* variant
    for name in archive.file_names() {
        if name.contains("collection.anki21") {
            chosen = Some(name.to_string());
            break;
        }
    }

    // Fallback to any 2* variant
    if chosen.is_none() {
        for name in archive.file_names() {
            if name.contains("collection.anki2") {
                chosen = Some(name.to_string());
                break;
            }
        }
    }

    let db_name = chosen.ok_or_else(|| {
        anyhow!("APKG archive is missing a collection.anki21/collection.anki2 database file")
    })?;

    let mut db_entry = archive
        .by_name(&db_name)
        .with_context(|| format!("APKG archive is missing {db_name}"))?;

    // ----------------------------------------
    // 3. Extract DB to a temp file so rusqlite can open it.
    //    (Avoids extra crates like `tempfile`.)
    // ----------------------------------------
    let tmp_dir = std::env::temp_dir();
    let tmp_path = tmp_dir.join("morflash_apkg_collection.db");

    {
        let mut tmp_file = File::create(&tmp_path)
            .with_context(|| format!("Failed to create temp file at {}", tmp_path.display()))?;
        std::io::copy(&mut db_entry, &mut tmp_file)
            .context("Failed to copy SQLite DB out of APKG archive")?;
        tmp_file.flush().ok();
    }

    // ----------------------------------------
    // 4. Open SQLite DB
    // ----------------------------------------
    let conn = Connection::open(&tmp_path).context("Failed to open APKG SQLite DB")?;

    // Build synthetic TXT from this DB.
    let synthetic_txt = synthetic_txt_from_notes(&conn)?;

    // Best-effort cleanup of temp file (ignore errors).
    let _ = std::fs::remove_file(&tmp_path);

    // ----------------------------------------
    // 5. Build Deck via existing TXT importer
    // ----------------------------------------
    finalize_deck_from_synthetic_txt(path, &synthetic_txt)
}

/// Handle the case where the user has unzipped the APKG into a directory.
///
/// Expected directory contents (at minimum):
/// - collection.anki21b  OR
/// - collection.anki21   OR
/// - collection.anki2
fn deck_from_unzipped_apkg_dir(dir: &Path) -> Result<Deck> {
    if !dir.is_dir() {
        return Err(anyhow!(
            "Expected a directory for unzipped APKG, got: {}",
            dir.display()
        ));
    }

    // Try a few common Anki DB filenames in preferred order.
    let candidates = [
        "collection.anki21b",
        "collection.anki21",
        "collection.anki2",
    ];

    let db_path: Option<PathBuf> = candidates
        .iter()
        .map(|name| dir.join(name))
        .find(|candidate| candidate.exists());

    let db_path = db_path.ok_or_else(|| {
        anyhow!(
            "Unzipped APKG directory `{}` is missing collection.anki21*/collection.anki2 file",
            dir.display()
        )
    })?;

    let conn = Connection::open(&db_path)
        .with_context(|| format!("Failed to open SQLite DB at {}", db_path.display()))?;

    let synthetic_txt = synthetic_txt_from_notes(&conn)?;

    finalize_deck_from_synthetic_txt(dir, &synthetic_txt)
}

/// Very small, zero-dependency cleaner for some common Anki markup.
///
/// Currently:
/// - Strips all `[sound:...]` tags
/// - Strips simple `[anki:tts ...]...[/anki:tts]` blocks entirely
fn strip_anki_markup(s: &str) -> String {
    // 1) Remove [sound:...]
    let mut out = String::new();
    let mut rest = s;

    loop {
        if let Some(idx) = rest.find("[sound:") {
            // keep text before the tag
            out.push_str(&rest[..idx]);
            // skip past the closing ']'
            if let Some(close_rel) = rest[idx..].find(']') {
                let next_start = idx + close_rel + 1;
                rest = &rest[next_start..];
            } else {
                // malformed; keep as plain text
                out.push_str(&rest[idx..]);
                rest = "";
                break;
            }
        } else {
            out.push_str(rest);
            break;
        }
    }

    // 2) Remove simple [anki:tts ...]...[/anki:tts] blocks
    let mut cleaned = String::new();
    let mut rest2 = out.as_str();

    loop {
        if let Some(start) = rest2.find("[anki:tts") {
            cleaned.push_str(&rest2[..start]);
            if let Some(end_rel) = rest2[start..].find("[/anki:tts]") {
                let after = start + end_rel + "[/anki:tts]".len();
                rest2 = &rest2[after..];
            } else {
                // no closing tag; treat as plain text
                cleaned.push_str(&rest2[start..]);
                rest2 = "";
                break;
            }
        } else {
            cleaned.push_str(rest2);
            break;
        }
    }

    cleaned
}

/// Shared helper: read `notes.flds` and convert to a synthetic TXT deck.
///
/// In Anki, `notes.flds` is a single string with `\x1F` separators.
/// We treat:
///   field[0] → term
///   field[1] → definition
fn synthetic_txt_from_notes(conn: &Connection) -> Result<String> {
    let mut stmt = conn
        .prepare("SELECT flds FROM notes")
        .context("Failed to prepare notes query")?;
    let rows = stmt
        .query_map([], |row| {
            let flds: String = row.get(0)?;
            Ok(flds)
        })
        .context("Failed to iterate notes from APKG DB")?;

    let mut synthetic_txt = String::new();

    for row in rows {
        let flds = row.context("Failed to read a `flds` row from notes")?;
        let mut parts = flds.split('\u{1f}'); // \x1F is Anki's field separator

        let term_raw = parts.next().unwrap_or("").trim();
        let definition_raw = parts.next().unwrap_or("").trim();

        let term = strip_anki_markup(term_raw);
        let definition = strip_anki_markup(definition_raw);

        // Skip totally empty rows
        if term.trim().is_empty() && definition.trim().is_empty() {
            continue;
        }

        synthetic_txt.push_str(term.trim());
        synthetic_txt.push('\t');
        synthetic_txt.push_str(definition.trim());
        synthetic_txt.push('\n');
    }

    if synthetic_txt.trim().is_empty() {
        return Err(anyhow!(
            "APKG import produced no usable notes (no term/definition pairs found)"
        ));
    }

    Ok(synthetic_txt)
}

/// Finalize into a MorFlash `Deck` using the existing TXT importer.
fn finalize_deck_from_synthetic_txt(source_path: &Path, synthetic_txt: &str) -> Result<Deck> {
    let deck_name = source_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Imported Anki deck");

    let deck = deck_from_txt(deck_name, None, synthetic_txt);
    Ok(deck)
}
