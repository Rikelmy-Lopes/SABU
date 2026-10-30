use serde::{Deserialize, Serialize};

use crate::config::language::Language;
use crate::config::theme::Theme;

/// Configuration persisted in `%LOCALAPPDATA%\Sabu\config.json`.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct UserConfig {
    /// Active interface language.
    pub language: Language,
    /// Active visual theme.
    pub theme: Theme,
}
