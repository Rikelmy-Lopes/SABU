use std::sync::Arc;

use eframe::egui;
use sabu_core::config::config_store::ConfigStore;
use sabu_core::config::user_config::UserConfig;
use sabu_core::paths::AppPaths;
use sabu_core::state::app_state::AppState;

use crate::commands::dispatch::CommandDispatcher;
use crate::i18n::translator::Translator;
use crate::startup::logging;
use crate::startup::startup_error::StartupError;
use crate::ui::sabu_app::MainWindow;

/// Starts SABU.
pub fn run() -> Result<(), StartupError> {
    let paths = AppPaths::discover()?;
    logging::init(&paths)?;

    log::info!("SABU {} starting", env!("CARGO_PKG_VERSION"));

    let config_store = ConfigStore::new(paths.config_file());

    let config = match config_store.load() {
        Ok(config) => config,
        Err(error) => {
            log::warn!("using the default configuration: {error}");
            UserConfig::default()
        }
    };

    let translator = Translator::new(config.language)?;
    let state = AppState::new(config);
    let dispatcher = CommandDispatcher::spawn(state.clone(), config_store)?;
    let title = translator.text("app.title");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(title)
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([800.0, 500.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Sabu",
        options,
        Box::new(move |creation_context| {
            Ok(Box::new(MainWindow::new(
                creation_context,
                dispatcher,
                translator,
                Arc::new(state),
            )))
        }),
    )?;

    Ok(())
}
