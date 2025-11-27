use eframe::egui;
use std::path::PathBuf;

pub fn draw_deck_list(ui: &mut egui::Ui, deck_paths: &[PathBuf]) -> Option<PathBuf> {
    ui.label("Choose a deck:");

    let mut clicked_deck: Option<PathBuf> = None;

    if deck_paths.is_empty() {
        ui.label("No decks found in ./decks");
    } else {
        for path in deck_paths {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("<deck>");

            if ui.button(name).clicked() {
                clicked_deck = Some(path.clone());
            }
        }
    }

    clicked_deck
}
