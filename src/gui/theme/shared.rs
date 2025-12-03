// src/gui/theme.rs
use crate::gui::app::screens::options_screen::FontChoice;
use eframe::egui;

pub struct Theme;

impl Theme {
    // ===============================================================
    //  COLORS
    // ===============================================================

    // Very dark blue-black overall app background
    pub const OUTER_BG: egui::Color32 = egui::Color32::from_rgb(3, 6, 18);
    pub const BG_APP: egui::Color32 = egui::Color32::from_rgb(3, 6, 18);

    // Base card surface (for generic panels)
    pub const CARD_BG: egui::Color32 = egui::Color32::from_rgb(9, 16, 34);
    pub const CARD_TEXT: egui::Color32 = egui::Color32::from_rgb(236, 236, 235);

    // Neon accents
    pub const NEON_CYAN: egui::Color32 = egui::Color32::from_rgb(120, 210, 255);
    pub const NEON_PURPLE: egui::Color32 = egui::Color32::from_rgb(178, 102, 255);

    // Generic card / panel stroke (fallback / legacy)
    pub const CARD_STROKE: egui::Color32 = egui::Color32::from_rgb(70, 90, 130);

    // Legacy / compatibility: old progress bar background
    // Kept so older code using CARD_BAR_BG still compiles.
    pub const CARD_BAR_BG: egui::Color32 = egui::Color32::from_rgb(40, 40, 40);

    // -------- Progress bar colors ---------------------------------

    // Background & outline around the bar
    pub const PROGRESS_BG: egui::Color32 = egui::Color32::from_rgb(12, 20, 42);
    pub const PROGRESS_OUTLINE: egui::Color32 = egui::Color32::from_rgb(120, 170, 230);

    // Fill gradient (left → right) — neon cyan → neon purple
    pub const PROGRESS_FILL_START: egui::Color32 = Self::NEON_CYAN;
    pub const PROGRESS_FILL_END: egui::Color32 = Self::NEON_PURPLE;

    // Knob / highlight on the current progress position
    pub const PROGRESS_KNOB: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);

    // -------- Buttons & feedback ----------------------------------

    // Button outlines / text
    pub const BUTTON_OUTLINE_NORMAL: egui::Color32 = egui::Color32::from_rgb(140, 200, 255);
    pub const BUTTON_OUTLINE_HOVER: egui::Color32 = egui::Color32::from_rgb(180, 230, 255);
    pub const BUTTON_OUTLINE_ACTIVE: egui::Color32 = egui::Color32::from_rgb(100, 170, 230);

    // Generic outline color if a single one is needed elsewhere
    pub const BUTTON_OUTLINE: egui::Color32 = Self::BUTTON_OUTLINE_NORMAL;

    pub const BUTTON_FILL: egui::Color32 = egui::Color32::from_rgb(10, 18, 38);
    pub const BUTTON_FILL_HOVER: egui::Color32 = egui::Color32::from_rgb(16, 26, 54);
    pub const BUTTON_FILL_ACTIVE: egui::Color32 = egui::Color32::from_rgb(8, 16, 34);

    pub const BUTTON_TEXT: egui::Color32 = egui::Color32::from_rgb(236, 236, 235);

    pub const CORRECT_OUTLINE: egui::Color32 = egui::Color32::from_rgb(80, 200, 140);
    pub const WRONG_OUTLINE: egui::Color32 = egui::Color32::from_rgb(230, 90, 120);

    // ===============================================================
    //  LAYOUT CONSTANTS
    // ===============================================================

    pub const CARD_ROUNDING: f32 = 24.0;
    pub const CARD_MARGIN: f32 = 32.0;
    pub const OUTER_TOP_PADDING_FRAC: f32 = 0.08;

    pub const BUTTON_ROUNDING: f32 = 14.0;

    pub fn card_width(available_width: f32) -> f32 {
        available_width.clamp(480.0, 1200.0)
    }

    pub fn answer_button_size(card_width: f32) -> egui::Vec2 {
        let w = card_width * 0.45;
        let h = 56.0;
        egui::vec2(w, h)
    }

    // Glassy background color for the main flashcard.
    // (Functions can call from_rgba_unmultiplied; consts cannot.)
    pub fn card_bg_glass() -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(12, 22, 52, 220)
    }

    // Neon glow color used for the card shadow.
    pub fn card_neon_glow() -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(40, 120, 255, 150)
    }

    /// Main study “flashcard” frame: glassy surface with neon border + glow.
    pub fn card_frame() -> egui::Frame {
        // Soft neon glow below card
        let shadow = egui::Shadow {
            offset: egui::vec2(0.0, 18.0),
            blur: 46.0,
            spread: 0.0,
            color: Self::card_neon_glow(),
        };

        egui::Frame {
            // Slightly transparent fill so the dark background bleeds through.
            fill: Self::card_bg_glass(),
            // Thin neon stroke around the card
            stroke: egui::Stroke::new(1.6, Self::NEON_CYAN),
            rounding: egui::Rounding::same(Self::CARD_ROUNDING),
            shadow,
            inner_margin: egui::Margin::symmetric(Self::CARD_MARGIN, Self::CARD_MARGIN - 4.0),
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
                        "../../../assets/fonts/CormorantGaramond.ttf"
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
                        "../../../assets/fonts/PublicPixel.ttf"
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

        // Global backgrounds
        style.visuals.window_fill = Self::BG_APP;
        style.visuals.panel_fill = Self::BG_APP;

        // Hyperlinks / selections pick up the neon accent
        style.visuals.hyperlink_color = Self::NEON_CYAN;
        style.visuals.selection.bg_fill = egui::Color32::from_rgba_unmultiplied(60, 120, 220, 160);
        style.visuals.selection.stroke = egui::Stroke::new(1.0, Self::CARD_TEXT);

        // Remove egui's default window outline / shadow so only our
        // inner flashcard frame is visible.
        style.visuals.window_rounding = egui::Rounding::same(Self::CARD_ROUNDING);
        style.visuals.window_shadow = egui::Shadow::NONE;
        style.visuals.window_stroke = egui::Stroke::NONE;

        // Slight “inner glow” along card edges by tweaking extreme light/dark
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(4, 10, 26);
        style.visuals.code_bg_color = egui::Color32::from_rgb(10, 18, 40);

        let widgets = &mut style.visuals.widgets;

        // Inactive
        widgets.inactive.rounding = egui::Rounding::same(Self::BUTTON_ROUNDING);
        widgets.inactive.bg_fill = Self::BUTTON_FILL;
        widgets.inactive.bg_stroke = egui::Stroke::new(1.5, Self::BUTTON_OUTLINE_NORMAL);
        widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::BUTTON_TEXT);

        // Hovered
        widgets.hovered.rounding = egui::Rounding::same(Self::BUTTON_ROUNDING);
        widgets.hovered.bg_fill = Self::BUTTON_FILL_HOVER;
        widgets.hovered.bg_stroke = egui::Stroke::new(2.0, Self::BUTTON_OUTLINE_HOVER);
        widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::BUTTON_TEXT);

        // Active / pressed
        widgets.active.rounding = egui::Rounding::same(Self::BUTTON_ROUNDING);
        widgets.active.bg_fill = Self::BUTTON_FILL_ACTIVE;
        widgets.active.bg_stroke = egui::Stroke::new(2.0, Self::BUTTON_OUTLINE_ACTIVE);
        widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::BUTTON_TEXT);

        // Non-interactive labels / text
        widgets.noninteractive.bg_fill = Self::BG_APP;
        widgets.noninteractive.rounding = egui::Rounding::ZERO;
        widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, Self::CARD_TEXT);

        ctx.set_style(style);
    }
}
