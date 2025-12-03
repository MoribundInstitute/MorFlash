// src/gui/app/deck_ops/import_ops.rs

use std::fs;
use std::path::Path;

use crate::gui::app::MorflashGui;
use crate::import;

impl MorflashGui {
    /// Import a deck file (json / csv / txt / etc.), normalize it into a `Deck`,
    /// and save it as JSON under `decks/`, then refresh the deck list.
    pub(crate) fn import_deck(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(
                "Deck files",
                &["json", "csv", "txt", "md", "markdown", "xml"],
            )
            .pick_file()
        {
            let decks_dir = Path::new("decks");
            if let Err(e) = fs::create_dir_all(decks_dir) {
                eprintln!("Failed to create decks dir: {e}");
                return;
            }

            match import::import_deck_file(&path) {
                Ok(deck) => {
                    let safe_name = deck.name.replace('/', "_");
                    let dest = decks_dir.join(format!("{safe_name}.json"));

                    if let Err(e) = fs::write(&dest, serde_json::to_string_pretty(&deck).unwrap()) {
                        eprintln!("Failed to write deck JSON: {e}");
                    } else {
                        self.refresh_decks();
                    }
                }
                Err(e) => {
                    eprintln!("Failed to import deck: {e}");
                }
            }
        }
    }
}
