use std::fs;
use std::path::{Path, PathBuf};

use crate::config::user_config::UserConfig;
use crate::error::core_error::{CoreError, Result};

/// Reads and writes [`UserConfig`] as JSON.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigStore {
    path: PathBuf,
}

impl ConfigStore {
    /// Creates a store that writes to `path`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// File managed by this store.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Loads the configuration, returning defaults when the file does not exist.
    pub fn load(&self) -> Result<UserConfig> {
        if !self.path.exists() {
            return Ok(UserConfig::default());
        }

        let contents = fs::read_to_string(&self.path)?;

        serde_json::from_str(&contents).map_err(|error| CoreError::Configuration(error.to_string()))
    }

    /// Persists the configuration, creating its parent folder when needed.
    pub fn save(&self, config: &UserConfig) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(config)
            .map_err(|error| CoreError::Configuration(error.to_string()))?;

        fs::write(&self.path, json)?;

        Ok(())
    }
}
