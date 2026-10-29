//! Locale-aware integer formatting.

use sabu_core::config::Language;

/// Formats an integer using the separators of the given language.
pub(crate) fn format_number(language: Language, value: u64) -> String {
    let digits = value.to_string();
    let separator = match language {
        Language::PtBr => ".",
        Language::En => ",",
    };

    let mut formatted = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            formatted.push_str(separator);
        }
        formatted.push(digit);
    }
    formatted
}
