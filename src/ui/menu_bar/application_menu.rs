use eframe::egui;
use sabu_core::config::{Language, Theme, UserConfig};

use crate::i18n::translator::Translator;

/// Action requested by the menu.
pub enum MenuAction {
    /// Ask the user for a CSV file.
    OpenFile,
    /// Unload the active dataset.
    ClearDataset,
    /// Close the application.
    Exit,
    /// Change the theme.
    SetTheme(Theme),
    /// Change the language.
    SetLanguage(Language),
    /// Open the about window.
    ShowAbout,
}

/// Main menu bar of the application.
pub struct ApplicationMenu;

impl ApplicationMenu {
    /// Draws the menu bar and returns the requested action, if any.
    pub fn show(
        ui: &mut egui::Ui,
        translator: &Translator,
        config: &UserConfig,
        has_dataset: bool,
    ) -> Option<MenuAction> {
        let mut action = None;

        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button(translator.text("menu.file"), |ui| {
                if ui.button(translator.text("menu.file.open")).clicked() {
                    action = Some(MenuAction::OpenFile);
                    ui.close();
                }

                if ui
                    .add_enabled(
                        has_dataset,
                        egui::Button::new(translator.text("menu.file.clear")),
                    )
                    .clicked()
                {
                    action = Some(MenuAction::ClearDataset);
                    ui.close();
                }

                ui.separator();

                if ui.button(translator.text("menu.file.exit")).clicked() {
                    action = Some(MenuAction::Exit);
                    ui.close();
                }
            });

            ui.menu_button(translator.text("menu.settings"), |ui| {
                ui.menu_button(translator.text("menu.settings.theme"), |ui| {
                    if ui
                        .selectable_label(
                            config.theme == Theme::Dark,
                            translator.text("menu.settings.theme.dark"),
                        )
                        .clicked()
                    {
                        action = Some(MenuAction::SetTheme(Theme::Dark));
                        ui.close();
                    }

                    if ui
                        .selectable_label(
                            config.theme == Theme::Light,
                            translator.text("menu.settings.theme.light"),
                        )
                        .clicked()
                    {
                        action = Some(MenuAction::SetTheme(Theme::Light));
                        ui.close();
                    }
                });

                ui.menu_button(translator.text("menu.settings.language"), |ui| {
                    for language in Language::ALL {
                        let key = format!("language.{}", language.code());
                        let label = translator.text(&key);

                        if ui
                            .selectable_label(config.language == language, label)
                            .clicked()
                        {
                            action = Some(MenuAction::SetLanguage(language));
                            ui.close();
                        }
                    }
                });
            });

            ui.menu_button(translator.text("menu.help"), |ui| {
                if ui.button(translator.text("menu.help.about")).clicked() {
                    action = Some(MenuAction::ShowAbout);
                    ui.close();
                }
            });
        });

        action
    }
}
