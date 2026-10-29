//! Renders the top panel.

use eframe::egui;
use sabu_core::config::Config;

use crate::i18n::Translator;
use crate::ui::{MenuAction, file_bar, menu_bar};

/// Actions returned by the top panel.
pub(crate) struct TopBarAction {
    /// True when the open button was clicked.
    pub(crate) open: bool,
    /// Activated menu action.
    pub(crate) menu: MenuAction,
}

/// Renders the top panel.
pub(crate) fn top_bar(
    ui: &mut egui::Ui,
    translator: &Translator,
    config: Config,
    path_input: &mut String,
) -> TopBarAction {
    let mut action = TopBarAction {
        open: false,
        menu: MenuAction::None,
    };

    egui::Panel::top("sabu_top").show(ui, |ui| {
        action.menu = menu_bar(ui, translator, config);
        ui.separator();
        action.open = file_bar(ui, translator, path_input);
    });

    action
}
