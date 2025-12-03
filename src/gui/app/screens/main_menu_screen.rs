use eframe::egui;
use egui::{Color32, FontId, RichText, TextureHandle};

use crate::gui::app::screens::options_screen::MainMenuOptions;
use crate::gui::theme::MenuTheme;

#[derive(Debug)]
pub enum MainMenuAction {
    None,
    ChooseDeck,
    OpenDeckBuilder,
    OpenOptions,
}

pub fn draw_main_menu(
    ui: &mut egui::Ui,
    focus_index: usize,
    mor_button_tex: Option<&TextureHandle>,
    critter_tex: Option<&TextureHandle>,
    main_menu_opts: &MainMenuOptions,
) -> MainMenuAction {
    MenuTheme::apply_to_ctx(ui.ctx());

    // Keeps the parameter “used” until you actually read fields from it.
    let _opts: &MainMenuOptions = main_menu_opts;

    let mut action = MainMenuAction::None;
    let mut critter_target: Option<egui::Rect> = None;

    // Fixed focus indices
    let choose_deck_index = 0;
    let deck_builder_index = 1;
    let options_index = 2;

    ui.vertical_centered(|ui| {
        ui.add_space(32.0);

        ui.label(
            RichText::new("MorFlash")
                .font(FontId::proportional(48.0))
                .strong(),
        );
        ui.label(RichText::new("Main Menu").font(FontId::proportional(26.0)));

        ui.add_space(32.0);

        // --- Choose Deck button ---
        let (choose_response, choose_rect) =
            draw_menu_button(ui, "Choose a deck...", mor_button_tex, 260.0);

        let choose_active =
            choose_response.hovered() || focus_index == choose_deck_index;

        if choose_active {
            critter_target = Some(choose_rect);
        }

        if choose_response.clicked() {
            action = MainMenuAction::ChooseDeck;
        }

        ui.add_space(18.0);

        // --- Deck Builder button ---
        let (builder_response, builder_rect) =
            draw_menu_button(ui, "🧱 Deck Builder", mor_button_tex, 260.0);

        let builder_active =
            builder_response.hovered() || focus_index == deck_builder_index;

        if builder_active {
            critter_target = Some(builder_rect);
        }

        if builder_response.clicked() {
            action = MainMenuAction::OpenDeckBuilder;
        }

        ui.add_space(18.0);

        // --- Options button ---
        let (options_response, options_rect) =
            draw_menu_button(ui, "⚙  Options", mor_button_tex, 260.0);

        let options_active =
            options_response.hovered() || focus_index == options_index;

        if options_active {
            critter_target = Some(options_rect);
        }

        if options_response.clicked() {
            action = MainMenuAction::OpenOptions;
        }

        ui.add_space(24.0);

        ui.label(
            RichText::new("Tip: you can adjust sound and other settings in Options.")
                .font(FontId::proportional(16.0)),
        );
    });

    // Draw critter sprite next to the active button (on top of everything)
    if let (Some(tex), Some(target)) = (critter_tex, critter_target) {
        let size = egui::vec2(48.0, 48.0);
        let pos = egui::pos2(
            target.left() - size.x - 12.0,
            target.center().y - size.y / 2.0,
        );
        let rect = egui::Rect::from_min_size(pos, size);

        ui.painter().image(
            tex.id(),
            rect,
            egui::Rect::from_min_max(
                egui::pos2(0.0, 0.0),
                egui::pos2(1.0, 1.0),
            ),
            Color32::WHITE,
        );
    }

    action
}

// returns (Response, button_rect)
fn draw_menu_button(
    ui: &mut egui::Ui,
    label: &str,
    mor_button_tex: Option<&TextureHandle>,
    min_width: f32,
) -> (egui::Response, egui::Rect) {
    let font_id = FontId::proportional(22.0);

    if mor_button_tex.is_none() {
        let text = RichText::new(label.to_string()).font(font_id);
        let button = egui::Button::new(text).min_size(egui::vec2(min_width, 44.0));
        let response = ui.add(button);
        let rect = response.rect;
        return (response, rect);
    }

    let tex = mor_button_tex.unwrap();

    let galley = ui.fonts(|f| f.layout_no_wrap(label.to_string(), font_id.clone(), Color32::WHITE));
    let sz = galley.size();

    let padding = egui::vec2(24.0, 8.0);
    let desired = egui::vec2(
        sz.x.max(min_width) + 2.0 * padding.x,
        sz.y + 2.0 * padding.y,
    );

    let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
    let painter = ui.painter();

    painter.image(
        tex.id(),
        rect,
        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
        Color32::WHITE,
    );

    let text_pos = egui::pos2(rect.center().x - sz.x / 2.0, rect.center().y - sz.y / 2.0);

    painter.galley(text_pos, galley, Color32::WHITE);

    (response, rect)
}
