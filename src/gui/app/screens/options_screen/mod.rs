// src/gui/app/screens/options_screen/mod.rs

use eframe::egui;

// child modules in this folder
mod completion_options;
mod deck_builder_options;
mod global_options;
mod main_menu_options;
mod state;
mod study_options;

// re-export types so the rest of the app can `use options_screen::...`
pub use state::{
    BackgroundChoice, CardColorMode, FontChoice, OptionsState, SoundSlotConfig, SoundSource,
};

pub use completion_options::CompletionOptions;
pub use deck_builder_options::DeckBuilderOptions;
pub use global_options::GlobalOptions;
pub use main_menu_options::MainMenuOptions;
pub use study_options::StudyOptions;

use crate::gui::theme::MenuTheme;

/// Simple MorFlash-style button wrapper that can use the textured Mor button.
fn mor_button(
    ui: &mut egui::Ui,
    label: &str,
    min_width: f32,
    tex_opt: Option<&egui::TextureHandle>,
) -> egui::Response {
    if tex_opt.is_none() {
        return ui.add(
            egui::Button::new(label).min_size(egui::vec2(min_width, 36.0)),
        );
    }

    let tex = tex_opt.unwrap();
    let font_id = egui::FontId::proportional(20.0);

    let galley = ui.fonts(|f| {
        f.layout_no_wrap(label.to_owned(), font_id, egui::Color32::WHITE)
    });
    let sz = galley.size();
    let padding = egui::vec2(20.0, 6.0);

    let mut desired = sz + padding * 2.0;
    desired.x = desired.x.max(min_width);

    let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
    let painter = ui.painter();

    painter.image(
        tex.id(),
        rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        egui::Color32::WHITE,
    );

    let text_pos = rect.center() - sz * 0.5;
    painter.galley(text_pos, galley, egui::Color32::WHITE);

    response
}

/// Main Options screen entry point.
/// Returns `true` if the user pressed "Back".
pub fn draw_options(
    ui: &mut egui::Ui,
    state: &mut OptionsState,
    mor_button_tex: Option<&egui::TextureHandle>,
) -> bool {
    let mut back = false;

    // Apply global menu visuals (PC-98 style).
    MenuTheme::apply_to_ctx(ui.ctx());

    let avail = ui.available_size();
    let panel_width = (avail.x * 0.7).clamp(600.0, 900.0);

    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.heading("Options");
        ui.add_space(20.0);

        egui::Frame::none()
            .fill(MenuTheme::PANEL_BG)
            .stroke(egui::Stroke::new(1.5, MenuTheme::BUTTON_OUTLINE))
            .rounding(egui::Rounding::same(18.0))
            .inner_margin(egui::Margin::symmetric(32.0, 24.0))
            .show(ui, |ui| {
                ui.set_width(panel_width);

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            // Global (audio, debug, UI scale, etc.)
                            ui.group(|ui| {
                                global_options::draw_global_options_section(
                                    ui,
                                    &mut state.global,
                                );
                            });

                            ui.add_space(16.0);

                            // Study options (card colors etc.)
                            ui.group(|ui| {
                                study_options::draw_study_options_section(
                                    ui,
                                    &mut state.study,
                                );
                            });

                            ui.add_space(16.0);

                            // Completion options
                            ui.group(|ui| {
                                completion_options::draw_completion_options_section(
                                    ui,
                                    &mut state.completion,
                                );
                            });

                            ui.add_space(16.0);

                            // Main menu options
                            ui.group(|ui| {
                                main_menu_options::draw_main_menu_options_section(
                                    ui,
                                    &mut state.main_menu,
                                );
                            });

                            ui.add_space(16.0);

                            // Deck builder options
                            ui.group(|ui| {
                                deck_builder_options::draw_deck_builder_options_section(
                                    ui,
                                    &mut state.deck_builder,
                                );
                            });
                        });
                    });
            });

        ui.add_space(24.0);
        if mor_button(ui, "⬛ Back", 160.0, mor_button_tex).clicked() {
            back = true;
        }
    });

    back
}
