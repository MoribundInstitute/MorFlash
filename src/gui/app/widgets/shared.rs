use eframe::egui::{self, Ui, Response, RichText, vec2, pos2, Rect, Mesh, Sense};
use crate::gui::app::theme::Theme;

/// Fancy gradient progress bar with rounded corners and a sleek knob.
pub fn progress_bar(ui: &mut Ui, progress: f32) {
    let progress = progress.clamp(0.0, 1.0);
    let height = 8.0;
    let corner_radius = 6.0;

    let (rect, _) = ui.allocate_exact_size(
        vec2(ui.available_width(), height),
        Sense::hover(),
    );

    let painter = ui.painter_at(rect.expand(4.0)); // Expand clip rect for outline + knob

    let track_rect = rect.expand(4.0);

    // Background track + outline
    painter.rect_filled(track_rect, corner_radius, Theme::PROGRESS_BG);
    painter.rect_stroke(
        track_rect,
        corner_radius,
        egui::Stroke::new(1.0, Theme::PROGRESS_OUTLINE),
    );

    // Gradient fill (now properly clipped with rounded corners)
    if progress > 0.0 {
        let fill_width = rect.width() * progress;
        let fill_rect = Rect::from_min_size(rect.min, vec2(fill_width, rect.height()));

        // Use painter with clip_rect to get perfect rounded corners on gradient
        let mut mesh = Mesh::default();
        let c1 = Theme::PROGRESS_FILL_START;
        let c2 = Theme::PROGRESS_FILL_END;

        let v0 = mesh.add_colored_vertex(pos2(fill_rect.left(),  fill_rect.top()),    c1);
        let v1 = mesh.add_colored_vertex(pos2(fill_rect.right(), fill_rect.top()),    c2);
        let v2 = mesh.add_colored_vertex(pos2(fill_rect.right(), fill_rect.bottom()), c2);
        let v3 = mesh.add_colored_vertex(pos2(fill_rect.left(),  fill_rect.bottom()), c1);

        mesh.add_triangle(v0, v1, v2);
        mesh.add_triangle(v0, v2, v3);

        painter.add(egui::Shape::mesh_at(fill_rect, mesh)); // mesh_at respects clip_rect
    }

    // Knob (only shown when progress > 0 and < 1 for a cleaner look)
    if (0.0..=1.0).contains(&progress) && progress > 0.001 && progress < 0.999 {
        let x = rect.left() + rect.width() * progress;
        let center = pos2(x, rect.center().y);
        painter.circle_filled(center, 5.0, Theme::PROGRESS_KNOB);
        // Optional subtle inner highlight
        painter.circle_filled(center, 2.0, egui::Color32::WHITE.linear_multiply(0.4));
    }
}

/// Primary button – big, bold, colored (used for main actions)
pub fn primary_button(ui: &mut Ui, text: impl Into<RichText>) -> Response {
    ui.add(
        egui::Button::new(text)
            .min_size(vec2(160.0, 44.0))
            .rounding(egui::Rounding::same(12.0))
            .fill(Theme::PRIMARY)                     // Now uses your theme!
            .stroke(egui::Stroke::none()),
    )
}

/// Ghost / subtle button – for secondary or navigation actions
pub fn ghost_button(ui: &mut Ui, text: impl Into<RichText>) -> Response {
    ui.add(
        egui::Button::new(text)
            .min_size(vec2(140.0, 36.0))
            .fill(egui::Color32::TRANSPARENT)
            .stroke(egui::Stroke::new(1.5, Theme::PRIMARY.gamma_multiply(0.7)))
            .rounding(egui::Rounding::same(10.0)),
    )
}

/// Consistent section header with proper spacing and style
pub fn section_header(ui: &mut Ui, text: &str) {
    ui.add_space(12.0);
    ui.colored_label(Theme::TEXT_HEADING, RichText::new(text).strong().size(20.0));
    ui.add_space(8.0);
    ui.separator();
    ui.add_space(8.0);
}

/// Optional framed panel – use when you want a clear content block
pub fn framed_panel(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    egui::Frame::default()
        .inner_margin(12.0)
        .outer_margin(8.0)
        .fill(Theme::PANEL_BG)
        .stroke(egui::Stroke::new(1.0, Theme::BORDER))
        .rounding(8.0)
        .show(ui, add_contents);
}

/// Shared "Back to deck list" button – centered, consistent across screens
pub fn back_to_deck_list_button(ui: &mut Ui) -> bool {
    ui.add_space(16.0);
    let response = ui.centered_and_justified(|ui| {
        ghost_button(ui, "Back to deck list").clicked()
    });
    ui.add_space(8.0);
    response.inner
}