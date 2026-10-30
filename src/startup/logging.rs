use std::fs::OpenOptions;

use sabu_core::paths::AppPaths;
use simplelog::{ConfigBuilder, LevelFilter, WriteLogger};

use crate::startup::startup_error::StartupError;

/// Starts file logging in `%LOCALAPPDATA%\Sabu\logs.log`.
pub fn init(paths: &AppPaths) -> Result<(), StartupError> {
    paths.ensure_exists()?;

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(paths.logs_file())
        .map_err(sabu_core::CoreError::from)?;

    let config = ConfigBuilder::new().build();

    WriteLogger::init(LevelFilter::Info, config, file)
        .map_err(|error| StartupError::Logging(error.to_string()))?;

    Ok(())
}
