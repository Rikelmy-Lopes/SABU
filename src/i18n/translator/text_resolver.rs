//! JSON translation loader with a Portuguese fallback.

use std::collections::HashMap;

use sabu_core::config::Language;

use crate::i18n::I18nError;
use crate::i18n::dictionary::Dictionary;
use crate::i18n::language_catalog::BUNDLED_LANGUAGES;
use crate::i18n::number_formatter::format_number;

/// Resolves translation keys for the active language.
#[derive(Debug)]
pub struct Translator {
    active: Language,
    dictionaries: HashMap<Language, Dictionary>,
}

impl Translator {
    /// Loads every bundled dictionary and selects the active language.
    ///
    /// If the requested language is not bundled yet, Brazilian Portuguese is
    /// used instead.
    pub fn load(active: Language) -> Result<Self, I18nError> {
        let mut dictionaries = HashMap::new();
        for bundled in BUNDLED_LANGUAGES {
            let dictionary = serde_json::from_str(bundled.json)?;
            dictionaries.insert(bundled.language, dictionary);
        }

        let active = if dictionaries.contains_key(&active) {
            active
        } else {
            log::warn!(
                "translation for {} is not available; using {}",
                active.code(),
                Language::PtBr.code()
            );
            Language::PtBr
        };

        Ok(Self {
            active,
            dictionaries,
        })
    }

    /// Changes the active language when its translation is bundled.
    pub fn set_language(&mut self, language: Language) {
        if self.dictionaries.contains_key(&language) {
            self.active = language;
        }
    }

    /// Returns the active language.
    pub fn active_language(&self) -> Language {
        self.active
    }

    /// Returns the languages that have a bundled translation.
    pub fn available_languages(&self) -> impl Iterator<Item = Language> + '_ {
        BUNDLED_LANGUAGES.iter().map(|bundled| bundled.language)
    }

    /// Translates a key, falling back to Portuguese and then to the key itself.
    pub fn t<'a>(&'a self, key: &'a str) -> &'a str {
        self.dictionaries
            .get(&self.active)
            .and_then(|dictionary| dictionary.get(key))
            .or_else(|| {
                self.dictionaries
                    .get(&Language::PtBr)
                    .and_then(|dictionary| dictionary.get(key))
            })
            .map(String::as_str)
            .unwrap_or(key)
    }

    /// Formats an integer using the separators of the active language.
    pub fn format_number(&self, value: u64) -> String {
        format_number(self.active, value)
    }

    #[cfg(test)]
    pub(crate) fn from_dictionaries(
        active: Language,
        dictionaries: HashMap<Language, Dictionary>,
    ) -> Self {
        Self {
            active,
            dictionaries,
        }
    }
}
