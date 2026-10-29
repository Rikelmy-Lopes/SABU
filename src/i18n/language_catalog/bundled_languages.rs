//! Translation files compiled into the application.

use sabu_core::config::Language;

/// One translation file bundled into the application.
pub(crate) struct BundledLanguage {
    /// Language provided by the file.
    pub(crate) language: Language,
    /// JSON contents embedded at compile time.
    pub(crate) json: &'static str,
}

/// Translation files compiled into the application.
///
/// To add a language, create its locale file and register it here. Only
/// languages registered in this list are loaded and offered by the interface.
///
/// Currently only Brazilian Portuguese is bundled. Do not register a language
/// until its translation file is fully implemented.
pub(crate) const BUNDLED_LANGUAGES: &[BundledLanguage] = &[BundledLanguage {
    language: Language::PtBr,
    json: include_str!("../../../locales/pt-BR.json"),
}];
