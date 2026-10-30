//! Column layout of the `BOLETIM_URNA` file.

/// Number of columns in the expected layout.
pub const COLUMN_COUNT: usize = 45;

pub const DT_GERACAO: usize = 0;
pub const HH_GERACAO: usize = 1;
pub const ANO_ELEICAO: usize = 2;
pub const CD_TIPO_ELEICAO: usize = 3;
pub const NM_TIPO_ELEICAO: usize = 4;
pub const CD_PLEITO: usize = 5;
pub const DT_PLEITO: usize = 6;
pub const NR_TURNO: usize = 7;
pub const CD_ELEICAO: usize = 8;
pub const DS_ELEICAO: usize = 9;
pub const SG_UF: usize = 10;
pub const CD_MUNICIPIO: usize = 11;
pub const NM_MUNICIPIO: usize = 12;
pub const NR_ZONA: usize = 13;
pub const NR_SECAO: usize = 14;
pub const NR_LOCAL_VOTACAO: usize = 15;
pub const CD_CARGO_PERGUNTA: usize = 16;
pub const DS_CARGO_PERGUNTA: usize = 17;
pub const NR_PARTIDO: usize = 18;
pub const SG_PARTIDO: usize = 19;
pub const NM_PARTIDO: usize = 20;
pub const DT_BU_RECEBIDO: usize = 21;
pub const QT_APTOS: usize = 22;
pub const QT_COMPARECIMENTO: usize = 23;
pub const QT_ABSTENCOES: usize = 24;
pub const CD_TIPO_URNA: usize = 25;
pub const DS_TIPO_URNA: usize = 26;
pub const CD_TIPO_VOTAVEL: usize = 27;
pub const DS_TIPO_VOTAVEL: usize = 28;
pub const NR_VOTAVEL: usize = 29;
pub const NM_VOTAVEL: usize = 30;
pub const QT_VOTOS: usize = 31;
pub const NR_URNA_EFETIVADA: usize = 32;
pub const CD_CARGA_1_URNA_EFETIVADA: usize = 33;
pub const CD_CARGA_2_URNA_EFETIVADA: usize = 34;
pub const CD_FLASHCARD_URNA_EFETIVADA: usize = 35;
pub const DT_CARGA_URNA_EFETIVADA: usize = 36;
pub const DS_CARGO_PERGUNTA_SECAO: usize = 37;
pub const DS_SECOES_AGREGADAS: usize = 38;
pub const DT_ABERTURA: usize = 39;
pub const DT_ENCERRAMENTO: usize = 40;
pub const QT_ELEI_BIOM_SEM_HABILITACAO: usize = 41;
pub const DT_EMISSAO_BU: usize = 42;
pub const NR_JUNTA_APURADORA: usize = 43;
pub const NR_TURMA_APURADORA: usize = 44;

/// Header names in the exact order required by SABU.
pub const EXPECTED_HEADERS: [&str; COLUMN_COUNT] = [
    "DT_GERACAO",
    "HH_GERACAO",
    "ANO_ELEICAO",
    "CD_TIPO_ELEICAO",
    "NM_TIPO_ELEICAO",
    "CD_PLEITO",
    "DT_PLEITO",
    "NR_TURNO",
    "CD_ELEICAO",
    "DS_ELEICAO",
    "SG_UF",
    "CD_MUNICIPIO",
    "NM_MUNICIPIO",
    "NR_ZONA",
    "NR_SECAO",
    "NR_LOCAL_VOTACAO",
    "CD_CARGO_PERGUNTA",
    "DS_CARGO_PERGUNTA",
    "NR_PARTIDO",
    "SG_PARTIDO",
    "NM_PARTIDO",
    "DT_BU_RECEBIDO",
    "QT_APTOS",
    "QT_COMPARECIMENTO",
    "QT_ABSTENCOES",
    "CD_TIPO_URNA",
    "DS_TIPO_URNA",
    "CD_TIPO_VOTAVEL",
    "DS_TIPO_VOTAVEL",
    "NR_VOTAVEL",
    "NM_VOTAVEL",
    "QT_VOTOS",
    "NR_URNA_EFETIVADA",
    "CD_CARGA_1_URNA_EFETIVADA",
    "CD_CARGA_2_URNA_EFETIVADA",
    "CD_FLASHCARD_URNA_EFETIVADA",
    "DT_CARGA_URNA_EFETIVADA",
    "DS_CARGO_PERGUNTA_SECAO",
    "DS_SECOES_AGREGADAS",
    "DT_ABERTURA",
    "DT_ENCERRAMENTO",
    "QT_ELEI_BIOM_SEM_HABILITACAO",
    "DT_EMISSAO_BU",
    "NR_JUNTA_APURADORA",
    "NR_TURMA_APURADORA",
];
