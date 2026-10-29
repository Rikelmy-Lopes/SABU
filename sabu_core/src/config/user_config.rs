//! Reading and writing the user configuration file.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::CoreError;

use super::{Language, Theme};

/// User configuration saved between sessions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    /// Active interface language.
    pub language: Language,
    /// Active visual theme.
    pub theme: Theme,
}

impl Config {
    /// Loads the configuration from `path`.
    ///
    /// A missing file is not an error; the defaults are returned instead.
    pub fn load(path: &Path) -> Result<Self, CoreError> {
        match fs::read_to_string(path) {
            Ok(contents) => Ok(serde_json::from_str(&contents)?),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(error) => Err(error.into()),
        }
    }

    /// Saves the configuration to `path`, creating parent directories.
    pub fn save(&self, path: &Path) -> Result<(), CoreError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut contents = serde_json::to_string_pretty(self)?;
        contents.push('\n');
        fs::write(path, contents)?;
        Ok(())
    }
}
