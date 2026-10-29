//! Renders the summary and analytics content.

use eframe::egui;

use crate::i18n::Translator;
use crate::ui::{AnalyticsCache, summary_grid, vote_table};

/// Renders the central content area.
pub(crate) fn content_panel(
    ui: &mut egui::Ui,
    translator: &Translator,
    cache: Option<&AnalyticsCache>,
) {
    let Some(cache) = cache else {
        ui.centered_and_justified(|ui| {
            ui.vertical_centered(|ui| {
                ui.label(translator.t("app.empty"));
                ui.small(translator.t("app.drop.hint"));
            });
        });
        return;
    };

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.heading(translator.t("summary.title"));
        summary_grid(ui, translator, cache.summary);

        ui.separator();

        ui.columns(2, |columns| {
            vote_table(&mut columns[0], translator, "tables.cargo", &cache.by_cargo);
            vote_table(
                &mut columns[1],
                translator,
                "tables.votavel",
                &cache.by_votavel,
            );
        });

        ui.separator();
        vote_table(ui, translator, "tables.party", &cache.by_partido);
    });
}
