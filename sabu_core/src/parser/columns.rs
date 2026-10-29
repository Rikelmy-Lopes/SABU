//! Header mapping and typed record construction.

use csv::ByteRecord;

use crate::error::CoreError;
use crate::model::BulletinRecord;

use super::decode::decode_text;
use super::numbers::{parse_optional_number, parse_optional_u16, parse_optional_u32};

/// Positions of the columns used by the parser.
pub(super) struct Columns {
    ano_eleicao: usize,
    nr_turno: usize,
    sg_uf: usize,
    nm_municipio: usize,
    nr_zona: usize,
    nr_secao: usize,
    ds_cargo: usize,
    sg_partido: usize,
    ds_tipo_votavel: usize,
    nr_votavel: usize,
    nm_votavel: usize,
    qt_votos: usize,
}

impl Columns {
    /// Maps mandatory columns by name.
    pub(super) fn from_headers(headers: &ByteRecord) -> Result<Self, CoreError> {
        let mut missing = Vec::new();
        let mut index_of = |name: &'static str| -> usize {
            match header_index(headers, name) {
                Some(index) => index,
                None => {
                    missing.push(name);
                    0
                }
            }
        };

        let columns = Self {
            ano_eleicao: index_of("ANO_ELEICAO"),
            nr_turno: index_of("NR_TURNO"),
            sg_uf: index_of("SG_UF"),
            nm_municipio: index_of("NM_MUNICIPIO"),
            nr_zona: index_of("NR_ZONA"),
            nr_secao: index_of("NR_SECAO"),
            ds_cargo: index_of("DS_CARGO_PERGUNTA"),
            sg_partido: index_of("SG_PARTIDO"),
            ds_tipo_votavel: index_of("DS_TIPO_VOTAVEL"),
            nr_votavel: index_of("NR_VOTAVEL"),
            nm_votavel: index_of("NM_VOTAVEL"),
            qt_votos: index_of("QT_VOTOS"),
        };

        if missing.is_empty() {
            Ok(columns)
        } else {
            Err(CoreError::MissingColumns {
                columns: missing.join(", "),
            })
        }
    }

    /// Builds a typed record from a CSV row.
    pub(super) fn parse_record(
        &self,
        record: &ByteRecord,
        line: u64,
    ) -> Result<BulletinRecord, CoreError> {
        let expected = self.required_width();
        if record.len() < expected {
            return Err(CoreError::InvalidRecordWidth {
                line,
                found: record.len(),
                expected,
            });
        }

        Ok(BulletinRecord {
            ano_eleicao: parse_optional_u32(field(record, self.ano_eleicao), "ANO_ELEICAO", line)?,
            nr_turno: parse_optional_u16(field(record, self.nr_turno), "NR_TURNO", line)?,
            uf: decode_text(field(record, self.sg_uf)),
            municipio: decode_text(field(record, self.nm_municipio)),
            nr_zona: parse_optional_u32(field(record, self.nr_zona), "NR_ZONA", line)?,
            nr_secao: parse_optional_u32(field(record, self.nr_secao), "NR_SECAO", line)?,
            cargo: decode_text(field(record, self.ds_cargo)),
            partido: decode_text(field(record, self.sg_partido)),
            tipo_votavel: decode_text(field(record, self.ds_tipo_votavel)),
            nr_votavel: parse_optional_number(field(record, self.nr_votavel), "NR_VOTAVEL", line)?,
            nm_votavel: decode_text(field(record, self.nm_votavel)),
            qt_votos: parse_optional_number(field(record, self.qt_votos), "QT_VOTOS", line)?,
        })
    }

    /// Index one past the highest mandatory column.
    fn required_width(&self) -> usize {
        [
            self.ano_eleicao,
            self.nr_turno,
            self.sg_uf,
            self.nm_municipio,
            self.nr_zona,
            self.nr_secao,
            self.ds_cargo,
            self.sg_partido,
            self.ds_tipo_votavel,
            self.nr_votavel,
            self.nm_votavel,
            self.qt_votos,
        ]
        .into_iter()
        .max()
        .map_or(0, |index| index + 1)
    }
}

/// Returns true when the header contains the given column name.
fn header_index(headers: &ByteRecord, name: &'static str) -> Option<usize> {
    headers.iter().position(|header| {
        let header = header.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(header);
        header.trim_ascii() == name.as_bytes()
    })
}

/// Returns a field or an empty slice when the record is too short.
fn field(record: &ByteRecord, index: usize) -> &[u8] {
    record.get(index).unwrap_or(&[])
}
