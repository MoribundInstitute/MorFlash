use eframe::egui;

use super::completion_screen;
use crate::gui::theme::Theme;
use crate::model::Card;

/// Shared signature:
/// - returns (clicked_term, back_to_list)
type StudyResult = (Option<String>, bool);

/// Convenience wrapper:
pub fn draw_study_screen(
    ui: &mut egui::Ui,
    current_card: Option<&Card>,
    options: &[Card],
    correct_term: Option<&str>,
    wrong_term: Option<&str>,
    feedback: &str,
    progress: f32,
    reviewed: usize,
    total: usize,
) -> StudyResult {
    // We no longer care about fullscreen/windowed here;
    // the outer `egui::Window` in app/mod.rs handles size.
    draw_study_screen_inner(
        ui,
        current_card,
        options,
        correct_term,
        wrong_term,
        feedback,
        progress,
        reviewed,
        total,
    )
}

fn draw_study_screen_inner(
    ui: &mut egui::Ui,
    current_card: Option<&Card>,
    options: &[Card],
    correct_term: Option<&str>,
    wrong_term: Option<&str>,
    feedback: &str,
    progress: f32,
    reviewed: usize,
    total: usize,
) -> StudyResult {
    let mut clicked_term: Option<String> = None;
    let mut back_to_list = false;

    // ----------------------------------------------------
    // Keyboard shortcuts: 1 / 2 / 3 / 4
    // ----------------------------------------------------
    let mut number_pressed: Option<usize> = None;
    ui.ctx().input(|i| {
        if i.key_pressed(egui::Key::Num1) {
            number_pressed = Some(0);
        }
        if i.key_pressed(egui::Key::Num2) {
            number_pressed = Some(1);
        }
        if i.key_pressed(egui::Key::Num3) {
            number_pressed = Some(2);
        }
        if i.key_pressed(egui::Key::Num4) {
            number_pressed = Some(3);
        }
    });

    let available = ui.available_size();
    let card_width = Theme::card_width(available.x);
    let button_size = Theme::answer_button_size(card_width);

    // Center content within the window
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.set_width(card_width);

        if let Some(card) = current_card {
            // =======================
            // Definition header
            // =======================
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("Definition:")
                        .size(22.0)
                        .color(Theme::CARD_TEXT),
                );
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(&card.definition)
                        .size(32.0)
                        .color(Theme::CARD_TEXT),
                );
            });

            ui.add_space(40.0);

            ui.label(
                egui::RichText::new("Choose an answer:")
                    .size(18.0)
                    .color(Theme::CARD_TEXT),
            );
            ui.add_space(16.0);

            // =======================
            // 2x2 answer grid
            // =======================
            egui::Grid::new("answer-grid")
                .num_columns(2)
                .spacing(egui::vec2(24.0, 20.0))
                .show(ui, |ui| {
                    for (idx, opt) in options.iter().enumerate() {
                        let term_str = opt.term.as_str();

                        let outline_color = if Some(term_str) == correct_term {
                            Theme::CORRECT_OUTLINE
                        } else if Some(term_str) == wrong_term {
                            Theme::WRONG_OUTLINE
                        } else {
                            Theme::BUTTON_OUTLINE
                        };

                        let label = egui::RichText::new(&opt.term)
                            .size(22.0)
                            .color(Theme::BUTTON_TEXT);

                        let button = egui::Button::new(label)
                            .min_size(button_size)
                            .fill(Theme::BUTTON_FILL)
                            .stroke(egui::Stroke::new(2.0, outline_color))
                            .rounding(egui::Rounding::same(12.0));

                        let resp = ui.add(button);

                        // Mouse click
                        if resp.clicked() {
                            clicked_term = Some(opt.term.clone());
                        }

                        // Keyboard press (1–4)
                        if let Some(n) = number_pressed {
                            if n == idx {
                                clicked_term = Some(opt.term.clone());
                            }
                        }

                        if idx % 2 == 1 {
                            ui.end_row();
                        }
                    }
                });

            ui.add_space(24.0);

            // =======================
            // Feedback text
            // =======================
            if !feedback.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(feedback)
                            .size(20.0)
                            .color(Theme::CARD_TEXT),
                    );
                });
            }

            ui.add_space(24.0);

            // =======================
            // Progress bar
            // =======================
            if total > 0 {
                let bar = egui::ProgressBar::new(progress)
                    .desired_width(card_width - 40.0)
                    .text(format!("{reviewed}/{total}"));
                ui.add(bar);
            }

            ui.add_space(16.0);

            // =======================
            // Back to deck button
            // =======================
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let label = egui::RichText::new("← Back to deck list")
                        .size(18.0)
                        .color(Theme::BUTTON_TEXT);

                    let back_button = egui::Button::new(label)
                        .min_size(egui::vec2(200.0, 40.0))
                        .fill(Theme::BUTTON_FILL)
                        .stroke(egui::Stroke::new(2.0, Theme::BUTTON_OUTLINE))
                        .rounding(egui::Rounding::same(10.0));

                    if ui.add(back_button).clicked() {
                        back_to_list = true;
                    }
                });
            });
        } else {
            // =======================
            // Finished / no-card state
            // =======================
            back_to_list = completion_screen::draw_completion_screen(ui);
        }
    });

    (clicked_term, back_to_list)
}
