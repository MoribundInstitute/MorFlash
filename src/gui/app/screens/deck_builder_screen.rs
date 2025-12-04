use eframe::egui;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::gui::app::screens::options_screen::DeckBuilderOptions;
use crate::gui::theme::MenuTheme;
use crate::import;
use crate::model::Deck;

/// One flashcard being edited in the deck builder.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BuilderCard {
    pub term: String,
    pub definition: String,

    /// Language code for the term, for example "en", "ja", "ko".
    pub term_lang: String,
    /// Language code for the definition, for example "en", "ja", "ko".
    pub def_lang: String,

    /// Optional hyperlink associated with this card.
    pub hyperlink: String,

    /// Optional media attached to this card (stored as a path string).
    pub media_path: String,

    /// Per-card tags.
    pub tags: Vec<String>,

    /// Example sentences for this card.
    pub examples: Vec<String>,
}

/// State for the deck builder screen.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeckBuilderState {
    /// Suggested file name for this deck (without extension).
    pub file_name: String,

    /// Tags that apply to the whole deck (comma-separated).
    pub tags: String,

    /// Optional deck thumbnail / media (image / GIF / video path as string).
    pub media_path: String,

    /// All cards in this deck.
    pub cards: Vec<BuilderCard>,
}

/// Draw the deck builder screen.
/// Returns `true` when the user chooses to leave the Deck Builder
/// (either via "Save & Exit" or via "Exit" without saving).
pub fn draw_deck_builder_screen(
    ctx: &egui::Context,
    state: &mut DeckBuilderState,
    opts: &DeckBuilderOptions,
) -> bool {
    let mut done = false;

    // Make Deck Builder text larger and easier to read.
    {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (egui::TextStyle::Body, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Button, egui::FontId::new(18.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Heading, egui::FontId::new(22.0, egui::FontFamily::Proportional)),
            (egui::TextStyle::Monospace, egui::FontId::new(17.0, egui::FontFamily::Monospace)),
            (egui::TextStyle::Small, egui::FontId::new(16.0, egui::FontFamily::Proportional)),
        ]
        .into();

        ctx.set_style(style);
    }

    // Apply menu theme globally.
    MenuTheme::apply_to_ctx(ctx);

    // ===== Bottom footer with Exit (left) and Save & Exit (right) =====
    egui::TopBottomPanel::bottom("deck_builder_footer").show(ctx, |ui| {
        egui::Frame::none()
            .fill(MenuTheme::PANEL_BG)
            .stroke(egui::Stroke::new(1.0, MenuTheme::BUTTON_OUTLINE))
            .inner_margin(egui::Margin::symmetric(16.0, 10.0))
            .show(ui, |ui| {
                ui.columns(2, |cols| {
                    // Left column: plain Exit (no save).
                    let exit_button = egui::Button::new("Exit")
                        .min_size(egui::vec2(120.0, 36.0))
                        .rounding(egui::Rounding::same(18.0));

                    if cols[0].add(exit_button).clicked() {
                        // Later you could hook opts.warn_on_unsaved_exit here.
                        done = true;
                    }

                    // Right column: Save & Exit, right-aligned.
                    cols[1].with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            let button = egui::Button::new("Save & Exit")
                                .min_size(egui::vec2(140.0, 36.0))
                                .rounding(egui::Rounding::same(18.0));

                            if ui.add(button).clicked() {
                                if save_deck_to_disk(state) {
                                    done = true;
                                }
                            }
                        },
                    );
                });
            });
    });

   // ===== Main content =====
egui::CentralPanel::default().show(ctx, |ui| {
    let available = ui.available_size();
    let panel_width = (available.x - 40.0).clamp(700.0, 1400.0);

    ui.add_space(16.0);
    ui.horizontal(|ui| {
        ui.heading("Deck Builder");
    });
    ui.add_space(8.0);

    ui.set_width(panel_width);

    // ------- Deck metadata -------
    ui.label("Deck title / file name (without extension):");
    ui.text_edit_singleline(&mut state.file_name);
    ui.add_space(8.0);

    ui.label("Deck tags (comma-separated):");
    ui.text_edit_singleline(&mut state.tags);
    ui.add_space(8.0);

    ui.label("Deck thumbnail (image / GIF / video):");
    ui.horizontal(|ui| {
        ui.label(if state.media_path.is_empty() {
            "None selected"
        } else {
            state.media_path.as_str()
        });

        if ui.button("Browse…").clicked() {
            if let Some(path) = FileDialog::new()
                .add_filter("Media", &["png", "jpg", "jpeg", "gif", "mp4", "webm"])
                .pick_file()
            {
                state.media_path = path.to_string_lossy().to_string();
            }
        }
    });

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    // ------- Cards section -------
    ui.horizontal(|ui| {
        ui.heading("Cards");
        ui.add_space(8.0);
        ui.label(format!("({} total)", state.cards.len()));

        // Import button (right side)
      ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
    if ui.button("Import from file…").clicked() {
        if let Some(path) = FileDialog::new()
            .add_filter(
                "Deck files",
                &[
                    "mflash",   // MorFlash native
                    "json",     // Standard JSON decks
                    "txt",      // Plain text lists
                    "csv",      // Spreadsheet-style lists
                    "md",       // Markdown
                    "markdown",
                    "xml",      // XML vocab exports
                    "apkg",     // Anki decks
                ],
            )
            .pick_file()
        {
            if let Err(err) = import_deck_into_builder(path.as_path(), state) {
                eprintln!("MorFlash: import into builder failed: {err}");
            }
        }
    }
});

    });
    ui.add_space(8.0);

    let mut remove_index: Option<usize> = None;

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            for (idx, card) in state.cards.iter_mut().enumerate() {
                ui.group(|ui| {
                    // Header row: "Card X" and Remove on the right.
                    ui.horizontal(|ui| {
                        ui.label(format!("Card {}", idx + 1));

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("🗑 Remove").clicked() {
                                remove_index = Some(idx);
                            }
                        });
                    });

                    ui.add_space(6.0);

                    // Main row: Term / Definition / Media+Link
                    ui.horizontal(|ui| {
                        let total = ui.available_width();

                        let term_w = total * 0.15;
                        let media_w = total * 0.20;
                        let def_w = total - term_w - media_w - ui.spacing().item_spacing.x * 2.0;

                        // --- TERM COLUMN ---
                        ui.allocate_ui_with_layout(
                            egui::vec2(term_w.max(140.0), 0.0),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                ui.label("Term");
                                ui.text_edit_singleline(&mut card.term);

                                ui.add_space(4.0);
                                language_combo(
                                    ui,
                                    (idx, "term_lang"),
                                    "Term language",
                                    &mut card.term_lang,
                                    opts,
                                );
                            },
                        );

                        // --- DEFINITION COLUMN ---
                        ui.allocate_ui_with_layout(
                            egui::vec2(def_w.max(320.0), 0.0),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                ui.label("Definition");
                                ui.add(
                                    egui::TextEdit::multiline(&mut card.definition)
                                        .desired_rows(4)
                                        .desired_width(f32::INFINITY),
                                );

                                ui.add_space(4.0);
                                language_combo(
                                    ui,
                                    (idx, "def_lang"),
                                    "Definition language",
                                    &mut card.def_lang,
                                    opts,
                                );
                            },
                        );

                        // --- MEDIA + LINK COLUMN ---
                        ui.allocate_ui_with_layout(
                            egui::vec2(media_w.max(180.0), 0.0),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                card_media_widget(ui, &mut card.media_path);
                                ui.add_space(4.0);

                                ui.label("Hyperlink (optional)");
                                ui.text_edit_singleline(&mut card.hyperlink);
                            },
                        );
                    });

                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Tags simple row.
                    ui.label("Tags for this card (comma-separated):");
                    let mut tags_str = card.tags.join(", ");
                    let tags_resp = ui.text_edit_singleline(&mut tags_str);
                    if tags_resp.changed() {
                        card.tags = tags_str
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }

                    ui.add_space(6.0);

                    // Examples – one per line.
                    ui.label("Example sentences (one per line):");
                    let mut examples_text = card.examples.join("\n");
                    let examples_resp = ui.text_edit_multiline(&mut examples_text);
                    if examples_resp.changed() {
                        card.examples = examples_text
                            .lines()
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                    }

                    ui.add_space(6.0);
                });

                ui.add_space(10.0);
            }

            ui.add_space(8.0);

            // "Add a card" at the bottom.
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

done
}

/// Use the core import stack to parse a file into a Deck
/// and convert its cards into BuilderCards.
///
/// Imported cards are **appended** to the existing list; they do not
/// clear or overwrite cards already created in the builder.
fn import_deck_into_builder(path: &Path, state: &mut DeckBuilderState) -> Result<(), String> {
    let deck = import::import_deck_file(path)
        .map_err(|e| format!("Failed to import deck from {:?}: {e}", path))?;

    // If the builder has no title yet, adopt the deck's name.
    if state.file_name.trim().is_empty() && !deck.name.trim().is_empty() {
        state.file_name = deck.name.clone();
    }

    // Optional: if the builder has no tags yet, seed from description.
    if state.tags.trim().is_empty() {
        if let Some(desc) = &deck.description {
            state.tags = desc.clone();
        }
    }

    // APPEND imported cards instead of clearing existing ones.
    for src in deck.cards {
        let mut card = BuilderCard::default();
        card.term = src.term;
        card.definition = src.definition;

        // TODO: when Deck/Card support languages/tags/examples/media/notes,
        // copy them across here as needed.
        // card.tags = src.tags.clone();
        // card.examples = src.examples.clone();
        // card.media_path = src.media.unwrap_or_default();

        state.cards.push(card);
    }

    Ok(())
}

/// Import a deck or list of cards from a file into the current state.
///
/// - `.json`: loads a full `DeckBuilderState` (replaces current state)
/// - anything else: uses `crate::import::import_deck_file` and maps `Deck` → `BuilderCard`
fn import_deck_from_file(path: &Path, state: &mut DeckBuilderState) -> Result<(), String> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    match ext.as_str() {
        "json" => import_from_json(path, state),
        _ => import_deck_into_builder(path, state),
    }
}

fn language_combo(
    ui: &mut egui::Ui,
    combo_id: impl std::hash::Hash,
    label: &str,
    value: &mut String,
    opts: &DeckBuilderOptions,
) {
    ui.label(label);

    let enabled_langs: Vec<_> = opts.languages.iter().filter(|l| l.enabled).collect();

    // Fallback if nothing is enabled: just show the raw text field.
    if enabled_langs.is_empty() {
        ui.text_edit_singleline(value);
        return;
    }

    let current_label = if value.is_empty() {
        "Select…".to_string()
    } else if let Some(lang) = enabled_langs.iter().find(|lang| lang.code == *value) {
        format!("{} ({})", lang.name, lang.code)
    } else {
        value.clone()
    };

    egui::ComboBox::from_id_source(combo_id)
        .selected_text(current_label)
        .show_ui(ui, |ui| {
            for lang in &enabled_langs {
                let label = format!("{} ({})", lang.name, lang.code);
                ui.selectable_value(value, lang.code.clone(), label);
            }
        });
}

/// Per-card media widget – click OR drag-and-drop to choose media.
fn card_media_widget(ui: &mut egui::Ui, media_path: &mut String) {
    ui.label("Image / media");

    let display_text: &str = if media_path.is_empty() {
        "Click or drag a file here\n(image / GIF / video)"
    } else {
        media_path.as_str()
    };

    let drop_response = ui
        .add_sized(
            egui::vec2(220.0, 110.0),
            egui::Button::new(display_text).wrap(),
        )
        .on_hover_text("Drop a file here or click to browse");

    let mut open_file_dialog = drop_response.clicked();

    // Drag & drop support.
    let dropped_files = ui.ctx().input(|i| i.raw.dropped_files.clone());
    if drop_response.hovered() && !dropped_files.is_empty() {
        if let Some(file) = dropped_files.first() {
            if let Some(path) = &file.path {
                *media_path = path.to_string_lossy().to_string();
            }
        }
    }

    ui.add_space(4.0);

    // Explicit "Browse…" button as an alternative.
    if ui.button("Browse…").clicked() {
        open_file_dialog = true;
    }

    if open_file_dialog {
        if let Some(path) = FileDialog::new()
            .add_filter("Media", &["png", "jpg", "jpeg", "gif", "mp4", "webm"])
            .pick_file()
        {
            *media_path = path.to_string_lossy().to_string();
        }
    }
}

/// Save the current deck to `decks/<safe_name>.json`.
/// Returns `true` on success.
fn save_deck_to_disk(state: &DeckBuilderState) -> bool {
    let decks_dir = Path::new("decks");
    if let Err(e) = fs::create_dir_all(decks_dir) {
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
            if let Err(e) = fs::write(&path, json) {
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

fn import_from_json(path: &Path, state: &mut DeckBuilderState) -> Result<(), String> {
    let text =
        fs::read_to_string(path).map_err(|e| format!("Failed to read JSON file: {e}"))?;

    let loaded: DeckBuilderState =
        serde_json::from_str(&text).map_err(|e| format!("Failed to parse JSON: {e}"))?;

    *state = loaded;

    // If file_name is empty, derive it from the file name.
    if state.file_name.trim().is_empty() {
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            state.file_name = stem.to_string();
        }
    }

    Ok(())
}
