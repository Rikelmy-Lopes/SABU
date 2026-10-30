use std::fs;

use tempfile::tempdir;

use crate::config::config_store::ConfigStore;
use crate::config::language::Language;
use crate::config::theme::Theme;
use crate::config::user_config::UserConfig;

#[test]
fn defaults_to_portuguese_and_dark_theme() {
    let config = UserConfig::default();

    assert_eq!(config.language, Language::PtBr);
    assert_eq!(config.theme, Theme::Dark);
}

#[test]
fn missing_file_returns_defaults() {
    let dir = tempdir().unwrap();
    let store = ConfigStore::new(dir.path().join("Sabu/config.json"));

    assert_eq!(store.load().unwrap(), UserConfig::default());
}

#[test]
fn saves_and_loads_configuration() {
    let dir = tempdir().unwrap();
    let store = ConfigStore::new(dir.path().join("Sabu/config.json"));
    let config = UserConfig {
        language: Language::PtBr,
        theme: Theme::Light,
    };

    store.save(&config).unwrap();

    assert_eq!(store.load().unwrap(), config);
}

#[test]
fn reports_invalid_configuration() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("config.json");
    fs::write(&path, "{ not json").unwrap();

    let store = ConfigStore::new(path);

    assert!(store.load().is_err());
}

#[test]
fn language_codes_are_stable() {
    assert_eq!(Language::PtBr.code(), "pt-BR");
    assert_eq!(Language::from_code("pt-BR"), Some(Language::PtBr));
    assert_eq!(Language::from_code("en"), None);
}
