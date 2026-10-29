//! Tests for the CSV parser.

use crate::error::CoreError;

use super::parse_bytes;

/// Header row with all mandatory columns.
const HEADER: &str = "ANO_ELEICAO;NR_TURNO;SG_UF;NM_MUNICIPIO;NR_ZONA;NR_SECAO;\
DS_CARGO_PERGUNTA;SG_PARTIDO;DS_TIPO_VOTAVEL;NR_VOTAVEL;NM_VOTAVEL;QT_VOTOS";

fn mock_csv(rows: &[&str]) -> Vec<u8> {
    let mut csv = String::from(HEADER);
    csv.push('\n');
    for row in rows {
        csv.push_str(row);
        csv.push('\n');
    }
    csv.into_bytes()
}

#[test]
fn parses_the_sample_dataset() {
    let bytes = include_bytes!("../../../example/data.csv");
    let records = parse_bytes(bytes);
    assert!(records.is_ok());
    let Ok(records) = records else {
        return;
    };

    assert_eq!(records.len(), 49);
    let first = &records[0];
    assert_eq!(first.ano_eleicao, Some(2026));
    assert_eq!(first.nr_turno, Some(2));
    assert_eq!(first.uf, "AC");
    assert_eq!(first.municipio, "SENADOR GUIOMARD");
    assert_eq!(first.cargo, "Presidente");
    assert_eq!(first.partido, "PT");
    assert_eq!(first.nm_votavel, "LULA");
    assert_eq!(first.nr_votavel, Some(13));
    assert_eq!(first.qt_votos, Some(43));
}

#[test]
fn decodes_latin1_accented_characters() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(HEADER.as_bytes());
    bytes.push(b'\n');
    bytes.extend_from_slice(b"2026;1;SP;Concei\xE7\xE3o;1;2;Vereador;PT;Nominal;13;Jo\xE3o;10\n");

    let records = parse_bytes(&bytes);
    assert!(records.is_ok());
    let Ok(records) = records else {
        return;
    };

    assert_eq!(records[0].municipio, "Conceição");
    assert_eq!(records[0].nm_votavel, "João");
}

#[test]
fn maps_null_and_not_registered_numbers_to_none() {
    let bytes = mock_csv(&[
        "-1;1;SP;São Paulo;1;2;Vereador;PT;Nominal;#NULO;João;#NE",
        "#NULO;1;SP;São Paulo;1;2;Vereador;PT;Nominal;13;João;-3",
    ]);
    let records = parse_bytes(&bytes);
    assert!(records.is_ok());
    let Ok(records) = records else {
        return;
    };

    assert_eq!(records[0].ano_eleicao, None);
    assert_eq!(records[0].nr_votavel, None);
    assert_eq!(records[0].qt_votos, None);
    assert_eq!(records[1].ano_eleicao, None);
    assert_eq!(records[1].qt_votos, None);
}

#[test]
fn reports_missing_mandatory_columns() {
    let bytes = b"ANO_ELEICAO;SG_UF\n2026;SP\n";
    let result = parse_bytes(bytes);
    match result {
        Err(CoreError::MissingColumns { columns }) => {
            assert!(columns.contains("QT_VOTOS"));
            assert!(columns.contains("NM_VOTAVEL"));
        }
        other => assert!(other.is_err()),
    }
}

#[test]
fn rejects_empty_files() {
    assert!(matches!(parse_bytes(b""), Err(CoreError::EmptyFile)));
}

#[test]
fn rejects_records_that_are_too_short() {
    let bytes = mock_csv(&["2026;1;SP"]);
    assert!(matches!(
        parse_bytes(&bytes),
        Err(CoreError::InvalidRecordWidth { .. })
    ));
}

#[test]
fn rejects_invalid_numbers_with_line_information() {
    let bytes = mock_csv(&["2026;1;SP;São Paulo;1;2;Vereador;PT;Nominal;13;João;not-a-number"]);
    match parse_bytes(&bytes) {
        Err(CoreError::InvalidNumber {
            column,
            line,
            value,
        }) => {
            assert_eq!(column, "QT_VOTOS");
            assert_eq!(line, 2);
            assert_eq!(value, "not-a-number");
        }
        other => assert!(other.is_err()),
    }
}

#[test]
fn maps_columns_by_name_instead_of_position() {
    let header = "QT_VOTOS;NM_VOTAVEL;NR_VOTAVEL;DS_TIPO_VOTAVEL;SG_PARTIDO;\
DS_CARGO_PERGUNTA;NR_SECAO;NR_ZONA;NM_MUNICIPIO;SG_UF;NR_TURNO;ANO_ELEICAO";
    let row = "25;Maria;12;Nominal;PSDB;Prefeito;8;7;Campinas;SP;1;2024";
    let csv = format!("{header}\n{row}\n");

    let records = parse_bytes(csv.as_bytes());
    assert!(records.is_ok());
    let Ok(records) = records else {
        return;
    };

    assert_eq!(records[0].uf, "SP");
    assert_eq!(records[0].municipio, "Campinas");
    assert_eq!(records[0].nm_votavel, "Maria");
    assert_eq!(records[0].qt_votos, Some(25));
}

#[test]
fn trims_whitespace_around_fields() {
    let bytes = mock_csv(&[
        " 2026 ; 1 ; SP ; Campinas ; 1 ; 2 ; Prefeito ; PSDB ; Nominal ; 12 ; Maria ; 25 ",
    ]);
    let records = parse_bytes(&bytes);
    assert!(records.is_ok());
    let Ok(records) = records else {
        return;
    };

    assert_eq!(records[0].uf, "SP");
    assert_eq!(records[0].municipio, "Campinas");
    assert_eq!(records[0].qt_votos, Some(25));
}
