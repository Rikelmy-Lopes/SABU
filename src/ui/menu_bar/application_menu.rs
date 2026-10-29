//! Renders the application menus.

use eframe::egui;
use sabu_core::config::{Config, Language, Theme};

use crate::i18n::Translator;

/// Action chosen in the menu bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum MenuAction {
    /// No menu item was activated.
    #[default]
    None,
    /// Clear the loaded dataset.
    ClearData,
    /// Close the window.
    Exit,
    /// Undo the last command.
    Undo,
    /// Redo the last undone command.
    Redo,
    /// Switch the interface language.
    SetLanguage(Language),
    /// Switch the visual theme.
    SetTheme(Theme),
}

/// Renders the menu bar and returns the activated action.
pub(crate) fn menu_bar(ui: &mut egui::Ui, translator: &Translator, config: Config) -> MenuAction {
    let mut action = MenuAction::None;

    egui::MenuBar::new().ui(ui, |ui| {
        ui.menu_button(translator.t("menu.file"), |ui| {
            if ui.button(translator.t("menu.file.clear")).clicked() {
                action = MenuAction::ClearData;
            }
            if ui.button(translator.t("menu.file.exit")).clicked() {
                action = MenuAction::Exit;
            }
        });

        ui.menu_button(translator.t("menu.edit"), |ui| {
            if ui.button(translator.t("menu.edit.undo")).clicked() {
                action = MenuAction::Undo;
            }
            if ui.button(translator.t("menu.edit.redo")).clicked() {
                action = MenuAction::Redo;
            }
        });

        ui.menu_button(translator.t("menu.language"), |ui| {
            for language in translator.available_languages() {
                if ui
                    .selectable_label(
                        translator.active_language() == language,
                        translator.t(language.name_key()),
                    )
                    .clicked()
                {
                    action = MenuAction::SetLanguage(language);
                }
            }
        });

        ui.menu_button(translator.t("menu.theme"), |ui| {
            if ui
                .selectable_label(config.theme == Theme::Dark, translator.t("theme.dark"))
                .clicked()
            {
                action = MenuAction::SetTheme(Theme::Dark);
            }
            if ui
                .selectable_label(config.theme == Theme::Light, translator.t("theme.light"))
                .clicked()
            {
                action = MenuAction::SetTheme(Theme::Light);
            }
        });
    });

    action
}
