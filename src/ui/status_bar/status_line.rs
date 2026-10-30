use eframe::egui;

use crate::i18n::translator::Translator;
use crate::ui::status_message::StatusMessage;

/// Renders the current status message.
pub struct StatusLine;

impl StatusLine {
    /// Shows the status message, with a spinner while a command runs.
    pub fn show(ui: &mut egui::Ui, translator: &Translator, status: &StatusMessage) {
        ui.horizontal(|ui| {
            if status.is_busy() {
                ui.spinner();
            }

            ui.label(status.text(translator));
        });
    }
}
