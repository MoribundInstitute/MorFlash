// src/gui/app/main_menu_screen.rs
use eframe::egui;
use egui::{Color32, FontId, RichText, TextureHandle};
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
    mor_button_tex: Option<&TextureHandle>,
    critter_tex: Option<&TextureHandle>, // NEW
) -> MainMenuAction {
    MenuTheme::apply_to_ctx(ui.ctx());

    let mut action = MainMenuAction::None;
    let mut critter_target: Option<egui::Rect> = None;

    let available = ui.available_size();
    let panel_width = (available.x * 0.7).clamp(600.0, 1000.0);

    ui.vertical_centered(|ui| {
        ui.add_space(32.0);

        ui.label(
            RichText::new("MorFlash")
                .font(FontId::proportional(48.0))
                .strong(),
        );
        ui.label(RichText::new("Main Menu").font(FontId::proportional(26.0)));

        ui.add_space(28.0);

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

                    for (idx, path) in deck_paths.iter().take(3).enumerate() {
                        let name = path
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string();

                        let deck_min_width = 240.0;

                        let (response, rect) = draw_menu_button(
                            ui,
                            &name,
                            mor_button_tex,
                            deck_min_width,
                        );

                        // "Most recent input": hover wins, otherwise focus_index
                        let is_active = if response.hovered() {
                            true
                        } else {
                            idx == focus_index
                        };

                        if is_active {
                            critter_target = Some(rect);
                        }

                        if response.clicked() {
                            action = MainMenuAction::OpenDeck(path.clone());
                        }

                        ui.add_space(10.0);
                    }
                });
            });

        ui.add_space(32.0);

        let deck_count = deck_paths.len().min(3);
        let options_index = deck_count;

        let (options_response, options_rect) = draw_menu_button(
            ui,
            "⚙  Options",
            mor_button_tex,
            220.0,
        );

        let options_active = if options_response.hovered() {
            true
        } else {
            focus_index == options_index
        };

        if options_active {
            critter_target = Some(options_rect);
        }

        if options_response.clicked() {
            action = MainMenuAction::OpenOptions;
        }

        ui.add_space(16.0);

        ui.label(
            RichText::new("Tip: you can adjust sound and other settings in Options.")
                .font(FontId::proportional(16.0)),
        );
    });

    // Draw critter sprite next to the active button (on top of everything)
    if let (Some(tex), Some(target)) = (critter_tex, critter_target) {
        let size = egui::vec2(48.0, 48.0);
        // Right side of the button; change to left if you prefer
        let pos = egui::pos2(
    target.left() - size.x - 12.0,
    target.center().y - size.y / 2.0,
);
        let rect = egui::Rect::from_min_size(pos, size);

        ui.painter().image(
            tex.id(),
            rect,
            egui::Rect::from_min_max(
                egui::pos2(0.0, 0.0),
                egui::pos2(1.0, 1.0),
            ),
            Color32::WHITE,
        );
    }

    action
}

// returns (Response, button_rect)
fn draw_menu_button(
    ui: &mut egui::Ui,
    label: &str,
    mor_button_tex: Option<&TextureHandle>,
    min_width: f32,
) -> (egui::Response, egui::Rect) {
    let font_id = FontId::proportional(22.0);

    if mor_button_tex.is_none() {
        let text = RichText::new(label.to_string()).font(font_id);
        let button = egui::Button::new(text).min_size(egui::vec2(min_width, 44.0));
        let response = ui.add(button);
        let rect = response.rect;
        return (response, rect);
    }

    let tex = mor_button_tex.unwrap();

    let galley = ui.fonts(|f| {
        f.layout_no_wrap(
            label.to_string(),
            font_id.clone(),
            Color32::WHITE,
        )
    });
    let sz = galley.size();

    let padding = egui::vec2(24.0, 8.0);
    let desired = egui::vec2(
        sz.x.max(min_width) + 2.0 * padding.x,
        sz.y + 2.0 * padding.y,
    );

    let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
    let painter = ui.painter();

    painter.image(
        tex.id(),
        rect,
        egui::Rect::from_min_max(
            egui::pos2(0.0, 0.0),
            egui::pos2(1.0, 1.0),
        ),
        Color32::WHITE,
    );

    let text_pos = egui::pos2(
        rect.center().x - sz.x / 2.0,
        rect.center().y - sz.y / 2.0,
    );

    painter.galley(text_pos, galley, Color32::WHITE);

    (response, rect)
}
