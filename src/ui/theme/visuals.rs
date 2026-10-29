//! Applies the selected visual theme.

use eframe::egui;
use sabu_core::config::Theme;

/// Applies the selected theme to the whole context.
pub(crate) fn apply_theme(context: &egui::Context, theme: Theme) {
    let mut visuals = match theme {
        Theme::Dark => egui::Visuals::dark(),
        Theme::Light => egui::Visuals::light(),
    };
    visuals.panel_fill = match theme {
        Theme::Dark => egui::Color32::from_gray(26),
        Theme::Light => egui::Color32::from_gray(246),
    };
    context.set_visuals(visuals);
}
