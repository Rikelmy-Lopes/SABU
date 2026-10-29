//! Tests for locale-aware number formatting.

use sabu_core::config::Language;

use super::format_number;

#[test]
fn formats_numbers_for_portuguese() {
    assert_eq!(format_number(Language::PtBr, 0), "0");
    assert_eq!(format_number(Language::PtBr, 999), "999");
    assert_eq!(format_number(Language::PtBr, 1_000), "1.000");
    assert_eq!(format_number(Language::PtBr, 1_234_567), "1.234.567");
}

#[test]
fn formats_numbers_for_english() {
    assert_eq!(format_number(Language::En, 1_000), "1,000");
    assert_eq!(format_number(Language::En, 1_234_567), "1,234,567");
}
