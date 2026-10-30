use sabu_core::config::Language;

use crate::i18n::number_formatter::LocaleNumber;
use crate::i18n::translator::{TextResolver, Translator};

#[test]
fn loads_brazilian_portuguese_catalog() {
    let translator = Translator::new(Language::PtBr).unwrap();

    assert_eq!(translator.language(), Language::PtBr);
    assert!(!translator.text("app.title").is_empty());
    assert_eq!(translator.text("summary.municipalities"), "Municípios");
    assert_eq!(translator.text("status.ready"), "Pronto.");
}

#[test]
fn returns_the_key_when_missing_everywhere() {
    let translator = Translator::new(Language::PtBr).unwrap();

    assert_eq!(translator.text("missing.key"), "missing.key");
}

#[test]
fn prefers_active_text_and_falls_back_when_absent() {
    assert_eq!(
        TextResolver::resolve(Some("Ativo"), Some("Reserva"), "key"),
        "Ativo"
    );
    assert_eq!(
        TextResolver::resolve(None, Some("Reserva"), "key"),
        "Reserva"
    );
    assert_eq!(TextResolver::resolve(None, None, "key"), "key");
}

#[test]
fn interpolates_placeholders() {
    let translator = Translator::new(Language::PtBr).unwrap();
    let text = translator.text_with("status.loaded", &[("count", "1.234")]);

    assert!(text.contains("1.234"));
    assert!(!text.contains("{count}"));
}

#[test]
fn formats_numbers_for_brazilian_portuguese() {
    assert_eq!(LocaleNumber::format_integer(Language::PtBr, 0), "0");
    assert_eq!(LocaleNumber::format_integer(Language::PtBr, 999), "999");
    assert_eq!(LocaleNumber::format_integer(Language::PtBr, 1_000), "1.000");
    assert_eq!(
        LocaleNumber::format_integer(Language::PtBr, 2_045_664),
        "2.045.664"
    );
    assert_eq!(
        LocaleNumber::format_integer(Language::PtBr, -1_234_567),
        "-1.234.567"
    );
}
