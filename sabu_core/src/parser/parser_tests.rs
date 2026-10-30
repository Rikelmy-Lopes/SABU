use std::path::PathBuf;

use encoding_rs::WINDOWS_1252;

use crate::error::core_error::CoreError;
use crate::parser::columns::{COLUMN_COUNT, EXPECTED_HEADERS};
use crate::parser::csv_parser::CsvParser;

fn header_line() -> String {
    EXPECTED_HEADERS
        .iter()
        .map(|header| format!("\"{header}\""))
        .collect::<Vec<_>>()
        .join(";")
}

fn base_fields() -> Vec<String> {
    vec!["0".to_owned(); COLUMN_COUNT]
}

fn set(fields: &mut [String], index: usize, value: &str) {
    fields[index] = value.to_owned();
}

fn quoted_row(fields: &[String]) -> String {
    fields
        .iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>()
        .join(";")
}

fn document(rows: &[Vec<String>]) -> String {
    let mut lines = vec![header_line()];
    lines.extend(rows.iter().map(|fields| quoted_row(fields)));
    lines.join("\r\n")
}

fn accented_row() -> Vec<String> {
    let mut fields = base_fields();
    set(&mut fields, 4, "Eleição Ordinária");
    set(&mut fields, 9, "Eleição Geral Federal 2026");
    set(&mut fields, 10, "SP");
    set(&mut fields, 12, "São Paulo");
    set(&mut fields, 17, "Deputado Federal");
    set(&mut fields, 20, "Partido dos Trabalhadores");
    set(&mut fields, 30, "João da Silva");
    set(&mut fields, 31, "150");
    set(&mut fields, 32, "2045664");

    fields
}

#[test]
fn parses_accented_latin1_document() {
    let text = document(&[accented_row()]);
    let (bytes, _, had_errors) = WINDOWS_1252.encode(&text);

    assert!(!had_errors);

    let records = CsvParser::parse_bytes(&bytes).unwrap();

    assert_eq!(records.len(), 1);
    assert_eq!(records[0].election_type_name, "Eleição Ordinária");
    assert_eq!(records[0].municipality_name, "São Paulo");
    assert_eq!(records[0].party_name, "Partido dos Trabalhadores");
    assert_eq!(records[0].votable_name, "João da Silva");
    assert_eq!(records[0].office_name, "Deputado Federal");
    assert_eq!(records[0].votes, Some(150));
    assert_eq!(records[0].ballot_box_number, Some(2045664));
}

#[test]
fn maps_null_markers_to_none() {
    let mut fields = base_fields();
    set(&mut fields, 9, "#NULO");
    set(&mut fields, 22, "-1");
    set(&mut fields, 23, "#NULO");
    set(&mut fields, 24, "#NE");
    set(&mut fields, 31, "#NULO#");
    set(&mut fields, 38, "#NULO#");
    set(&mut fields, 43, "-1");
    set(&mut fields, 44, "-3");

    let records = CsvParser::parse_str(&document(&[fields])).unwrap();
    let record = &records[0];

    assert_eq!(record.election_description, "");
    assert_eq!(record.eligible_voters, None);
    assert_eq!(record.turnout, None);
    assert_eq!(record.abstentions, None);
    assert_eq!(record.votes, None);
    assert_eq!(record.aggregated_sections, None);
    assert_eq!(record.counting_board_number, None);
    assert_eq!(record.counting_team_number, None);
}

#[test]
fn rejects_unknown_header() {
    let mut header = header_line().replacen("\"DT_GERACAO\"", "\"WRONG\"", 1);
    header.push_str("\r\n");
    header.push_str(&quoted_row(&base_fields()));

    let error = CsvParser::parse_str(&header).unwrap_err();

    assert!(matches!(
        error,
        CoreError::InvalidHeader {
            index: 0,
            expected: "DT_GERACAO",
            ..
        }
    ));
}

#[test]
fn rejects_rows_with_missing_columns() {
    let mut fields = base_fields();
    fields.pop();

    let error = CsvParser::parse_str(&document(&[fields])).unwrap_err();

    assert!(matches!(
        error,
        CoreError::MissingColumn {
            column: "NR_TURMA_APURADORA",
            ..
        }
    ));
}

#[test]
fn parses_example_file() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../example/data.csv");
    let records = CsvParser::parse_file(&path).unwrap();

    assert!(!records.is_empty());
    assert_eq!(records[0].state_code, "AC");
    assert_eq!(records[0].election_type_name, "Eleição Ordinária");
    assert_eq!(records[0].votable_name, "LULA");
    assert_eq!(records[0].votes, Some(43));
}
