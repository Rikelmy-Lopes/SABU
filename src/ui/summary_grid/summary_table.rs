//! Renders the dataset summary.

use eframe::egui;
use sabu_core::dataset::DatasetSummary;

use crate::i18n::Translator;

/// Renders the summary grid.
pub(crate) fn summary_grid(ui: &mut egui::Ui, translator: &Translator, summary: DatasetSummary) {
    egui::Grid::new("summary_grid")
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            summary_row(
                ui,
                translator,
                "summary.records",
                summary.total_records as u64,
            );
            summary_row(ui, translator, "summary.votes", summary.total_votes);
            summary_row(
                ui,
                translator,
                "summary.municipalities",
                summary.municipalities as u64,
            );
            summary_row(ui, translator, "summary.cargos", summary.cargos as u64);
            summary_row(ui, translator, "summary.votaveis", summary.votaveis as u64);
        });
}

fn summary_row(ui: &mut egui::Ui, translator: &Translator, label_key: &str, value: u64) {
    ui.label(translator.t(label_key));
    ui.label(translator.format_number(value));
    ui.end_row();
}
