// src/gui/app/main_menu_screen.rs
use eframe::egui;
use egui::{FontId, RichText};
use std::path::PathBuf;

use crate::gui::menu_theme::MenuTheme;

#[derive(Debug)]
pub enum MainMenuAction {
    None,
    OpenDeck(PathBuf),
    OpenOptions,
}

pub fn draw_main_menu(
    ui: &mut egui::Ui,
    deck_paths: &[PathBuf],
    focus_index: usize,
) -> MainMenuAction {
    // Apply the PC-98 style menu theme to this screen
    MenuTheme::apply_to_ctx(ui.ctx());

    let mut action = MainMenuAction::None;

    let available = ui.available_size();
    let panel_width = (available.x * 0.7).clamp(600.0, 1000.0);

    ui.vertical_centered(|ui| {
        ui.add_space(32.0);

        // ===== Title =====
        ui.label(
            RichText::new("MorFlash")
                .font(FontId::proportional(48.0))
                .strong(),
        );
        ui.label(RichText::new("Main Menu").font(FontId::proportional(26.0)));

        ui.add_space(28.0);

        // ===== Main strip / panel =====
        egui::Frame::none()
            .fill(MenuTheme::PANEL_BG)
            .stroke(egui::Stroke::new(1.5, MenuTheme::BUTTON_OUTLINE))
            .rounding(egui::Rounding::same(18.0))
            .inner_margin(egui::Margin {
                left: 32.0,
                right: 32.0,
                top: 24.0,
                bottom: 32.0,
            })
            .show(ui, |ui| {
                ui.set_width(panel_width);

                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Choose a deck").font(FontId::proportional(24.0)));
                    ui.add_space(18.0);

                    // Up to 3 decks for now, laid out in a column
                    for (idx, path) in deck_paths.iter().take(3).enumerate() {
                        let name = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        let is_focused = idx == focus_index;

                        // Base text
                        let mut text = RichText::new(name).font(FontId::proportional(22.0));

                        // Highlight focused entry with stronger style
                        if is_focused {
                            text = text.strong().underline();
                        }

                        let button =
                            egui::Button::new(text).min_size(egui::vec2(panel_width * 0.5, 44.0));

                        if ui.add(button).clicked() {
                            action = MainMenuAction::OpenDeck(path.clone());
                        }

                        ui.add_space(10.0);
                    }
                });
            });

        ui.add_space(32.0);

        // ===== Options button =====
        let deck_count = deck_paths.len().min(3);
        let options_index = deck_count; // Options lives "after" the last deck

        let mut options_text = RichText::new("⚙  Options").font(FontId::proportional(24.0));

        if focus_index == options_index {
            options_text = options_text.strong().underline();
        }

        let options_button = egui::Button::new(options_text).min_size(egui::vec2(220.0, 46.0));

        if ui.add(options_button).clicked() {
            action = MainMenuAction::OpenOptions;
        }

        ui.add_space(16.0);

        // ===== Tip text =====
        ui.label(
            RichText::new("Tip: you can adjust sound and other settings in Options.")
                .font(FontId::proportional(16.0)),
        );
    });

    action
}
