use chrono::NaiveDate;
use std::marker::PhantomData;

#[derive(Debug)]
pub(crate) struct Continuation<T> {
    pub continuation: u32,
    pub remaining: String,
    pub phantom: PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct Author(pub String);

#[derive(Debug, Clone)]
pub enum ExperimentalTechnique {
    XRayDiffraction,
    FiberDiffraction,
    NeutronDiffraction,
    ElectronCrystallography,
    ElectronMicroscopy,
    SolidStateNmr,
    SolutionNmr,
    SolutionScattering,
}

#[derive(Debug, PartialEq, Clone)]
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
    ExpressionSystemOrganelle(String),
    ExpressionSystemCellularLocation(String),
    ExpressionSystemVectorType(String),
    ExpressionSystemVector(String),
    ExpressionSystemPlasmid(String),
    ExpressionSystemGene(String),
}

#[derive(Debug, Clone)]
pub struct Revdat {
    pub modification_number: u32,
    pub modification_date: NaiveDate,
    pub idcode: String,
    pub modification_type: ModificationType,
    pub modification_detail: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ModificationType {
    InitialRelease,
    OtherModification,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SerialNumber {
    Issn,
    Essn,
}

#[derive(Debug, Clone)]
pub struct Header {
    pub classification: String,
    pub deposition_date: NaiveDate,
    pub id_code: String,
}

impl std::default::Default for Header {
    fn default() -> Self {
        Header {
            classification: String::default(),
            deposition_date: NaiveDate::from_ymd(1900, 1, 1),
            id_code: String::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Title {
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct Obslte {
    pub replacement_date: NaiveDate,
    pub replacement_ids: Vec<String>,
}

impl std::default::Default for Obslte {
    fn default() -> Self {
        Obslte {
            replacement_date: NaiveDate::from_ymd(1900, 1, 1),
            replacement_ids: Vec::new(),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct Split {
    pub id_codes: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Caveat {
    pub id_code: String,
    pub comment: String,
}

#[derive(Debug, Clone)]
pub struct Sprsde {
    pub sprsde_date: NaiveDate,
    pub id_code: String,
    pub superseeded: Vec<String>,
}

impl std::default::Default for Sprsde {
    fn default() -> Self {
        Sprsde {
            sprsde_date: NaiveDate::from_ymd(1900, 1, 1),
            superseeded: Vec::new(),
            id_code: String::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Seqres {
    pub chain_id: Option<char>,
    pub residues: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Mdltyp {
    pub structural_annotation: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Revdats {
    pub revdat: Vec<Revdat>,
}

#[derive(Debug, Clone, Default)]
pub struct Cmpnd {
    pub tokens: Vec<Token>,
}

#[derive(Debug, Clone, Default)]
pub struct Source {
    pub tokens: Vec<Token>,
}
#[derive(Debug, Clone, Default)]
pub struct Keywds {
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Authors {
    pub authors: Vec<Author>,
}
#[derive(Debug, Clone)]
pub enum Record {
    Header(Header),
    Title(Title),
    Obslte(Obslte),
    Split(Split),
    Caveat(Caveat),
    Sprsde(Sprsde),
    Seqres(Seqres),
    Mdltyp(Mdltyp),
    Revdats(Revdats),
    Cmpnd(Cmpnd),
    Source(Source),
    Keywds(Keywds),
    JournalAuthors {
        authors: Vec<Author>,
    },
    JournalTitle {
        title: String,
    },
    JournalEditors {
        name: Vec<Author>,
    },
    JournalReference {
        publication_name: String,
        volume: Option<u32>,
        page: Option<u32>,
        year: Option<u32>,
    },
    JournalPublication {
        publication: String,
    },
    JournalCitation {
        serial_type: Option<SerialNumber>,
        serial: Option<String>,
    },
    JournalPubMedId {
        id: u32,
    },
    JournalDoi {
        id: String,
    },
    Experimental {
        techniques: Vec<ExperimentalTechnique>,
    },
    Nummdl {
        num: u32,
    },
    Authors(Authors),
}
