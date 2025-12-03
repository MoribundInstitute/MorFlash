use crate::model::{Card, Deck};

/// Flexible CSV importer:
///
/// - Accepts files *with or without* a header row:
///     term,definition
///     front,back
///     question,answer
///     word,meaning
///
/// - Extra columns (like tags, notes) are ignored for now.
/// - Falls back to a simple "term,definition" parser if CSV parsing fails.
pub fn deck_from_csv(raw: &str) -> anyhow::Result<Deck> {
    // First, try a more robust CSV parser (handles quotes, commas in text, etc.).
    if let Ok(deck) = deck_from_flexible_csv(raw) {
        return Ok(deck);
    }

    // If that fails (or yields nothing), fall back to the original simple logic.
    deck_from_legacy_csv(raw)
}

/// Use the `csv` crate to parse more real-world CSVs.
fn deck_from_flexible_csv(raw: &str) -> anyhow::Result<Deck> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false) // we’ll detect headers ourselves
        .flexible(true) // allow varying numbers of columns
        .from_reader(raw.as_bytes());

    let mut cards = Vec::new();
    let mut next_id: u64 = 1;
    let mut row_index: usize = 0;

    for result in rdr.records() {
        let record = result?;
        let len = record.len();
        if len == 0 {
            row_index += 1;
            continue;
        }

        let first = record.get(0).unwrap_or("").trim();
        let second = record.get(1).unwrap_or("").trim();

        // If the *first* row looks like a header ("term,definition", "front,back", etc.),
        // skip it.
        if row_index == 0 && looks_like_header(first, second) {
            row_index += 1;
            continue;
        }

        if first.is_empty() && second.is_empty() {
            row_index += 1;
            continue;
        }

        // Normal case: first column = term, second = definition.
        let term = first;
        let definition = second;

        if term.is_empty() || definition.is_empty() {
            row_index += 1;
            continue;
        }

        cards.push(Card {
            id: next_id,
            term: term.to_string(),
            definition: definition.to_string(),
        });
        next_id += 1;
        row_index += 1;
    }

    if cards.is_empty() {
        anyhow::bail!("No cards parsed from CSV");
    }

    Ok(Deck {
        name: "CSV Deck".to_string(),
        description: None,
        cards,
    })
}

/// Very simple header detection for common schemas.
fn looks_like_header(first: &str, second: &str) -> bool {
    let f = first.to_lowercase();
    let s = second.to_lowercase();

    let front_words = ["term", "front", "question", "word", "prompt"];
    let back_words = ["definition", "back", "answer", "meaning", "translation"];

    front_words.contains(&f.as_str()) && back_words.contains(&s.as_str())
}

/// Original, minimal "term,definition" parser used as a fallback.
///
/// This keeps compatibility with your old CSV behavior.
fn deck_from_legacy_csv(raw: &str) -> anyhow::Result<Deck> {
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

    if cards.is_empty() {
        anyhow::bail!("No cards parsed from legacy CSV");
    }

    Ok(Deck {
        name: "CSV Deck".to_string(),
        description: None,
        cards,
    })
}
