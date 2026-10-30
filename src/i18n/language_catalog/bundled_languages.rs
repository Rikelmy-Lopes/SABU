use sabu_core::config::Language;

/// Maps a [`Language`] to its embedded JSON catalog.
pub struct BundledLanguages;

impl BundledLanguages {
    /// JSON source for `language`, when the build includes it.
    pub fn json_for(language: Language) -> Option<&'static str> {
        match language {
            Language::PtBr => Some(include_str!("../../../locales/pt-BR.json")),
        }
    }

    /// Language used when a key is missing in the active catalog.
    pub fn fallback() -> Language {
        Language::PtBr
    }
}
