//! The ballot box bulletin record.

/// A single row of the `BOLETIM_URNA` CSV file.
///
/// Only the fields used by the current features are typed. Additional columns
/// from the source file are ignored while parsing.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BulletinRecord {
    /// Election year (`ANO_ELEICAO`).
    pub ano_eleicao: Option<u32>,
    /// Election round (`NR_TURNO`).
    pub nr_turno: Option<u16>,
    /// Federation unit code (`SG_UF`).
    pub uf: String,
    /// Municipality name (`NM_MUNICIPIO`).
    pub municipio: String,
    /// Electoral zone number (`NR_ZONA`).
    pub nr_zona: Option<u32>,
    /// Electoral section number (`NR_SECAO`).
    pub nr_secao: Option<u32>,
    /// Office or question description (`DS_CARGO_PERGUNTA`).
    pub cargo: String,
    /// Party acronym (`SG_PARTIDO`).
    pub partido: String,
    /// Votable type description (`DS_TIPO_VOTAVEL`).
    pub tipo_votavel: String,
    /// Votable number (`NR_VOTAVEL`).
    pub nr_votavel: Option<u64>,
    /// Votable name (`NM_VOTAVEL`).
    pub nm_votavel: String,
    /// Number of votes received (`QT_VOTOS`).
    pub qt_votos: Option<u64>,
}
