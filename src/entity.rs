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
}
