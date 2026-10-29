//! Log file initialization.

use std::fs::OpenOptions;

use log::LevelFilter;
use sabu_core::paths::AppPaths;

use super::StartupError;

/// Opens `logs.log` and installs the global logger.
pub(super) fn init(paths: &AppPaths) -> Result<(), StartupError> {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(paths.log_file())?;
    simplelog::WriteLogger::init(LevelFilter::Info, simplelog::Config::default(), log_file)?;
    Ok(())
}
