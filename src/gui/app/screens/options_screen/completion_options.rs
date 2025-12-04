// src/gui/app/screens/options_screen/completion_options.rs
use eframe::egui;

#[derive(Debug, Clone)]
pub struct CompletionOptions {
    /// Show a little stats summary (X/Y correct, accuracy %) on completion.
    pub show_stats: bool,

    /// Automatically return to the main menu / deck list after a short delay.
    pub auto_return_enabled: bool,

    /// Delay (in seconds) before auto return, if enabled.
    pub auto_return_delay_secs: f32,

    /// Play a fun "celebration" animation (confetti, sparkles, etc.)
    /// when the deck is completed.
    pub celebration_enabled: bool,
}

impl Default for CompletionOptions {
    fn default() -> Self {
        Self {
            show_stats: true,
            auto_return_enabled: false,
            auto_return_delay_secs: 3.0,
            celebration_enabled: true,
        }
    }
}

/// Draw the "Completion" section inside the Options screen.
pub fn draw_completion_options_section(
    ui: &mut egui::Ui,
    completion: &mut CompletionOptions,
) {
    ui.heading("Completion");
    ui.add_space(8.0);

    ui.vertical(|ui| {
        ui.checkbox(&mut completion.show_stats, "Show stats summary");
        ui.checkbox(
            &mut completion.celebration_enabled,
            "Play celebration animation on completion",
        );

        ui.add_space(8.0);
        ui.checkbox(
            &mut completion.auto_return_enabled,
            "Automatically return to main menu / deck list",
        );

        if completion.auto_return_enabled {
            ui.horizontal(|ui| {
                ui.label("Auto-return delay (seconds):");
                ui.add(
                    egui::DragValue::new(&mut completion.auto_return_delay_secs)
                        .range(1.0..=30.0)
                        .speed(0.1),
                );
            });
            ui.label("Tip: shorter delays keep you moving; longer delays let you enjoy the victory.");
        } else {
            ui.label("You will stay on the completion screen until you click a button.");
        }
    });
}
