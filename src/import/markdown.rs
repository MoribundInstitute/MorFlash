use crate::model::{Card, Deck};
use anyhow::Context;

/// Try multiple Markdown formats and return the first one that works.
pub fn deck_from_markdown(raw: &str) -> anyhow::Result<Deck> {
    // Try all formats in order of strict → loose
    try_markdown_heading_pairs(raw)
        .or_else(|_| try_markdown_bullets(raw))
        .or_else(|_| try_markdown_table_2col(raw))
        .or_else(|_| try_markdown_flashcard_blocks(raw))
        .or_else(|_| try_markdown_glossary_style(raw))
        .or_else(|_| try_markdown_term_colon_def(raw))
        .with_context(|| "Unsupported Markdown deck format")
}

//
// ─────────────────────────────────────────────────────
//  FORMAT 1 — # Term\nDefinition
// ─────────────────────────────────────────────────────
// Example:
//    ## Photosynthesis
//    The process by which plants...
//
fn try_markdown_heading_pairs(raw: &str) -> anyhow::Result<Deck> {
    let mut cards = Vec::new();
    let mut current_term: Option<String> = None;

    for line in raw.lines() {
        let line = line.trim();
        if line.starts_with("#") {
            // Heading → new term
            if let Some(term) = current_term.take() {
                // Previous heading had no definition → skip
                cards.push(Card {
                    id: cards.len() as u64 + 1,
                    term,
                    definition: "(no definition)".into(),
                });
            }
            let term = line.trim_start_matches('#').trim().to_string();
            current_term = Some(term);
        } else if !line.is_empty() {
            if let Some(term) = current_term.take() {
                // This line is the definition for the last heading
                cards.push(Card {
                    id: cards.len() as u64 + 1,
                    term,
                    definition: line.into(),
                });
            }
        }
    }

    if cards.is_empty() {
        anyhow::bail!("not heading-pairs markdown");
    }

    Ok(Deck {
        name: "Markdown Deck (Headings)".into(),
        description: None,
        cards,
    })
}

//
// ─────────────────────────────────────────────────────
//  FORMAT 2 — Bullet list: - term: definition
// ─────────────────────────────────────────────────────
//   - Dog: A domesticated mammal
//   - Cat: A small carnivore.
//
fn try_markdown_bullets(raw: &str) -> anyhow::Result<Deck> {
    let mut cards = Vec::new();

    for line in raw.lines() {
        let line = line.trim();
        if !(line.starts_with("- ") || line.starts_with("* ")) {
            continue;
        }

        let item = line
            .trim_start_matches("- ")
            .trim_start_matches("* ")
            .trim();
        if let Some((term, def)) = item.split_once(':') {
            cards.push(Card {
                id: cards.len() as u64 + 1,
                term: term.trim().into(),
                definition: def.trim().into(),
            });
        }
    }

    if cards.is_empty() {
        anyhow::bail!("not bullet-style markdown");
    }

    Ok(Deck {
        name: "Markdown Deck (Bullets)".into(),
        description: None,
        cards,
    })
}

//
// ─────────────────────────────────────────────────────
//  FORMAT 3 — Markdown table
// ─────────────────────────────────────────────────────
// | Term | Definition |
// |------|------------|
// | Dog  | A mammal   |
//
fn try_markdown_table_2col(raw: &str) -> anyhow::Result<Deck> {
    let mut lines = raw.lines();

    let header = lines.next().unwrap_or("").trim();
    if !header.starts_with('|') {
        anyhow::bail!("not a table");
    }

    // skip the separator row
    let _sep = lines.next().unwrap_or("");

    let mut cards = Vec::new();
    for line in lines {
        let cols: Vec<_> = line
            .split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if cols.len() < 2 {
            continue;
        }
        cards.push(Card {
            id: cards.len() as u64 + 1,
            term: cols[0].into(),
            definition: cols[1].into(),
        });
    }

    if cards.is_empty() {
        anyhow::bail!("not markdown table");
    }

    Ok(Deck {
        name: "Markdown Deck (Table)".into(),
        description: None,
        cards,
    })
}

//
// ─────────────────────────────────────────────────────
//  FORMAT 4 — Flashcard blocks
// ─────────────────────────────────────────────────────
// ```card
// Term: Dog
// Definition: A mammal
// ```
//
fn try_markdown_flashcard_blocks(raw: &str) -> anyhow::Result<Deck> {
    let mut cards = Vec::new();
    let mut in_block = false;
    let mut term = String::new();
    let mut def = String::new();

    for line in raw.lines() {
        let line = line.trim();

        if line == "```card" {
            in_block = true;
            term.clear();
            def.clear();
            continue;
        }
        if line == "```" {
            if in_block && !term.is_empty() {
                cards.push(Card {
                    id: cards.len() as u64 + 1,
                    term: term.clone(),
                    definition: def.clone(),
                });
            }
            in_block = false;
            continue;
        }

        if in_block {
            if let Some((k, v)) = line.split_once(':') {
                match k.trim().to_lowercase().as_str() {
                    "term" => term = v.trim().into(),
                    "definition" => def = v.trim().into(),
                    _ => {}
                }
            }
        }
    }

    if cards.is_empty() {
        anyhow::bail!("not code-fenced markdown cards");
    }

    Ok(Deck {
        name: "Markdown Deck (Card Blocks)".into(),
        description: None,
        cards,
    })
}

//
// ─────────────────────────────────────────────────────
//  FORMAT 5 — Glossary style: **Term** — Definition
// ─────────────────────────────────────────────────────
//
// **Dog** — A mammal
//
fn try_markdown_glossary_style(raw: &str) -> anyhow::Result<Deck> {
    let mut cards = Vec::new();

    for line in raw.lines() {
        let line = line.trim();
        if !line.starts_with("**") {
            continue;
        }
        if let Some((term_raw, def_raw)) = line.split_once('—') {
            let term = term_raw
                .trim()
                .trim_start_matches("**")
                .trim_end_matches("**");
            let def = def_raw.trim();
            cards.push(Card {
                id: cards.len() as u64 + 1,
                term: term.into(),
                definition: def.into(),
            });
        }
    }

    if cards.is_empty() {
        anyhow::bail!("not glossary markdown");
    }

    Ok(Deck {
        name: "Markdown Deck (Glossary)".into(),
        description: None,
        cards,
    })
}

//
// ─────────────────────────────────────────────────────
//  FORMAT 6 — term: definition (bare lines)
//
// Dog: A mammal
// Cat: A small carnivore
//
fn try_markdown_term_colon_def(raw: &str) -> anyhow::Result<Deck> {
    let mut cards = Vec::new();

    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some((term, def)) = trimmed.split_once(':') {
            cards.push(Card {
                id: cards.len() as u64 + 1,
                term: term.trim().into(),
                definition: def.trim().into(),
            });
        }
    }

    if cards.is_empty() {
        anyhow::bail!("not colon-definition markdown");
    }

    Ok(Deck {
        name: "Markdown Deck (Colon Format)".into(),
        description: None,
        cards,
    })
}
