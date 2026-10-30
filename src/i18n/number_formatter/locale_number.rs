use sabu_core::config::Language;

/// Formats integers according to the active language.
pub struct LocaleNumber;

impl LocaleNumber {
    /// Formats an integer with the thousands separator of `language`.
    pub fn format_integer(language: Language, value: i64) -> String {
        let separator = match language {
            Language::PtBr => '.',
        };

        let digits = value.unsigned_abs().to_string();
        let mut formatted = String::with_capacity(digits.len() + digits.len() / 3 + 1);

        for (index, digit) in digits.chars().enumerate() {
            if index > 0 && (digits.len() - index).is_multiple_of(3) {
                formatted.push(separator);
            }

            formatted.push(digit);
        }

        if value < 0 {
            format!("-{formatted}")
        } else {
            formatted
        }
    }
}
