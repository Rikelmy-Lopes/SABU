//! Renders the status bar.

use eframe::egui;

use crate::i18n::Translator;
use crate::ui::StatusMessage;

/// Renders the status bar.
pub(crate) fn status_bar(
    ui: &mut egui::Ui,
    translator: &Translator,
    pending_commands: usize,
    status: Option<StatusMessage>,
) {
    egui::Panel::bottom("sabu_status").show(ui, |ui| {
        ui.horizontal(|ui| {
            if pending_commands > 0 {
                ui.spinner();
                ui.label(translator.t("app.loading"));
            } else if let Some(status) = status {
                let text = format!(
                    "{}: {}",
                    translator.t(status.prefix_key),
                    translator.t(status.message_key)
                );
                if status.is_error {
                    ui.colored_label(ui.visuals().error_fg_color, text);
                } else {
                    ui.label(text);
                }
            } else {
                ui.label(translator.t("status.ready"));
            }
        });
    });
}
