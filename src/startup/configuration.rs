//! Configuration loading.

use sabu_core::config::Config;
use sabu_core::paths::AppPaths;

/// Loads the configuration, falling back to defaults when it is invalid.
pub(super) fn load(paths: &AppPaths) -> Config {
    match Config::load(&paths.config_file()) {
        Ok(config) => config,
        Err(error) => {
            log::warn!("could not load configuration, using defaults: {error}");
            Config::default()
        }
    }
}
