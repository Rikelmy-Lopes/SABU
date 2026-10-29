//! Maps core errors to translation keys.

use sabu_core::CoreError;

/// Returns the translation key that describes an error.
pub fn error_key(error: &CoreError) -> &'static str {
    match error {
        CoreError::Io(_) => "error.io",
        CoreError::Csv(_) => "error.csv",
        CoreError::Json(_) => "error.json",
        CoreError::EmptyFile => "error.empty_file",
        CoreError::MissingColumns { .. } => "error.missing_columns",
        CoreError::InvalidRecordWidth { .. } => "error.invalid_record",
        CoreError::InvalidNumber { .. } => "error.invalid_number",
        CoreError::MissingLocalAppData => "error.missing_local_appdata",
        CoreError::InvalidLanguage(_) => "error.invalid_language",
        CoreError::StatePoisoned => "error.state_poisoned",
    }
}
