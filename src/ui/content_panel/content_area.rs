use eframe::egui;
use sabu_core::analytics::vote_group::VoteGroup;
use sabu_core::dataset::ballot_dataset::BallotDataset;
use sabu_core::state::app_state::AppState;

use crate::i18n::number_formatter::LocaleNumber;
use crate::i18n::translator::Translator;

/// Summary and vote tables, or the empty state.
pub struct ContentArea;

impl ContentArea {
    /// Draws the central area.
    pub fn show(
        ui: &mut egui::Ui,
        translator: &Translator,
        state: &AppState,
        open_requested: &mut bool,
    ) {
        match state.dataset() {
            Some(dataset) => Self::show_dataset(ui, translator, dataset),
            None => Self::show_empty(ui, translator, open_requested),
        }
    }

    fn show_empty(ui: &mut egui::Ui, translator: &Translator, open_requested: &mut bool) {
        ui.vertical_centered(|ui| {
            ui.add_space(48.0);
            ui.label(translator.text("empty.no_dataset"));

            if ui.button(translator.text("action.open_file")).clicked() {
                *open_requested = true;
            }
        });
    }

    fn show_dataset(ui: &mut egui::Ui, translator: &Translator, dataset: &BallotDataset) {
        ui.heading(translator.text("summary.title"));
        summary_grid(ui, translator, dataset);

        ui.add_space(12.0);
        ui.separator();
        ui.heading(translator.text("votes.title"));

        vote_section(
            ui,
            translator,
            "votes.by_office",
            &dataset.analytics().by_office,
            true,
        );
        vote_section(
            ui,
            translator,
            "votes.by_party",
            &dataset.analytics().by_party,
            false,
        );
        vote_section(
            ui,
            translator,
            "votes.by_votable",
            &dataset.analytics().by_votable,
            false,
        );
    }
}

fn summary_grid(ui: &mut egui::Ui, translator: &Translator, dataset: &BallotDataset) {
    let language = translator.language();
    let summary = dataset.summary();

    egui::Grid::new("summary_grid")
        .num_columns(2)
        .spacing([24.0, 6.0])
        .striped(true)
        .show(ui, |ui| {
            summary_row(
                ui,
                translator.text("summary.records"),
                LocaleNumber::format_integer(language, summary.record_count as i64),
            );
            summary_row(
                ui,
                translator.text("summary.total_votes"),
                LocaleNumber::format_integer(language, summary.total_votes as i64),
            );
            summary_row(
                ui,
                translator.text("summary.ballot_boxes"),
                LocaleNumber::format_integer(language, summary.ballot_box_count as i64),
            );
            summary_row(
                ui,
                translator.text("summary.municipalities"),
                LocaleNumber::format_integer(language, summary.municipality_count as i64),
            );
            summary_row(
                ui,
                translator.text("summary.file"),
                dataset.path().display().to_string(),
            );
        });
}

fn summary_row(ui: &mut egui::Ui, label: String, value: String) {
    ui.label(label);
    ui.label(value);
    ui.end_row();
}

fn vote_section(
    ui: &mut egui::Ui,
    translator: &Translator,
    title_key: &'static str,
    groups: &[VoteGroup],
    default_open: bool,
) {
    egui::CollapsingHeader::new(translator.text(title_key))
        .default_open(default_open)
        .show(ui, |ui| {
            let language = translator.language();
            let row_height =
                ui.text_style_height(&egui::TextStyle::Body) + ui.spacing().item_spacing.y;

            egui::ScrollArea::vertical()
                .id_salt(title_key)
                .max_height(200.0)
                .auto_shrink([false, true])
                .show_rows(ui, row_height, groups.len(), |ui, range| {
                    for group in &groups[range] {
                        ui.horizontal(|ui| {
                            let votes_width = 100.0;
                            let key_width = (ui.available_width() - votes_width).max(0.0);

                            ui.add_sized(
                                [key_width, row_height],
                                egui::Label::new(group.key.as_str()).truncate(),
                            );

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(LocaleNumber::format_integer(
                                        language,
                                        group.votes as i64,
                                    ));
                                },
                            );
                        });
                    }
                });
        });
}
