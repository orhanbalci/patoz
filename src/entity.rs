use chrono::NaiveDate;

#[derive(Debug)]
pub struct Header {
    pub classification: String,
    pub deposition_date: NaiveDate,
    pub id_code: String,
}

#[derive(Debug)]
pub struct Obslte {
    pub continuation: u32,
    pub replacement_date: NaiveDate,
    pub replacement_ids: Vec<String>,
}

#[derive(Debug)]
pub struct Title {
    pub continuation: u32,
    pub title: String,
}

#[derive(Debug)]
pub struct Split {
    pub continuation: u32,
    pub id_codes: Vec<String>,
}

#[derive(Debug)]
pub struct Caveat {
    pub continuation: u32,
    pub comment: String,
}

#[derive(Debug)]
pub struct CmpndLine {
    pub continuation: u32,
    pub remaining: String,
}

#[derive(Debug)]
pub struct SourceLine {
    pub continuation: u32,
    pub remaining: String,
}

#[derive(Debug)]
pub enum Token {
    MoleculeId(u32),
    Molecule(String),
    Chain { identifiers: Vec<String> },
    Fragment(String),
    Synonym { synonyms: Vec<String> },
    Ec { commission_numbers: Vec<String> },
    Engineered(bool),
    Mutation(bool),
    OtherDetails(String),
    Synthetic(String),
    OrganismScientific(String),
    OrganismCommon { organisms: Vec<String> },
    OrganismTaxId { id: Vec<u32> },
    Strain(String),
    Variant(String),
    CellLine(String),
    Atcc(u32),
    Organ(String),
    Tissue(String),
    Cell(String),
    Organelle(String),
    Secretion(String),
    CellularLocation(String),
    Plasmid(String),
    Gene { gene: Vec<String> },
    ExpressionSystem(String),
    ExpressionSystemCommon { systems: Vec<String> },
    ExpressionSystemTaxId { id: Vec<u32> },
    ExpressionSystemStrain(String),
    ExpressionSystemVariant(String),
    ExpressionSystemCellLine(String),
    ExpressionSystemAtcc(u32),
    ExpressionSystemOrgan(String),
    ExpressionSystemTissue(String),
    ExpressionSystemCell(String),
    ExpressionSystemOrgenelle(String),
    ExpressionSystemCelularLocation(String),
    ExpressionSystemVectorType(String),
    ExpressionSystemVector(String),
    ExpressionSystemPlasmid(String),
    ExpressionSystemGene(String),
}
