use eframe::egui;
use sabu_core::config::Theme;

/// Applies a simple dark or light palette.
pub fn apply(context: &egui::Context, theme: Theme) {
    let mut visuals = match theme {
        Theme::Dark => egui::Visuals::dark(),
        Theme::Light => egui::Visuals::light(),
    };

    visuals.panel_fill = match theme {
        Theme::Dark => egui::Color32::from_rgb(24, 24, 27),
        Theme::Light => egui::Color32::from_rgb(246, 246, 248),
    };

    visuals.window_fill = visuals.panel_fill;

    context.set_visuals(visuals);
}
