use eframe::egui;
use sabu_core::state::app_state::AppState;

use crate::i18n::translator::Translator;

/// Toolbar that shows the open button and the loaded file path.
pub struct FilePathBar;

impl FilePathBar {
    /// Draws the toolbar and reports whether the open button was clicked.
    pub fn show(ui: &mut egui::Ui, translator: &Translator, state: &AppState) -> bool {
        let mut open_requested = false;

        ui.horizontal(|ui| {
            if ui.button(translator.text("action.open_file")).clicked() {
                open_requested = true;
            }

            if let Some(dataset) = state.dataset() {
                ui.separator();
                ui.label(translator.text("summary.file"));
                ui.label(dataset.path().display().to_string());
            }
        });

        open_requested
    }
}
