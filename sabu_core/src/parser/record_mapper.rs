use csv::StringRecord;

use crate::error::core_error::{CoreError, Result};
use crate::model::bulletin_record::BulletinRecord;
use crate::parser::columns;
use crate::parser::numbers;

/// Converts one raw CSV row into a typed [`BulletinRecord`].
pub(crate) fn map_record(record: &StringRecord) -> Result<BulletinRecord> {
    ensure_width(record)?;

    Ok(BulletinRecord {
        generation_date: number_text(record, columns::DT_GERACAO),
        generation_time: number_text(record, columns::HH_GERACAO),
        election_year: number_code(record, columns::ANO_ELEICAO, "ANO_ELEICAO")?,
        election_type_code: number_code(record, columns::CD_TIPO_ELEICAO, "CD_TIPO_ELEICAO")?,
        election_type_name: number_text(record, columns::NM_TIPO_ELEICAO),
        plebiscite_code: number_code(record, columns::CD_PLEITO, "CD_PLEITO")?,
        plebiscite_date: number_text(record, columns::DT_PLEITO),
        round: number_code(record, columns::NR_TURNO, "NR_TURNO")?,
        election_code: number_code(record, columns::CD_ELEICAO, "CD_ELEICAO")?,
        election_description: number_text(record, columns::DS_ELEICAO),
        state_code: number_text(record, columns::SG_UF),
        municipality_code: number_code(record, columns::CD_MUNICIPIO, "CD_MUNICIPIO")?,
        municipality_name: number_text(record, columns::NM_MUNICIPIO),
        zone: number_code(record, columns::NR_ZONA, "NR_ZONA")?,
        section: number_code(record, columns::NR_SECAO, "NR_SECAO")?,
        voting_place: number_code(record, columns::NR_LOCAL_VOTACAO, "NR_LOCAL_VOTACAO")?,
        office_code: number_code(record, columns::CD_CARGO_PERGUNTA, "CD_CARGO_PERGUNTA")?,
        office_name: number_text(record, columns::DS_CARGO_PERGUNTA),
        party_number: number_code(record, columns::NR_PARTIDO, "NR_PARTIDO")?,
        party_acronym: number_text(record, columns::SG_PARTIDO),
        party_name: number_text(record, columns::NM_PARTIDO),
        bulletin_received_at: number_text(record, columns::DT_BU_RECEBIDO),
        eligible_voters: number_quantity(record, columns::QT_APTOS, "QT_APTOS")?,
        turnout: number_quantity(record, columns::QT_COMPARECIMENTO, "QT_COMPARECIMENTO")?,
        abstentions: number_quantity(record, columns::QT_ABSTENCOES, "QT_ABSTENCOES")?,
        ballot_box_type_code: number_code(record, columns::CD_TIPO_URNA, "CD_TIPO_URNA")?,
        ballot_box_type_name: number_text(record, columns::DS_TIPO_URNA),
        votable_type_code: number_code(record, columns::CD_TIPO_VOTAVEL, "CD_TIPO_VOTAVEL")?,
        votable_type_name: number_text(record, columns::DS_TIPO_VOTAVEL),
        votable_number: number_code(record, columns::NR_VOTAVEL, "NR_VOTAVEL")?,
        votable_name: number_text(record, columns::NM_VOTAVEL),
        votes: number_quantity(record, columns::QT_VOTOS, "QT_VOTOS")?,
        ballot_box_number: number_quantity(
            record,
            columns::NR_URNA_EFETIVADA,
            "NR_URNA_EFETIVADA",
        )?,
        ballot_box_load_code_prefix: number_text(record, columns::CD_CARGA_1_URNA_EFETIVADA),
        ballot_box_load_code_suffix: number_text(record, columns::CD_CARGA_2_URNA_EFETIVADA),
        flashcard_code: number_text(record, columns::CD_FLASHCARD_URNA_EFETIVADA),
        ballot_box_load_date: number_text(record, columns::DT_CARGA_URNA_EFETIVADA),
        office_section_label: number_text(record, columns::DS_CARGO_PERGUNTA_SECAO),
        aggregated_sections: numbers::parse_optional_text(field(
            record,
            columns::DS_SECOES_AGREGADAS,
        )),
        opened_at: number_text(record, columns::DT_ABERTURA),
        closed_at: number_text(record, columns::DT_ENCERRAMENTO),
        biometric_voters_not_enabled: number_quantity(
            record,
            columns::QT_ELEI_BIOM_SEM_HABILITACAO,
            "QT_ELEI_BIOM_SEM_HABILITACAO",
        )?,
        bulletin_issued_at: number_text(record, columns::DT_EMISSAO_BU),
        counting_board_number: number_quantity(
            record,
            columns::NR_JUNTA_APURADORA,
            "NR_JUNTA_APURADORA",
        )?,
        counting_team_number: number_quantity(
            record,
            columns::NR_TURMA_APURADORA,
            "NR_TURMA_APURADORA",
        )?,
    })
}

fn ensure_width(record: &StringRecord) -> Result<()> {
    if record.len() >= columns::COLUMN_COUNT {
        return Ok(());
    }

    let index = record.len();
    let column = columns::EXPECTED_HEADERS
        .get(index)
        .copied()
        .unwrap_or("unknown");

    Err(CoreError::MissingColumn {
        column,
        line: record.position().map_or(0, |position| position.line()),
    })
}

fn field(record: &StringRecord, index: usize) -> &str {
    record.get(index).unwrap_or_default()
}

fn number_text(record: &StringRecord, index: usize) -> String {
    numbers::parse_text(field(record, index))
}

fn number_code(record: &StringRecord, index: usize, column: &'static str) -> Result<i64> {
    numbers::parse_code(column, field(record, index))
}

fn number_quantity(
    record: &StringRecord,
    index: usize,
    column: &'static str,
) -> Result<Option<i64>> {
    numbers::parse_quantity(column, field(record, index))
}
