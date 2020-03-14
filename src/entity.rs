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

#[derive(Debug, Clone)]
pub enum Record {
    Header {
        classification: String,
        deposition_date: NaiveDate,
        id_code: String,
    },
    Title {
        title: String,
    },
    Obslte {
        replacement_date: NaiveDate,
        replacement_ids: Vec<String>,
    },
    Split {
        id_codes: Vec<String>,
    },
    Caveat {
        id_code: String,
        comment: String,
    },
    Sprsde {
        sprsde_date: NaiveDate,
        id_code: String,
        superseeded: Vec<String>,
    },
    Seqres {
        chain_id: Option<char>,
        residues: Vec<String>,
    },
    Mdltyp {
        structural_annotation: Vec<String>,
    },
    Revdats {
        revdat: Vec<Revdat>,
    },
    Cmpnd {
        tokens: Vec<Token>,
    },
    Source {
        tokens: Vec<Token>,
    },
    Keywds {
        keywords: Vec<String>,
    },
    JournalAuthor {
        name: String,
    },
    JournalTitle {
        title: String,
    },
    JournalEditor {
        name: String,
    },
    Experimental {
        techniques: Vec<ExperimentalTechnique>,
    },
    Nummdl {
        num: u32,
    },
    Authors {
        authors: Vec<Author>,
    },
}
