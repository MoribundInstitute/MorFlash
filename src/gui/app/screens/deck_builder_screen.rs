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
    /// Optional media attached to this card (image / gif / video path).
    pub media_path: Option<PathBuf>,
}

/// State for the deck builder screen.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeckBuilderState {
    /// Suggested file name for this deck (without extension).
    pub file_name: String,

    /// Tags that apply to the whole deck (comma-separated).
    pub tags: String,

    /// Optional deck thumbnail / media (image / gif / video path).
    pub media_path: Option<PathBuf>,

    /// All cards in this deck.
    pub cards: Vec<BuilderCard>,
}

/// Draw the deck builder screen.
/// Returns `true` when the user clicks "Save & Exit" and the deck saves successfully.
pub fn draw_deck_builder_screen(
    ctx: &egui::Context,
    state: &mut DeckBuilderState,
    _opts: &DeckBuilderOptions,
) -> bool {
    let mut done = false;

    // Apply menu theme globally.
    MenuTheme::apply_to_ctx(ctx);

    // ===== Bottom footer with Save & Exit =====
    egui::TopBottomPanel::bottom("deck_builder_footer").show(ctx, |ui| {
        egui::Frame::none()
            .fill(MenuTheme::PANEL_BG)
            .stroke(egui::Stroke::new(1.0, MenuTheme::BUTTON_OUTLINE))
            .inner_margin(egui::Margin::symmetric(16.0, 10.0))
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let button = egui::Button::new("Save & Exit")
                        .min_size(egui::vec2(140.0, 36.0))
                        .rounding(egui::Rounding::same(18.0));

                    if ui.add(button).clicked() {
                        if save_deck_to_disk(state) {
                            done = true;
                        }
                    }
                });
            });
    });

    // ===== Main content =====
    egui::CentralPanel::default().show(ctx, |ui| {
        let available = ui.available_size();

        // Use most of the available width.
        let panel_width = (available.x - 40.0).clamp(700.0, 1400.0);

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

                    // ===== Deck thumbnail / media =====
                    ui.label("Deck thumbnail (image / GIF / video):");
                    ui.horizontal(|ui| {
                        let current_media = state
                            .media_path
                            .as_ref()
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|| "None selected".to_string());

                        ui.label(current_media);

                        if ui.button("Browse…").clicked() {
                            if let Some(path) = FileDialog::new()
                                .add_filter(
                                    "Media",
                                    &["png", "jpg", "jpeg", "gif", "mp4", "webm"],
                                )
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

                    let mut remove_index: Option<usize> = None;

                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for (idx, card) in state.cards.iter_mut().enumerate() {
                                ui.group(|ui| {
                                    // Header row: "Card X" and Remove button on the right.
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

                                    ui.add_space(6.0);

                                    // Big card layout:
                                    // [ Term ] [      Definition      ] [ Image / Hyperlink ]
                                    egui::Frame::none()
                                        .stroke(egui::Stroke::new(
                                            1.0,
                                            MenuTheme::BUTTON_OUTLINE,
                                        ))
                                        .rounding(egui::Rounding::same(10.0))
                                        .inner_margin(egui::Margin::symmetric(8.0, 8.0))
                                        .show(ui, |ui| {
                                            ui.set_min_height(140.0);

                                            ui.horizontal(|ui| {
                                                let total = ui.available_width();
                                                let term_w = total * 0.22;
                                                let image_w = total * 0.22;
                                                let def_w = total
                                                    - term_w
                                                    - image_w
                                                    - ui.spacing().item_spacing.x * 2.0;

                                                // Term column (left).
                                                ui.allocate_ui_with_layout(
                                                    egui::vec2(term_w.max(160.0), 0.0),
                                                    egui::Layout::top_down(
                                                        egui::Align::LEFT,
                                                    ),
                                                    |ui| {
                                                        ui.label("Term");
                                                        ui.add_space(4.0);
                                                        ui.text_edit_singleline(&mut card.term);
                                                    },
                                                );

                                                // Definition column (center, big).
                                                ui.allocate_ui_with_layout(
                                                    egui::vec2(def_w.max(260.0), 0.0),
                                                    egui::Layout::top_down(
                                                        egui::Align::LEFT,
                                                    ),
                                                    |ui| {
                                                        ui.label("Definition");
                                                        ui.add_space(4.0);
                                                        ui.text_edit_multiline(
                                                            &mut card.definition,
                                                        );
                                                    },
                                                );

                                                // Right column: Image at top, Hyperlink at bottom.
                                                ui.allocate_ui_with_layout(
                                                    egui::vec2(image_w.max(180.0), 0.0),
                                                    egui::Layout::top_down(
                                                        egui::Align::LEFT,
                                                    ),
                                                    |ui| {
                                                        card_media_widget(
                                                            ui,
                                                            &mut card.media_path,
                                                        );
                                                        ui.add_space(6.0);
                                                        ui.label("Hyperlink (optional)");
                                                        ui.text_edit_singleline(
                                                            &mut card.hyperlink,
                                                        );
                                                    },
                                                );
                                            });
                                        });
                                });

                                ui.add_space(10.0);
                            }

                            ui.add_space(8.0);

                            // "Add a card" is always at the very bottom, centered.
                            ui.vertical_centered(|ui| {
                                if ui.button("➕ Add a card").clicked() {
                                    state.cards.push(BuilderCard::default());
                                }
                            });
                        });

                    // Actually remove card after iterating.
                    if let Some(i) = remove_index {
                        if i < state.cards.len() {
                            state.cards.remove(i);
                        }
                    }
                });
        });
    });

    done
}

/// Per-card media widget on the right side.
/// Browse… works; drag & drop is *attempted* but not fully tuned yet.
fn card_media_widget(ui: &mut egui::Ui, media_path: &mut Option<PathBuf>) {
    ui.label("Image / media");
    ui.add_space(4.0);

    // Draw the drop zone as a frame and get back a response.
    let response = egui::Frame::none()
        .stroke(egui::Stroke::new(1.0, MenuTheme::BUTTON_OUTLINE))
        .rounding(egui::Rounding::same(10.0))
        .inner_margin(egui::Margin::symmetric(6.0, 6.0))
        .show(ui, |ui| {
            ui.set_min_size(egui::vec2(180.0, 110.0));
            ui.vertical_centered(|ui| {
                ui.label("Drag image / GIF / video here");
            });
        })
        .response;

    // Try to react when files are dropped.
    let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());
    if response.hovered() && !dropped_files.is_empty() {
        if let Some(file) = dropped_files.first() {
            if let Some(path) = &file.path {
                *media_path = Some(path.clone());
            }
        }
    }

    ui.add_space(4.0);

    let current = media_path
        .as_ref()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "No file selected".to_string());
    ui.label(current);

    if ui.button("Browse…").clicked() {
        if let Some(path) = FileDialog::new()
            .add_filter("Media", &["png", "jpg", "jpeg", "gif", "mp4", "webm"])
            .pick_file()
        {
            *media_path = Some(path);
        }
    }
}

/// Save the current deck to `decks/<safe_name>.json`.
/// Returns `true` on success.
fn save_deck_to_disk(state: &DeckBuilderState) -> bool {
    let decks_dir = Path::new("decks");
    if let Err(e) = std::fs::create_dir_all(decks_dir) {
        eprintln!("MorFlash: failed to create decks dir {:?}: {e}", decks_dir);
        return false;
    }

    let raw_name = state.file_name.trim();
    let base_name = if raw_name.is_empty() { "new_deck" } else { raw_name };

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

    match serde_json::to_string_pretty(state) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                eprintln!("MorFlash: failed to save deck to {:?}: {e}", path);
                false
            } else {
                println!("MorFlash: deck saved to {:?}", path);
                true
            }
        }
        Err(e) => {
            eprintln!("MorFlash: failed to serialize deck: {e}");
            false
        }
    }
}
