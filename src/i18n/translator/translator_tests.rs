//! Tests for the translation loader.

use std::collections::HashMap;

use sabu_core::config::Language;

use crate::i18n::I18nError;

use super::Translator;

#[test]
fn loads_the_default_portuguese_dictionary() {
    let translator = Translator::load(Language::PtBr);
    assert!(translator.is_ok());
    let Ok(translator) = translator else {
        return;
    };

    assert_eq!(translator.active_language(), Language::PtBr);
    assert_eq!(translator.t("app.path.open"), "Abrir");
    assert!(translator.t("app.title").contains("SABU"));
}

#[test]
fn exposes_only_bundled_languages() {
    let translator = Translator::load(Language::PtBr);
    assert!(translator.is_ok());
    let Ok(translator) = translator else {
        return;
    };

    let available = translator.available_languages().collect::<Vec<_>>();
    assert_eq!(available, vec![Language::PtBr]);
}

#[test]
fn ignores_a_language_without_a_bundled_translation() {
    let translator = Translator::load(Language::PtBr);
    assert!(translator.is_ok());
    let Ok(mut translator) = translator else {
        return;
    };

    translator.set_language(Language::En);
    assert_eq!(translator.active_language(), Language::PtBr);
    assert_eq!(translator.t("app.path.open"), "Abrir");
}

#[test]
fn falls_back_to_portuguese_when_the_active_language_is_unavailable() {
    let translator = Translator::load(Language::En);
    assert!(translator.is_ok());
    let Ok(translator) = translator else {
        return;
    };

    assert_eq!(translator.active_language(), Language::PtBr);
}

#[test]
fn falls_back_to_portuguese_for_missing_keys() {
    let mut pt_br = HashMap::new();
    pt_br.insert("only.pt".to_owned(), "Português".to_owned());

    let mut dictionaries = HashMap::new();
    dictionaries.insert(Language::PtBr, pt_br);

    let translator = Translator::from_dictionaries(Language::En, dictionaries);
    assert_eq!(translator.t("only.pt"), "Português");
    assert_eq!(translator.t("missing.key"), "missing.key");
}

#[test]
fn exposes_json_errors() {
    let result: Result<HashMap<String, String>, serde_json::Error> =
        serde_json::from_str("{ invalid");
    let Err(json_error) = result else {
        assert!(result.is_err());
        return;
    };

    let error = I18nError::Json(json_error);
    assert!(!error.to_string().is_empty());
}
