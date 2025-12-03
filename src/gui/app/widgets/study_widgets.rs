use eframe::egui;
use crate::gui::app::theme::Theme;
use crate::gui::app::widgets::shared;
use crate::model::Card;

/// What the study UI reports back to the screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudyAction {
    None,
    ChoseTerm, // actual term returned separately
    BackToDeckList,
}

/// Main study layout — returns (clicked_term, action)
pub fn study_layout(
    ui: &mut egui::Ui,
    current_card: Option<&Card>,
    options: &[Card],
    correct_term: Option<&str>,
    wrong_term: Option<&str>,
    feedback: &str,
    progress: f32,
    reviewed: usize,
    total: usize,
) -> (Option<String>, StudyAction) {
    let mut clicked_term = None;
    let mut action = StudyAction::None;

    ui.vertical_centered(|ui| {
        ui.add_space(20.0);

        // ── Card ──
        study_card(ui, current_card);

        ui.add_space(24.0);

        // ── Answer grid ──
        if let Some(term) = answer_grid(ui, options) {
            clicked_term = Some(term);
            action = StudyAction::ChoseTerm;
        }

        ui.add_space(20.0);

        // ── Feedback ──
        feedback_label(ui, feedback, correct_term, wrong_term);

        ui.add_space(32.0);

        // ── Footer (progress + counter + back button) ──
        if study_footer(ui, progress, reviewed, total).back_clicked {
            action = StudyAction::BackToDeckList;
        }
    });

    (clicked_term, action)
}

/// Big central card with definition
pub fn study_card(ui: &mut egui::Ui, current_card: Option<&Card>) {
    let text = current_card
        .map(|c| c.definition.clone())
        .unwrap_or_else(|| "No more cards!".to_string());

    let frame = egui::Frame::none()
        .fill(Theme::CARD_BG)
        .rounding(egui::Rounding::same(20.0))
        .inner_margin(egui::Margin::symmetric(32.0, 40.0))
        .stroke(egui::Stroke::new(2.0, Theme::CARD_OUTLINE))
        .shadow(egui::epaint::Shadow::big_dark());

    frame.show(ui, |ui| {
        ui.set_min_width(420.0);
        ui.vertical_centered(|ui| {
            ui.label(egui::RichText::new(text).size(24.0).strong());
        });
    });
}

/// 2×2 (or 1×4) answer grid
pub fn answer_grid(ui: &mut egui::Ui, options: &[Card]) -> Option<String> {
    if options.is_empty() {
        ui.centered_and_justified(|ui| {
            ui.label(egui::RichText::new("No options available").italics());
        });
        return None;
    }

    let mut clicked = None;
    let chunk_size = if options.len() <= 2 { 1 } else { 2 };

    ui.vertical(|ui| {
        for chunk in options.chunks(chunk_size) {
            ui.horizontal(|ui| {
                let button_width = (ui.available_width() / chunk.len() as f32) - 8.0;
                for card in chunk {
                    let resp = ui.add_sized(
                        [button_width, 64.0],
                        egui::Button::new(
                            egui::RichText::new(&card.term)
                                .size(18.0)
                                .strong(),
                        )
                        .fill(Theme::BUTTON_BG)
                        .rounding(egui::Rounding::same(14.0))
                        .stroke(egui::Stroke::new(1.5, Theme::BUTTON_OUTLINE)),
                    );

                    if resp.clicked() {
                        clicked = Some(card.term.clone());
                    }
                }
            });
            if chunk.len() < options.len() {
                ui.add_space(12.0);
            }
        }
    });

    clicked
}

/// Feedback label with smart coloring
pub fn feedback_label(
    ui: &mut egui::Ui,
    feedback: &str,
    correct_term: Option<&str>,
    wrong_term: Option<&str>,
) {
    if feedback.is_empty() {
        return;
    }

    let is_correct = correct_term.is_some();
    let color = if is_correct {
        Theme::SUCCESS
    } else if wrong_term.is_some() {
        Theme::ERROR
    } else {
        Theme::TEXT_SECONDARY
    };

    let mut text = feedback.to_owned();
    if let (Some(wrong), Some(correct)) = (wrong_term, correct_term) {
        text = text.replace("{wrong}", wrong).replace("{correct}", correct);
    }

    ui.centered_and_justified(|ui| {
        ui.label(
            egui::RichText::new(text)
                .size(20.0)
                .strong()
                .color(color),
        );
    });
}

/// Footer with progress bar, counter, and back button
pub struct FooterResponse {
    pub back_clicked: bool,
}

pub fn study_footer(
    ui: &mut egui::Ui,
    progress: f32,
    reviewed: usize,
    total: usize,
) -> FooterResponse {
    let mut back_clicked = false;

    ui.horizontal(|ui| {
        // Left: progress bar
        shared::progress_bar(ui, progress);

        // Center: counter
        ui.centered_and_justified(|ui| {
            ui.label(
                egui::RichText::new(format!("{reviewed}/{total}"))
                    .size(15.0)
                    .strong(),
            );
        });

        // Right: back button
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if shared::back_to_deck_list_button(ui).clicked() {
                back_clicked = true;
            }
        });
    });

    FooterResponse { back_clicked }
}