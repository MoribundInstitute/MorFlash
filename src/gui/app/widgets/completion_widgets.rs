use eframe::egui;
use crate::gui::app::widgets::shared;
use crate::gui::app::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionAction {
    None,
    BackToDecks,
    StudyAgain,
}

pub fn completion_layout(
    ui: &mut egui::Ui,
    reviewed: usize,
    total: usize,
) -> CompletionAction {
    let mut action = CompletionAction::None;

    ui.vertical_centered(|ui| {
        ui.add_space(40.0);

        completion_card(ui, reviewed, total);

        ui.add_space(20.0);

        ui.horizontal_centered(|ui| {
            if shared::primary_button(ui, "Study again").clicked() {
                action = CompletionAction::StudyAgain;
            }

            ui.add_space(10.0);

            if shared::ghost_button(ui, "Back to deck list").clicked() {
                action = CompletionAction::BackToDecks;
            }
        });
    });

    action
}

pub fn completion_card(ui: &mut egui::Ui, reviewed: usize, total: usize) {
    let frame = egui::Frame::none()
        .fill(Theme::CARD_BG)
        .rounding(egui::Rounding::same(18.0))
        .inner_margin(egui::Margin::symmetric(32.0, 24.0))
        .stroke(egui::Stroke::new(1.0, Theme::CARD_OUTLINE));

    frame.show(ui, |ui| {
        ui.heading("Set complete!");
        ui.add_space(8.0);
        ui.label(format!("You reviewed {}/{} cards.", reviewed, total));
    });
}
