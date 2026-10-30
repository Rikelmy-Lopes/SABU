use std::fs;
use std::path::{Path, PathBuf};

use crate::error::core_error::{CoreError, Result};

/// Runtime folders and files owned by SABU.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppPaths {
    root: PathBuf,
}

impl AppPaths {
    /// Resolves `%LOCALAPPDATA%\Sabu`.
    pub fn discover() -> Result<Self> {
        let base = dirs::data_local_dir().ok_or(CoreError::DataDirectoryUnavailable)?;

        Ok(Self {
            root: base.join("Sabu"),
        })
    }

    /// Uses an explicit root folder. Useful for tests and portable runs.
    pub fn with_root(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Root folder.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Log file path.
    pub fn logs_file(&self) -> PathBuf {
        self.root.join("logs.log")
    }

    /// Configuration file path.
    pub fn config_file(&self) -> PathBuf {
        self.root.join("config.json")
    }

    /// Creates the root folder when it does not exist.
    pub fn ensure_exists(&self) -> Result<()> {
        fs::create_dir_all(&self.root)?;

        Ok(())
    }
}
