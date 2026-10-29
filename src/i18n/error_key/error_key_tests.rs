//! Tests for the error key mapping.

use sabu_core::CoreError;

use super::error_key;

#[test]
fn maps_every_core_error() {
    assert_eq!(error_key(&CoreError::EmptyFile), "error.empty_file");
    assert_eq!(error_key(&CoreError::StatePoisoned), "error.state_poisoned");
    assert_eq!(
        error_key(&CoreError::MissingLocalAppData),
        "error.missing_local_appdata"
    );
    assert_eq!(
        error_key(&CoreError::InvalidLanguage("xx".to_owned())),
        "error.invalid_language"
    );
}
