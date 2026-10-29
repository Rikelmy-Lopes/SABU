//! Renders the CSV path input.

use eframe::egui;

use crate::i18n::Translator;

/// Renders the path input and returns true when the open button was clicked.
pub(crate) fn file_bar(
    ui: &mut egui::Ui,
    translator: &Translator,
    path_input: &mut String,
) -> bool {
    let mut open = false;

    ui.horizontal(|ui| {
        ui.label(translator.t("app.path.label"));
        let width = (ui.available_width() - 100.0).max(120.0);
        ui.add_sized(
            [width, 22.0],
            egui::TextEdit::singleline(path_input).hint_text(translator.t("app.path.hint")),
        );
        if ui.button(translator.t("app.path.open")).clicked() {
            open = true;
        }
    });

    open
}
