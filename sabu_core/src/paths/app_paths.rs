//! Resolution and creation of the runtime directories.

use std::env;
use std::path::{Path, PathBuf};

use crate::error::CoreError;

/// Directory that holds the runtime files.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppPaths {
    root: PathBuf,
}

impl AppPaths {
    /// Resolves `%LOCALAPPDATA%\Sabu`.
    pub fn discover() -> Result<Self, CoreError> {
        let local_app_data = env::var_os("LOCALAPPDATA").ok_or(CoreError::MissingLocalAppData)?;
        Ok(Self::with_root(PathBuf::from(local_app_data).join("Sabu")))
    }

    /// Builds paths from an explicit root directory.
    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }

    /// Root directory for SABU runtime files.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Path of the configuration file.
    pub fn config_file(&self) -> PathBuf {
        self.root.join("config.json")
    }

    /// Path of the log file.
    pub fn log_file(&self) -> PathBuf {
        self.root.join("logs.log")
    }

    /// Creates the root directory when it does not exist.
    pub fn ensure(&self) -> Result<(), CoreError> {
        std::fs::create_dir_all(&self.root)?;
        Ok(())
    }
}
