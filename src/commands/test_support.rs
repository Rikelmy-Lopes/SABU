//! Shared helpers for command tests.

use std::path::{Path, PathBuf};

use sabu_core::config::Config;
use sabu_core::state::AppState;

/// Path of the bundled sample dataset.
pub(crate) fn sample_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("example")
        .join("data.csv")
}

/// Application state configured inside a temporary directory.
pub(crate) fn state_in(directory: &tempfile::TempDir) -> AppState {
    AppState::new(Config::default(), directory.path().join("config.json"))
}
