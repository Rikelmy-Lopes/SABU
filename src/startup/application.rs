//! Startup sequence.

use std::sync::{Arc, Mutex};

use sabu_core::paths::AppPaths;
use sabu_core::state::AppState;

use crate::commands::CommandDispatcher;
use crate::i18n::Translator;

use super::StartupError;
use super::{configuration, logging, window};

/// Prepares paths, logging, configuration, and the command worker.
pub(crate) fn run() -> Result<(), StartupError> {
    let paths = AppPaths::discover()?;
    paths.ensure()?;
    logging::init(&paths)?;
    log::info!("starting SABU {}", env!("CARGO_PKG_VERSION"));

    let mut config = configuration::load(&paths);
    let translator = Translator::load(config.language)?;
    config.language = translator.active_language();
    let state = Arc::new(Mutex::new(AppState::new(config, paths.config_file())));
    let dispatcher = CommandDispatcher::spawn(Arc::clone(&state))?;

    window::run(state, dispatcher, translator)?;

    log::info!("SABU stopped");
    Ok(())
}
