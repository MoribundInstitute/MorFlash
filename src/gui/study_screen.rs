// src/gui/study_screen.rs
use eframe::egui;

use crate::model::Card;

/// Draw the study screen.
///
/// Returns:
/// - Some(term) if the user clicked an answer
/// - back_to_list = true if they clicked the “Back to deck list” button
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
) -> (Option<String>, bool) {
    let mut clicked_term: Option<String> = None;
    let mut back_to_list = false;

    let available = ui.available_size();

    // vertical padding so card sits nicely in the middle-ish
    ui.add_space(available.y * 0.08);

    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        let card_width = available.x.min(1000.0);

        let frame = egui::Frame::none()
            .fill(egui::Color32::from_rgb(20, 22, 60)) // inner “card” color
            .rounding(egui::Rounding::same(24.0))
            .inner_margin(egui::Margin::same(32.0))
            .shadow(egui::epaint::Shadow {
                offset: egui::vec2(0.0, 16.0),
                blur: 32.0,
                spread: 0.0,
                color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180),
            });

        frame.show(ui, |ui| {
            ui.set_width(card_width);

            if let Some(card) = current_card {
                // Definition at the top
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new("Definition:")
                            .size(22.0)
                            .color(egui::Color32::from_rgb(236, 236, 235)),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(&card.definition)
                            .size(32.0)
                            .color(egui::Color32::from_rgb(236, 236, 235)),
                    );
                });

                ui.add_space(40.0);

                ui.label(
                    egui::RichText::new("Choose an answer:")
                        .size(18.0)
                        .color(egui::Color32::from_rgb(236, 236, 235)),
                );
                ui.add_space(16.0);

                let button_width = card_width * 0.45;
                let button_height = 56.0;

                // 2x2 grid of answers near the bottom of the card
                egui::Grid::new("answer-grid")
                    .num_columns(2)
                    .spacing(egui::vec2(24.0, 20.0))
                    .min_col_width(button_width)
                    .show(ui, |ui| {
                        for (idx, opt) in options.iter().enumerate() {
                            let term_str = opt.term.as_str();

                            let stroke = if Some(term_str) == correct_term {
                                // correct = glowing green
                                egui::Stroke::new(
                                    4.0,
                                    egui::Color32::from_rgb(80, 200, 120),
                                )
                            } else if Some(term_str) == wrong_term {
                                // wrong = glowing red
                                egui::Stroke::new(
                                    4.0,
                                    egui::Color32::from_rgb(220, 80, 80),
                                )
                            } else {
                                // neutral purple outline
                                egui::Stroke::new(
                                    2.0,
                                    egui::Color32::from_rgb(180, 180, 230),
                                )
                            };

                            let button = egui::Button::new(
                                egui::RichText::new(&opt.term)
                                    .size(22.0)
                                    .color(egui::Color32::from_rgb(0, 0, 0)),
                            )
                            .min_size(egui::vec2(button_width, button_height))
                            .fill(egui::Color32::from_rgb(163, 165, 219)) // purple fill
                            .stroke(stroke)
                            .rounding(egui::Rounding::same(12.0));

                            let resp = ui.add(button);
                            if resp.clicked() {
                                clicked_term = Some(opt.term.clone());
                            }

                            if idx % 2 == 1 {
                                ui.end_row();
                            }
                        }
                    });

                ui.add_space(24.0);

                if !feedback.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new(feedback)
                                .size(20.0)
                                .color(egui::Color32::from_rgb(236, 236, 235)),
                        );
                    });
                }

                ui.add_space(24.0);

                // progress bar at the bottom of the card
                if total > 0 {
                    let bar = egui::ProgressBar::new(progress)
                        .desired_width(card_width - 40.0)
                        .text(format!("{reviewed}/{total}"));
                    ui.add(bar);
                }

                ui.add_space(16.0);

                // Back to deck list button, aligned to the right
                ui.horizontal(|ui| {
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            let back_button = egui::Button::new("Back to deck list")
                                .min_size(egui::vec2(180.0, 36.0));
                            if ui.add(back_button).clicked() {
                                back_to_list = true;
                            }
                        },
                    );
                });
            } else {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(feedback)
                            .size(24.0)
                            .color(egui::Color32::from_rgb(236, 236, 235)),
                    );
                    ui.add_space(16.0);
                    let back_button = egui::Button::new("Back to deck list")
                        .min_size(egui::vec2(180.0, 36.0));
                    if ui.add(back_button).clicked() {
                        back_to_list = true;
                    }
                });
            }
        });
    });

    (clicked_term, back_to_list)
}
