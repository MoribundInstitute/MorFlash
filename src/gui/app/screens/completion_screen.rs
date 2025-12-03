// src/gui/app/screens/completion_screen.rs
use eframe::egui;
use std::time::Instant;

use crate::gui::app::screens::options_screen::CompletionOptions;

#[derive(Debug, Clone)]
pub struct CompletionState {
    pub celebration_played: bool,
    pub total_reviewed: u32,
    pub correct_count: u32,
    pub incorrect_count: u32,
    pub new_count: u32,

    pub started_at: Option<Instant>,
    pub finished_at: Option<Instant>,

    pub auto_return_enabled: bool,
    pub auto_return_secs: f32,
    pub auto_return_deadline: Option<Instant>,
}

impl Default for CompletionState {
    fn default() -> Self {
        Self {
            celebration_played: false,
            total_reviewed: 0,
            correct_count: 0,
            incorrect_count: 0,
            new_count: 0,
            started_at: None,
            finished_at: None,
            auto_return_enabled: false,
            auto_return_secs: 5.0,
            auto_return_deadline: None,
        }
    }
}

/// Draw the completion screen.
///
/// - Draws the tiling background texture if provided.
/// - Triggers the celebration sound exactly once per session via `on_play_celebration`.
/// - Returns `true` if the user requests to go back to the deck list.
pub fn draw_completion_screen<F>(
    ui: &mut egui::Ui,
    state: &mut CompletionState,
    _completion_opts: &CompletionOptions,
    bg_texture: Option<&egui::TextureHandle>,
    mut on_play_celebration: F,
) -> bool
where
    F: FnMut(),
{
    let mut go_back = false;

    // === Draw global tiling background, if available ===
    if let Some(tex) = bg_texture {
        let rect = ui.max_rect();
        let painter = ui.painter_at(rect);

        let tex_size = tex.size(); // [w, h]

        // Draw the background using UVs > 1.0 to tile it across the screen.
        painter.image(
            tex.id(),
            rect,
            egui::Rect {
                min: egui::pos2(0.0, 0.0),
                max: egui::pos2(
                    rect.width() / tex_size[0] as f32,
                    rect.height() / tex_size[1] as f32,
                ),
            },
            egui::Color32::WHITE,
        );
    }

    // === Trigger the celebration sound exactly once ===
    if !state.celebration_played {
        on_play_celebration();
        state.celebration_played = true;
    }

    // === Foreground UI ===
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        ui.add_space(40.0);

        ui.heading("Session Complete! 🎉");
        ui.add_space(12.0);

        ui.label("You've reviewed all due cards for now.");
        ui.add_space(24.0);

        egui::Frame::group(ui.style())
            .rounding(egui::Rounding::same(8.0))
            .fill(ui.visuals().extreme_bg_color)
            .show(ui, |ui| {
                ui.set_min_width(260.0);
                ui.add_space(8.0);

                ui.vertical_centered(|ui| {
                    ui.label("📊 Session Summary");
                    ui.add_space(4.0);

                    ui.label(format!("Total reviewed: {}", state.total_reviewed));
                    ui.label(format!(
                        "Correct: {}  |  Incorrect: {}",
                        state.correct_count, state.incorrect_count
                    ));

                    if state.new_count > 0 {
                        ui.label(format!("New cards: {}", state.new_count));
                    }

                    if state.total_reviewed > 0 {
                        let acc =
                            (state.correct_count as f32 / state.total_reviewed as f32) * 100.0;
                        ui.label(format!("Accuracy: {:.1}%", acc));
                    } else {
                        ui.small("No stats yet.");
                    }

                    if state.auto_return_enabled {
                        ui.add_space(8.0);
                        ui.small(format!(
                            "Auto-return enabled (≈ {:.1} seconds)…",
                            state.auto_return_secs
                        ));
                    }
                });

                ui.add_space(8.0);
            });

        ui.add_space(32.0);

        let back_button =
            ui.add(egui::Button::new("← Back to Deck List").min_size(egui::vec2(240.0, 44.0)));

        if back_button.clicked() {
            go_back = true;
        }

        ui.add_space(20.0);
    });

    go_back
}
