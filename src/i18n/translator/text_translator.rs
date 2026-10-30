use sabu_core::config::Language;

use crate::i18n::dictionary::TranslationMap;
use crate::i18n::error::I18nError;
use crate::i18n::language_catalog::BundledLanguages;
use crate::i18n::translator::TextResolver;

/// Resolves interface text for the active language.
#[derive(Debug, Clone)]
pub struct Translator {
    language: Language,
    active: TranslationMap,
    fallback: TranslationMap,
}

impl Translator {
    /// Loads the active catalog and the fallback catalog.
    pub fn new(language: Language) -> Result<Self, I18nError> {
        let fallback_language = BundledLanguages::fallback();
        let fallback_json = BundledLanguages::json_for(fallback_language)
            .ok_or(I18nError::MissingTranslations(fallback_language.code()))?;
        let fallback = TranslationMap::from_json(fallback_language, fallback_json)?;

        let active = if language == fallback_language {
            fallback.clone()
        } else {
            match BundledLanguages::json_for(language) {
                Some(json) => TranslationMap::from_json(language, json)?,
                None => fallback.clone(),
            }
        };

        Ok(Self {
            language,
            active,
            fallback,
        })
    }

    /// Active language.
    pub fn language(&self) -> Language {
        self.language
    }

    /// Returns the text for `key`, falling back to `pt-BR` and then to the key.
    pub fn text(&self, key: &str) -> String {
        TextResolver::resolve(self.active.get(key), self.fallback.get(key), key)
    }

    /// Returns the text for `key` with `{name}` placeholders replaced.
    pub fn text_with(&self, key: &str, arguments: &[(&str, &str)]) -> String {
        TextResolver::interpolate(&self.text(key), arguments)
    }
}
