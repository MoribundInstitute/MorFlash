// src/gui/theme.rs
use crate::gui::app::screens::options_screen::FontChoice;
use eframe::egui;

pub struct Theme;

impl Theme {
    // ========== COLORS ==========

    pub const OUTER_BG: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);
    pub const BG_APP: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);

    pub const CARD_BG: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);
    pub const CARD_TEXT: egui::Color32 = egui::Color32::from_rgb(236, 236, 235);

    pub const CARD_BAR_BG: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);
    pub const CARD_STROKE: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);

    pub const BUTTON_OUTLINE_NORMAL: egui::Color32 = egui::Color32::from_rgb(240, 240, 240);
    pub const BUTTON_OUTLINE_HOVER: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);
    pub const BUTTON_OUTLINE_ACTIVE: egui::Color32 = egui::Color32::from_rgb(210, 210, 210);

    pub const BUTTON_FILL: egui::Color32 = egui::Color32::TRANSPARENT;
    pub const BUTTON_TEXT: egui::Color32 = egui::Color32::from_rgb(236, 236, 235);
    pub const BUTTON_OUTLINE: egui::Color32 = egui::Color32::from_rgb(240, 240, 240);

    pub const CORRECT_OUTLINE: egui::Color32 = egui::Color32::from_rgb(80, 200, 120);
    pub const WRONG_OUTLINE: egui::Color32 = egui::Color32::from_rgb(220, 80, 80);

    // ========== LAYOUT ==========

    pub const CARD_ROUNDING: f32 = 24.0;
    pub const CARD_MARGIN: f32 = 32.0;
    pub const OUTER_TOP_PADDING_FRAC: f32 = 0.08;

    pub fn card_width(available_width: f32) -> f32 {
        available_width.clamp(480.0, 1200.0)
    }

    pub fn answer_button_size(card_width: f32) -> egui::Vec2 {
        let w = card_width * 0.45;
        let h = 56.0;
        egui::vec2(w, h)
    }

    pub fn card_frame() -> egui::Frame {
        egui::Frame {
            fill: Self::CARD_BG,
            stroke: egui::Stroke::new(2.0, Self::CARD_STROKE),
            rounding: egui::Rounding::same(Self::CARD_ROUNDING),
            ..Default::default()
        }
    }

    // ===============================================================
    //  APPLY THEME (COLORS + FONTS)
    // ===============================================================

    /// Apply visuals and fonts based on the current font choice.
    pub fn apply_to_ctx(
        ctx: &egui::Context,
        font_choice: FontChoice,
        custom_font_path: Option<&str>,
    ) {
        Self::apply_fonts(ctx, font_choice, custom_font_path);
        Self::apply_colors(ctx);
    }

    fn apply_fonts(ctx: &egui::Context, font_choice: FontChoice, custom_font_path: Option<&str>) {
        let mut fonts = egui::FontDefinitions::default();

        match font_choice {
            FontChoice::MorflashSerif => {
                // Cormorant Garamond (bundled)
                fonts.font_data.insert(
                    "morflash_serif".to_owned(),
                    egui::FontData::from_static(include_bytes!(
                        "../../assets/fonts/CormorantGaramond.ttf"
                    )),
                );
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "morflash_serif".to_owned());
                fonts
                    .families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, "morflash_serif".to_owned());
            }

            FontChoice::Pixel => {
                fonts.font_data.insert(
                    "public_pixel".to_owned(),
                    egui::FontData::from_static(include_bytes!(
                        "../../assets/fonts/PublicPixel.ttf"
                    )),
                );
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "public_pixel".to_owned());
                fonts
                    .families
                    .entry(egui::FontFamily::Monospace)
                    .or_default()
                    .insert(0, "public_pixel".to_owned());
            }

            FontChoice::System => {
                // Use egui's default font stack.
                ctx.set_fonts(fonts);
                return;
            }

            FontChoice::Custom => {
                let Some(path) = custom_font_path else {
                    // No path yet → fall back to system fonts.
                    ctx.set_fonts(fonts);
                    return;
                };

                match std::fs::read(path) {
                    Ok(bytes) => {
                        fonts
                            .font_data
                            .insert("custom_font".to_owned(), egui::FontData::from_owned(bytes));
                        fonts
                            .families
                            .entry(egui::FontFamily::Proportional)
                            .or_default()
                            .insert(0, "custom_font".to_owned());
                        fonts
                            .families
                            .entry(egui::FontFamily::Monospace)
                            .or_default()
                            .insert(0, "custom_font".to_owned());
                    }
                    Err(e) => {
                        eprintln!("Failed to load custom font {path}: {e}");
                        // Fall back to system fonts.
                        ctx.set_fonts(fonts);
                        return;
                    }
                }
            }
        }

        ctx.set_fonts(fonts);
    }

    /// Applies your colors, button styles, layout, etc.
    fn apply_colors(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(Self::CARD_TEXT);
        style.visuals.window_fill = Self::BG_APP;
        style.visuals.panel_fill = Self::BG_APP;

        let widgets = &mut style.visuals.widgets;
        let button_rounding = egui::Rounding::same(14.0);

        widgets.inactive.bg_fill = egui::Color32::TRANSPARENT;
        widgets.inactive.bg_stroke = egui::Stroke::new(2.0, Self::BUTTON_OUTLINE_NORMAL);
        widgets.inactive.rounding = button_rounding;

        widgets.hovered.bg_fill = egui::Color32::TRANSPARENT;
        widgets.hovered.bg_stroke = egui::Stroke::new(2.5, Self::BUTTON_OUTLINE_HOVER);
        widgets.hovered.rounding = button_rounding;

        widgets.active.bg_fill = egui::Color32::TRANSPARENT;
        widgets.active.bg_stroke = egui::Stroke::new(2.0, Self::BUTTON_OUTLINE_ACTIVE);
        widgets.active.rounding = button_rounding;

        widgets.noninteractive.bg_fill = Self::BG_APP;
        widgets.noninteractive.rounding = egui::Rounding::ZERO;
        widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, Self::CARD_TEXT);

        ctx.set_style(style);
    }
}
