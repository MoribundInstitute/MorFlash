use crate::model::{Card, Deck};

/// Core implementation: parse "Term - Definition" / "Term,Definition" style
/// text (including Quizlet-style exports) into a Deck.
///
/// Supported row separators:
///   - Newline (`\n`)  – normal pasted text / "New Line" option
///   - Semicolon (`;`) – Quizlet "Semicolon" between rows
///
/// Supported term/definition separators (in order of preference):
///   - Tab (`\t`)                 – many tools' default
///   - " - " (space, dash, space) – typical "Term - Definition"
///   - " – " / " — "              – smart quotes / long dashes
///   - ","                        – Quizlet "Comma" between term/definition
///
/// If no separator is found, the whole chunk becomes the term and the
/// definition is left empty.
pub fn deck_from_txt(name: &str, description: Option<String>, raw: &str) -> Deck {
    let mut cards = Vec::new();
    let mut next_id: u64 = 1;

    // Normalize Windows line endings just in case.
    let normalized = raw.replace("\r\n", "\n");

    // Split on potential row separators: newline OR semicolon.
    for chunk in normalized.split(|c| c == '\n' || c == ';') {
        let trimmed = chunk.trim();
        if trimmed.is_empty() {
            continue;
        }

        let (term, definition) = split_term_and_definition(trimmed);

        if term.is_empty() {
            continue;
        }

        cards.push(Card {
            id: next_id,
            term: term.to_string(),
            definition: definition.to_string(),
        });
        next_id += 1;
    }

    Deck {
        name: name.to_string(),
        description,
        cards,
    }
}

/// Try a bunch of separators that people commonly use between term & definition.
fn split_term_and_definition(s: &str) -> (&str, &str) {
    // Order matters: we prefer more specific separators first.
    // NOTE: we deliberately do NOT include bare "-" so hyphenated
    // terms like "tirra-lirra" or "mother-in-law" are safe.
    const SEPARATORS: [&str; 5] = ["\t", " - ", " – ", " — ", ","];

    for sep in SEPARATORS {
        if let Some((left, right)) = s.split_once(sep) {
            return (left.trim(), right.trim());
        }
    }

    // No recognizable separator → treat whole thing as the term.
    (s.trim(), "")
}

/// Backwards-compatible name used by older code.
pub fn deck_from_paste(name: &str, description: Option<String>, raw: &str) -> Deck {
    deck_from_txt(name, description, raw)
}
