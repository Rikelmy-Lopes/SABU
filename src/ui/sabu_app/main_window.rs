//! Main eframe application.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use eframe::egui;
use sabu_core::config::Config;
use sabu_core::state::AppState;

use crate::commands::{
    ClearDatasetCommand, Command, CommandDispatcher, CommandEvent, LoadDatasetCommand,
    SetLanguageCommand, SetThemeCommand,
};
use crate::i18n::{Translator, error_key};
use crate::ui::{
    AnalyticsCache, MenuAction, StatusMessage, TableCache, apply_theme, content_panel, status_bar,
    top_bar,
};

/// Main eframe application.
pub struct SabuApp {
    state: Arc<Mutex<AppState>>,
    dispatcher: CommandDispatcher,
    translator: Translator,
    path_input: String,
    status: Option<StatusMessage>,
    pending_commands: usize,
    cache: Option<AnalyticsCache>,
    display_config: Config,
    last_title: String,
}

impl SabuApp {
    /// Creates the application using the shared state and command worker.
    pub fn new(
        _creation_context: &eframe::CreationContext<'_>,
        state: Arc<Mutex<AppState>>,
        dispatcher: CommandDispatcher,
        translator: Translator,
    ) -> Self {
        let last_title = translator.t("app.title").to_owned();
        let display_config = state
            .try_lock()
            .map(|state| state.config)
            .unwrap_or_default();
        Self {
            state,
            dispatcher,
            translator,
            path_input: String::new(),
            status: None,
            pending_commands: 0,
            cache: None,
            display_config,
            last_title,
        }
    }

    fn open_path(&mut self) {
        let path = self.path_input.trim();
        if path.is_empty() {
            self.status = Some(StatusMessage::error("error.empty_path"));
            return;
        }
        self.dispatch(Box::new(LoadDatasetCommand::new(PathBuf::from(path))));
    }

    fn dispatch(&mut self, command: Box<dyn Command>) {
        match self.dispatcher.execute(command) {
            Ok(()) => self.pending_commands = self.pending_commands.saturating_add(1),
            Err(error) => {
                log::error!("could not dispatch command: {error}");
                self.status = Some(StatusMessage::error("error.worker_stopped"));
            }
        }
    }

    fn undo(&mut self) {
        match self.dispatcher.undo() {
            Ok(()) => self.pending_commands = self.pending_commands.saturating_add(1),
            Err(error) => {
                log::error!("could not dispatch undo: {error}");
                self.status = Some(StatusMessage::error("error.worker_stopped"));
            }
        }
    }

    fn redo(&mut self) {
        match self.dispatcher.redo() {
            Ok(()) => self.pending_commands = self.pending_commands.saturating_add(1),
            Err(error) => {
                log::error!("could not dispatch redo: {error}");
                self.status = Some(StatusMessage::error("error.worker_stopped"));
            }
        }
    }

    fn handle_menu_action(&mut self, action: MenuAction, context: &egui::Context) {
        match action {
            MenuAction::None => {}
            MenuAction::ClearData => {
                self.dispatch(Box::new(ClearDatasetCommand::new()));
            }
            MenuAction::Exit => {
                context.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            MenuAction::Undo => self.undo(),
            MenuAction::Redo => self.redo(),
            MenuAction::SetLanguage(language) => {
                self.dispatch(Box::new(SetLanguageCommand::new(language)));
            }
            MenuAction::SetTheme(theme) => {
                self.dispatch(Box::new(SetThemeCommand::new(theme)));
            }
        }
    }

    fn poll_events(&mut self) {
        for event in self.dispatcher.poll_events() {
            self.pending_commands = self.pending_commands.saturating_sub(1);
            match event {
                CommandEvent::Completed { label_key } => {
                    self.status = Some(StatusMessage::success(label_key));
                }
                CommandEvent::Failed { label_key, error } => {
                    log::error!("command {label_key} failed: {error}");
                    self.status = Some(StatusMessage::error(error_key(&error)));
                }
                CommandEvent::Undone { label_key } | CommandEvent::Redone { label_key } => {
                    self.status = Some(StatusMessage::info(label_key));
                }
                CommandEvent::NothingToUndo => {
                    self.status = Some(StatusMessage::info("status.nothing_to_undo"));
                }
                CommandEvent::NothingToRedo => {
                    self.status = Some(StatusMessage::info("status.nothing_to_redo"));
                }
            }
        }
    }

    fn handle_dropped_files(&mut self, context: &egui::Context) {
        let dropped = context.input(|input| input.raw.dropped_files.clone());
        let Some(path) = dropped.first().map(|file| file.path().to_path_buf()) else {
            return;
        };

        self.path_input = path.display().to_string();
        self.dispatch(Box::new(LoadDatasetCommand::new(path)));
    }

    fn refresh_cache(&mut self) {
        let state = match self.state.try_lock() {
            Ok(state) => state,
            Err(std::sync::TryLockError::WouldBlock) => return,
            Err(std::sync::TryLockError::Poisoned(_)) => {
                log::error!("application state lock is poisoned");
                return;
            }
        };

        let revision = state.dataset_revision();
        if self
            .cache
            .as_ref()
            .is_some_and(|cache| cache.revision == revision)
        {
            return;
        }

        self.cache = state.dataset.as_ref().map(|dataset| {
            let analytics = dataset.analytics();
            AnalyticsCache {
                revision,
                summary: dataset.summary(),
                by_cargo: TableCache::new(&analytics.by_cargo),
                by_votavel: TableCache::new(&analytics.by_votavel),
                by_partido: TableCache::new(&analytics.by_partido),
            }
        });
    }

    fn refresh_config(&mut self) -> Config {
        match self.state.try_lock() {
            Ok(state) => {
                self.display_config = state.config;
                self.display_config
            }
            Err(std::sync::TryLockError::WouldBlock) => self.display_config,
            Err(std::sync::TryLockError::Poisoned(_)) => {
                log::error!("application state lock is poisoned");
                self.status = Some(StatusMessage::error("error.state_poisoned"));
                self.display_config
            }
        }
    }

    fn update_title(&mut self, context: &egui::Context) {
        let title = self.translator.t("app.title").to_owned();
        if title != self.last_title {
            context.send_viewport_cmd(egui::ViewportCommand::Title(title.clone()));
            self.last_title = title;
        }
    }
}

impl eframe::App for SabuApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let context = ui.ctx().clone();

        let config = self.refresh_config();
        self.translator.set_language(config.language);
        apply_theme(&context, config.theme);
        self.poll_events();
        self.handle_dropped_files(&context);
        self.refresh_cache();
        self.update_title(&context);

        let action = top_bar(ui, &self.translator, config, &mut self.path_input);
        if action.open {
            self.open_path();
        }
        self.handle_menu_action(action.menu, &context);

        status_bar(ui, &self.translator, self.pending_commands, self.status);
        egui::CentralPanel::default().show(ui, |ui| {
            content_panel(ui, &self.translator, self.cache.as_ref());
        });

        if self.pending_commands > 0 {
            context.request_repaint_after(Duration::from_millis(50));
        }
    }
}
