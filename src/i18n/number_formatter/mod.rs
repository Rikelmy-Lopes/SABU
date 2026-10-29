//! Number formatting for the active language.

mod locale_number;

pub(crate) use locale_number::format_number;

#[cfg(test)]
#[path = "number_formatter_tests.rs"]
mod number_formatter_tests;
