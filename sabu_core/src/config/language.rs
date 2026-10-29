//! Language identifiers and their locale codes.

use serde::{Deserialize, Serialize};

use crate::error::CoreError;

/// Languages supported by the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Language {
    /// Brazilian Portuguese, the default language.
    #[default]
    #[serde(rename = "pt-BR")]
    PtBr,
    /// English, reserved for future activation.
    #[serde(rename = "en")]
    En,
}

impl Language {
    /// Every language known by the application.
    pub const ALL: [Language; 2] = [Language::PtBr, Language::En];

    /// Returns the locale code used in configuration and translation files.
    pub fn code(self) -> &'static str {
        match self {
            Language::PtBr => "pt-BR",
            Language::En => "en",
        }
    }

    /// Returns the translation key with the language's display name.
    pub fn name_key(self) -> &'static str {
        match self {
            Language::PtBr => "language.pt_br",
            Language::En => "language.en",
        }
    }

    /// Parses a locale code.
    pub fn from_code(code: &str) -> Result<Self, CoreError> {
        match code {
            "pt-BR" => Ok(Language::PtBr),
            "en" => Ok(Language::En),
            other => Err(CoreError::InvalidLanguage(other.to_owned())),
        }
    }
}
