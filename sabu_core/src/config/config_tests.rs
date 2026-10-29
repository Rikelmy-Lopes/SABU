//! Tests for configuration loading, saving, and locale codes.

use super::{Config, Language, Theme};

#[test]
fn default_config_is_portuguese_and_dark() {
    let config = Config::default();
    assert_eq!(config.language, Language::PtBr);
    assert_eq!(config.theme, Theme::Dark);
}

#[test]
fn language_codes_round_trip() {
    for language in Language::ALL {
        let parsed = Language::from_code(language.code());
        assert_eq!(parsed.ok(), Some(language));
    }
}

#[test]
fn language_name_keys_are_stable() {
    assert_eq!(Language::PtBr.name_key(), "language.pt_br");
    assert_eq!(Language::En.name_key(), "language.en");
}

#[test]
fn unknown_language_code_is_rejected() {
    assert!(Language::from_code("fr").is_err());
}

#[test]
fn missing_config_file_returns_defaults() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let config = Config::load(&directory.path().join("config.json"));
    assert_eq!(config.ok(), Some(Config::default()));
}

#[test]
fn config_round_trips_through_json_file() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let path = directory.path().join("nested").join("config.json");
    let expected = Config {
        language: Language::En,
        theme: Theme::Light,
    };

    assert!(expected.save(&path).is_ok());
    assert_eq!(Config::load(&path).ok(), Some(expected));
}

#[test]
fn invalid_config_file_is_an_error() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let Ok(directory) = directory else {
        return;
    };

    let path = directory.path().join("config.json");
    assert!(std::fs::write(&path, "{ not json").is_ok());
    assert!(Config::load(&path).is_err());
}

#[test]
fn config_json_uses_locale_code_for_language() {
    let serialized = serde_json::to_string(&Config {
        language: Language::PtBr,
        theme: Theme::Light,
    });
    assert!(serialized.is_ok());
    let Ok(serialized) = serialized else {
        return;
    };

    assert!(serialized.contains("\"pt-BR\""));
    assert!(serialized.contains("\"light\""));
}
