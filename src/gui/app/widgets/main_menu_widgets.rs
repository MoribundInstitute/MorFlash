use eframe::egui::{self, RichText, Ui};
use crate::gui::app::widgets::shared;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainMenuAction {
    None,
    Start,
    Options,
    Quit,
}

/// Renders the complete main menu and returns the chosen action
pub fn main_menu_layout(ui: &mut Ui) -> MainMenuAction {
    let mut action = MainMenuAction::None;

    // Perfectly center everything both vertically and horizontally
    ui.centered_and_justified(|ui| {
        ui.vertical(|ui| {
            // Title area
            title_block(ui);

            ui.add_space(64.0);

            // Buttons – fixed reasonable width for consistency
            let button_width = 360.0;
            ui.set_min_width(button_width);

            if shared::primary_button(ui, "Start Reviewing").clicked() {
                action = MainMenuAction::Start;
            }

            ui.add_space(16.0);

            if shared::primary_button(ui, "Options").clicked() {
                action = MainMenuAction::Options;
            }

            ui.add_space(16.0);

            if shared::ghost_button(ui, "Quit").clicked() {
                action = MainMenuAction::Quit;
            }
        });
    });

    action
}

/// Title + subtitle block with nice styling
fn title_block(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);

        ui.heading(
            RichText::new("MorFlash")
                .size(56.0)
                .strong(),
        );

        ui.add_space(8.0);

        ui.label(
            RichText::new("Offline vocabulary trainer")
                .size(21.0)
                .italics()
                .color(ui.visuals().weak_text_color()),
        );

        ui.add_space(20.0);
    });
}