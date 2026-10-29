//! Renders an analytics vote table.

use eframe::egui;

use crate::i18n::Translator;
use crate::ui::TableCache;

/// Renders one table of votes.
pub(crate) fn vote_table(
    ui: &mut egui::Ui,
    translator: &Translator,
    title_key: &str,
    table: &TableCache,
) {
    ui.heading(translator.t(title_key));
    if table.rows.is_empty() {
        ui.label(translator.t("tables.no_data"));
        return;
    }

    egui::Grid::new(title_key)
        .num_columns(2)
        .striped(true)
        .show(ui, |ui| {
            for group in &table.rows {
                ui.label(&group.name);
                ui.label(translator.format_number(group.votes));
                ui.end_row();
            }
        });

    if table.has_more {
        ui.small(translator.t("tables.limit"));
    }
}
