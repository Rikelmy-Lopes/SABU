use serde::{Deserialize, Serialize};

/// Languages supported by SABU.
///
/// Only Brazilian Portuguese ships today, but the enum and the JSON catalog
/// are prepared for more languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Language {
    /// Brazilian Portuguese.
    #[default]
    #[serde(rename = "pt-BR")]
    PtBr,
}

impl Language {
    /// Every language known by this build.
    pub const ALL: [Language; 1] = [Language::PtBr];

    /// Stable language code used in `config.json` and in locale file names.
    pub fn code(self) -> &'static str {
        match self {
            Language::PtBr => "pt-BR",
        }
    }

    /// Resolves a language from its stable code.
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "pt-BR" => Some(Language::PtBr),
            _ => None,
        }
    }
}
