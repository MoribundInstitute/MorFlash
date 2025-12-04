// src/gui/app/screens/controls_screen.rs

use eframe::egui;

/// Draw the Controls screen.
/// 
/// Returns `true` if the caller should exit back to the main menu.
pub fn draw_controls_screen(ui: &mut egui::Ui, ctx: &egui::Context) -> bool {
    // If ESC is pressed, immediately exit.
    let esc_pressed = ctx.input(|i| i.key_pressed(egui::Key::Escape));
    if esc_pressed {
        return true;
    }

    ui.heading("Controls");
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            // =========================
            // GENERAL UI
            // =========================
            ui.heading("General UI");
            ui.add_space(4.0);
            ui.label("• Ctrl + Scrollwheel — Zoom UI in/out");
            ui.label("• Ctrl + 0 — Reset zoom to normal");
            ui.label("• Ctrl + '+' / Ctrl + '–' (optional) — Keyboard zoom");
            ui.label("• Middle-mouse drag / Right-click drag — Pan view (optional)");
            ui.add_space(12.0);

            ui.separator();
            ui.add_space(8.0);

            // =========================
            // NAVIGATION / MENUS
            // =========================
            ui.heading("Navigation / Menus");
            ui.add_space(4.0);
            ui.label("• Up / Down Arrow — Move selection");
            ui.label("• Enter — Select");
            ui.label("• Esc — Go back");
            ui.add_space(12.0);

            ui.separator();
            ui.add_space(8.0);

            // =========================
            // STUDY MODE
            // =========================
            ui.heading("Study Mode");
            ui.add_space(4.0);
            ui.label("• Click correct meaning — Answer the card");
            ui.label("• Esc — Return to deck list");
            ui.label("• Space / Enter (optional) — Submit");
            ui.label("• ← / → (optional) — Next/Previous card");
            ui.add_space(12.0);

            ui.separator();
            ui.add_space(8.0);

            // =========================
            // DECK BUILDER
            // =========================
            ui.heading("Deck Builder");
            ui.add_space(4.0);
            ui.label("• Ctrl + S — Save & Exit");
            ui.label("• Ctrl + N — Add new card");
            ui.label("• Del / Backspace — Remove selected card");
            ui.label("• Drag & Drop — Reorder cards (future)");
            ui.label("• Click media box — Attach media");
            ui.label("• Drag media file — Attach file");
            ui.add_space(12.0);

            ui.separator();
            ui.add_space(8.0);

            // =========================
            // OPTIONS
            // =========================
            ui.heading("Options Screen");
            ui.add_space(4.0);
            ui.label("• Sound toggles");
            ui.label("• Background/theme selection");
            ui.label("• Card color settings");
            ui.label("• Debug mode");
            ui.label("• Language preferences (future)");
            ui.add_space(16.0);
        });

    ui.separator();
    ui.add_space(8.0);

    // Back button at the bottom
    if ui.button("Back to Main Menu").clicked() {
        return true;
    }

    false
}
