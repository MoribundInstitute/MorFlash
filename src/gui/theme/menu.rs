use eframe::egui;

pub struct MenuTheme;

impl MenuTheme {
    // Colors for the main menu look
    pub const OUTER_BG: egui::Color32 = egui::Color32::from_rgb(6, 6, 8);
    pub const STRIP_BG: egui::Color32 = egui::Color32::from_rgb(16, 16, 28);
    pub const PANEL_BG: egui::Color32 = egui::Color32::from_rgb(10, 10, 20);

    pub const TITLE_TEXT: egui::Color32 = egui::Color32::from_rgb(234, 234, 244);
    pub const NORMAL_TEXT: egui::Color32 = egui::Color32::from_rgb(210, 210, 220);

    pub const BUTTON_FILL: egui::Color32 = egui::Color32::from_rgb(14, 24, 54);
    pub const BUTTON_OUTLINE: egui::Color32 = egui::Color32::from_rgb(90, 205, 255);
    pub const BUTTON_HOVER_FILL: egui::Color32 = egui::Color32::from_rgb(18, 32, 70);
    pub const BUTTON_HOVER_OUTLINE: egui::Color32 = egui::Color32::from_rgb(120, 230, 255);

    pub const BUTTON_ROUNDING: f32 = 18.0;

    /// Apply menu-specific visuals to the whole egui context.
    pub fn apply_to_ctx(ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();

        style.visuals.window_fill = Self::PANEL_BG;
        style.visuals.panel_fill = Self::PANEL_BG;
        style.visuals.override_text_color = Some(Self::NORMAL_TEXT);

        let widgets = &mut style.visuals.widgets;
        let r = egui::Rounding::same(Self::BUTTON_ROUNDING);

        widgets.inactive.bg_fill = Self::BUTTON_FILL;
        widgets.inactive.bg_stroke = egui::Stroke::new(2.0, Self::BUTTON_OUTLINE);
        widgets.inactive.rounding = r;

        widgets.hovered.bg_fill = Self::BUTTON_HOVER_FILL;
        widgets.hovered.bg_stroke = egui::Stroke::new(2.5, Self::BUTTON_HOVER_OUTLINE);
        widgets.hovered.rounding = r;

        widgets.active.bg_fill = Self::BUTTON_HOVER_FILL;
        widgets.active.bg_stroke = egui::Stroke::new(2.0, Self::BUTTON_HOVER_OUTLINE);
        widgets.active.rounding = r;

        ctx.set_style(style);
    }

    /// Helper matching the old call-site signature.
    pub fn with_menu_style(ui: &mut egui::Ui, add_contents: impl FnOnce(&mut egui::Ui)) {
        let ctx = ui.ctx().clone();
        Self::apply_to_ctx(&ctx);
        add_contents(ui);
    }
}
