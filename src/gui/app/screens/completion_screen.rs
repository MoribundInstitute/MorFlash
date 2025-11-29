use eframe::egui;

/// UI for “No more cards due”.
/// Returns: should_go_back_to_deck_list
pub fn draw_completion_screen(ui: &mut egui::Ui) -> bool {
    let mut go_back = false;

    ui.vertical_centered(|ui| {
        ui.add_space(16.0);

        ui.heading("No more cards due right now. 🎉");

        ui.add_space(16.0);

        let button =
            ui.add(egui::Button::new("← Back to deck list").min_size(egui::vec2(180.0, 36.0)));

        if button.clicked() {
            go_back = true;
        }
    });

    go_back
}
