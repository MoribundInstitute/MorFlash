// src/gui/app/screens/deck_builder_screen.rs

use eframe::egui;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::gui::app::screens::options_screen::DeckBuilderOptions;
use crate::gui::theme::MenuTheme;

/// One flashcard being edited in the deck builder.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuilderCard {
    pub term: String,
    pub definition: String,
    /// Optional hyperlink associated with this card.
    pub hyperlink: String,
}

/// State for the deck builder screen.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeckBuilderState {
    /// Suggested file name for this deck (without extension).
    pub file_name: String,

    /// Tags that apply to the whole deck (comma-separated).
    pub tags: String,

    /// Optional media attached to the deck (image / gif / video path).
    pub media_path: Option<PathBuf>,

    /// All cards in this deck.
    pub cards: Vec<BuilderCard>,
}

/// Draw the deck builder screen.
/// Returns `true` when the user clicks "Done" (after attempting to save).
pub fn draw_deck_builder_screen(
    ctx: &egui::Context,
    state: &mut DeckBuilderState,
    _opts: &DeckBuilderOptions,
) -> bool {
    let mut done = false;

    egui::CentralPanel::default().show(ctx, |ui| {
        MenuTheme::apply_to_ctx(ui.ctx());

        let available = ui.available_size();
        let panel_width = (available.x * 0.8).clamp(700.0, 1100.0);

        ui.vertical_centered(|ui| {
            ui.add_space(16.0);
            ui.heading("Deck Builder");
            ui.add_space(8.0);

            egui::Frame::none()
                .fill(MenuTheme::PANEL_BG)
                .stroke(egui::Stroke::new(1.5, MenuTheme::BUTTON_OUTLINE))
                .rounding(egui::Rounding::same(18.0))
                .inner_margin(egui::Margin::symmetric(24.0, 20.0))
                .show(ui, |ui| {
                    ui.set_width(panel_width);

                    // ===== Deck metadata =====
                    ui.label("Deck file name (without extension):");
                    ui.text_edit_singleline(&mut state.file_name);
                    ui.add_space(8.0);

                    ui.label("Tags for this deck (comma-separated):");
                    ui.text_edit_singleline(&mut state.tags);
                    ui.add_space(12.0);

                    // ===== Deck media (image / gif / video) =====
                    ui.label("Deck media (image / GIF / video):");
                    ui.horizontal(|ui| {
                        let current_media = state
                            .media_path
                            .as_ref()
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|| "None selected".to_string());

                        ui.label(current_media);

                        if ui.button("Browse…").clicked() {
                            if let Some(path) = FileDialog::new()
                                .add_filter("Media", &["png", "jpg", "jpeg", "gif", "mp4", "webm"])
                                .pick_file()
                            {
                                state.media_path = Some(path);
                            }
                        }
                    });

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(12.0);

                    // ===== Cards section =====
                    ui.heading("Cards");
                    ui.add_space(8.0);

                    // Add card button
                    if ui.button("➕ Add a card").clicked() {
                        state.cards.push(BuilderCard::default());
                    }

                    ui.add_space(12.0);

                    // Show all cards
                    let mut remove_index: Option<usize> = None;

                    for (idx, card) in state.cards.iter_mut().enumerate() {
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("Card {}", idx + 1));
                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::Center),
                                    |ui| {
                                        if ui.button("🗑 Remove").clicked() {
                                            remove_index = Some(idx);
                                        }
                                    },
                                );
                            });

                            ui.add_space(4.0);

                            ui.label("Term");
                            ui.text_edit_singleline(&mut card.term);
                            ui.add_space(4.0);

                            ui.label("Definition");
                            ui.text_edit_multiline(&mut card.definition);
                            ui.add_space(4.0);

                            ui.label("Hyperlink (optional)");
                            ui.text_edit_singleline(&mut card.hyperlink);
                        });

                        ui.add_space(8.0);
                    }

                    if let Some(i) = remove_index {
                        state.cards.remove(i);
                    }

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(12.0);

                    // ===== Done button =====
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let done_button = egui::Button::new("Done")
                            .min_size(egui::vec2(120.0, 36.0))
                            .rounding(egui::Rounding::same(18.0));

                        if ui.add(done_button).clicked() {
                            // 1) Ensure the decks directory exists.
                            let decks_dir = Path::new("decks");
                            if let Err(e) = std::fs::create_dir_all(decks_dir) {
                                eprintln!(
                                    "MorFlash: failed to create decks dir {:?}: {e}",
                                    decks_dir
                                );
                                return;
                            }

                            // 2) Pick a safe file name.
                            let raw_name = state.file_name.trim().to_string();

                            let base_name = if raw_name.is_empty() {
                                "new_deck".to_string()
                            } else {
                                raw_name
                            };

                            // Replace anything weird with underscores.
                            let safe_name: String = base_name
                                .chars()
                                .map(|c| {
                                    if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                                        c
                                    } else {
                                        '_'
                                    }
                                })
                                .collect();

                            let path = decks_dir.join(format!("{safe_name}.json"));

                            // 3) Serialize DeckBuilderState to JSON and write it.
                            match serde_json::to_string_pretty(state) {
                                Ok(json) => {
                                    if let Err(e) = std::fs::write(&path, json) {
                                        eprintln!(
                                            "MorFlash: failed to save deck to {:?}: {e}",
                                            path
                                        );
                                    } else {
                                        println!("MorFlash: deck saved to {:?}", path);
                                        done = true; // tell caller to go back to DeckList
                                    }
                                }
                                Err(e) => {
                                    eprintln!("MorFlash: failed to serialize deck: {e}");
                                }
                            }
                        }
                    });
                });
        });
    });

    done
}
