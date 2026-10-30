use std::collections::HashMap;

use sabu_core::config::Language;

use crate::i18n::error::I18nError;

/// Flat key/value dictionary loaded from a locale JSON file.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TranslationMap {
    entries: HashMap<String, String>,
}

impl TranslationMap {
    /// Parses a locale JSON object.
    pub fn from_json(language: Language, json: &str) -> Result<Self, I18nError> {
        let entries: HashMap<String, String> =
            serde_json::from_str(json).map_err(|source| I18nError::InvalidTranslations {
                language: language.code(),
                source,
            })?;

        Ok(Self { entries })
    }

    /// Looks up a key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(String::as_str)
    }
}
