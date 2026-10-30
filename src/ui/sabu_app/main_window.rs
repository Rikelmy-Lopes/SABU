use std::sync::Arc;
use std::time::Duration;

use eframe::egui;
use sabu_core::config::Theme;
use sabu_core::state::app_state::AppState;

use crate::commands::clear_dataset::clear_command::CLEAR_DATASET;
use crate::commands::command::Command;
use crate::commands::dispatch::{CommandDispatcher, CommandEvent};
use crate::commands::load_dataset::load_command::LOAD_DATASET;
use crate::commands::{
    ClearDatasetCommand, LoadDatasetCommand, SetLanguageCommand, SetThemeCommand,
};
use crate::i18n::translator::Translator;
use crate::ui::content_panel::ContentArea;
use crate::ui::file_bar::FilePathBar;
use crate::ui::menu_bar::{ApplicationMenu, MenuAction};
use crate::ui::status_bar::StatusLine;
use crate::ui::status_message::StatusMessage;
use crate::ui::theme::apply as apply_theme;

/// Root `eframe` application.
pub struct MainWindow {
    dispatcher: CommandDispatcher,
    translator: Translator,
    snapshot: Arc<AppState>,
    status: StatusMessage,
    applied_theme: Option<Theme>,
    show_about: bool,
}

impl MainWindow {
    /// Creates the window and applies the persisted theme.
    pub fn new(
        creation_context: &eframe::CreationContext<'_>,
        dispatcher: CommandDispatcher,
        translator: Translator,
        snapshot: Arc<AppState>,
    ) -> Self {
        let theme = snapshot.config().theme;
        apply_theme(&creation_context.egui_ctx, theme);

        Self {
            dispatcher,
            translator,
            snapshot,
            status: StatusMessage::Ready,
            applied_theme: Some(theme),
            show_about: false,
        }
    }

    fn drain_events(&mut self) {
        while let Some(event) = self.dispatcher.try_recv() {
            match event {
                CommandEvent::Started => {
                    self.status = StatusMessage::Busy;
                }
                CommandEvent::Succeeded { id, state } => {
                    self.snapshot = state;
                    self.status = if id == LOAD_DATASET {
                        let records = self
                            .snapshot
                            .dataset()
                            .map_or(0, |dataset| dataset.summary().record_count);

                        StatusMessage::Loaded { records }
                    } else if id == CLEAR_DATASET {
                        StatusMessage::Cleared
                    } else {
                        StatusMessage::Ready
                    };
                }
                CommandEvent::Failed { id, message, state } => {
                    log::debug!("command failed: {id}");
                    self.snapshot = state;
                    self.status = StatusMessage::Failed(message);
                }
            }
        }
    }

    fn sync_language(&mut self) {
        let language = self.snapshot.config().language;

        if self.translator.language() == language {
            return;
        }

        match Translator::new(language) {
            Ok(translator) => self.translator = translator,
            Err(error) => self.status = StatusMessage::Failed(error.to_string()),
        }
    }

    fn dispatch(&mut self, command: Box<dyn Command>) {
        match self.dispatcher.dispatch(command) {
            Ok(()) => self.status = StatusMessage::Busy,
            Err(error) => self.status = StatusMessage::Failed(error.to_string()),
        }
    }

    fn handle_action(&mut self, action: MenuAction, context: &egui::Context) {
        match action {
            MenuAction::OpenFile => self.open_file_dialog(),
            MenuAction::ClearDataset => {
                self.dispatch(Box::new(ClearDatasetCommand::new()));
            }
            MenuAction::Exit => {
                context.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            MenuAction::SetTheme(theme) => {
                self.dispatch(Box::new(SetThemeCommand::new(theme)));
            }
            MenuAction::SetLanguage(language) => {
                self.dispatch(Box::new(SetLanguageCommand::new(language)));
            }
            MenuAction::ShowAbout => {
                self.show_about = true;
            }
        }
    }

    fn open_file_dialog(&mut self) {
        let filter = self.translator.text("dialog.csv_filter");

        if let Some(path) = rfd::FileDialog::new()
            .add_filter(&filter, &["csv"])
            .pick_file()
        {
            self.dispatch(Box::new(LoadDatasetCommand::new(path)));
        }
    }

    fn show_about_window(&mut self, context: &egui::Context) {
        if !self.show_about {
            return;
        }

        let mut open = self.show_about;

        egui::Window::new(self.translator.text("about.title"))
            .collapsible(false)
            .resizable(false)
            .open(&mut open)
            .show(context, |ui| {
                ui.label(self.translator.text("about.description"));
                ui.add_space(8.0);

                let version = self
                    .translator
                    .text_with("about.version", &[("version", env!("CARGO_PKG_VERSION"))]);

                ui.label(version);
            });

        self.show_about = open;
    }
}

impl eframe::App for MainWindow {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.drain_events();
        self.sync_language();

        let theme = self.snapshot.config().theme;

        if self.applied_theme != Some(theme) {
            apply_theme(ui.ctx(), theme);
            self.applied_theme = Some(theme);
        }

        if self.dispatcher.pending() > 0 || self.status.is_busy() {
            ui.ctx().request_repaint_after(Duration::from_millis(80));
        }

        let mut open_requested = false;

        egui::Panel::top("menu_bar").show(ui, |ui| {
            let action = ApplicationMenu::show(
                ui,
                &self.translator,
                self.snapshot.config(),
                self.snapshot.has_dataset(),
            );

            if let Some(action) = action {
                self.handle_action(action, ui.ctx());
            }
        });

        egui::Panel::top("file_bar").show(ui, |ui| {
            open_requested |= FilePathBar::show(ui, &self.translator, &self.snapshot);
        });

        egui::Panel::bottom("status_bar").show(ui, |ui| {
            StatusLine::show(ui, &self.translator, &self.status);
        });

        egui::CentralPanel::default_margins().show(ui, |ui| {
            ContentArea::show(ui, &self.translator, &self.snapshot, &mut open_requested);
        });

        if open_requested {
            self.open_file_dialog();
        }

        self.show_about_window(ui.ctx());
    }
}
