use chrono::NaiveDate;
use std::{marker::PhantomData, str::FromStr};

#[derive(Debug)]
pub(crate) struct Continuation<T> {
    pub continuation: u32,
    pub remaining: String,
    pub phantom: PhantomData<T>,
}

///Holds name of an author utilized by multiple
///parsers such as author and journal author parsers
#[derive(Debug, Clone, PartialEq)]
pub struct Author(pub String);

/// Experimental techniques utilized in obtaining
/// structure data
#[derive(Debug, Clone, PartialEq)]
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

impl FromStr for ExperimentalTechnique {
    type Err = String;
    fn from_str(inp: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match inp {
            "X-RAY DIFFRACTION" => Ok(ExperimentalTechnique::XRayDiffraction),
            "FIBER DIFFRACTION" => Ok(ExperimentalTechnique::FiberDiffraction),
            "NEUTRON DIFFRACTION" => Ok(ExperimentalTechnique::NeutronDiffraction),
            "ELECTRON CRYSTALLOGRAPHY" => Ok(ExperimentalTechnique::ElectronCrystallography),
            "ELECTRON MICROSCOPY" => Ok(ExperimentalTechnique::ElectronMicroscopy),
            "SOLID-STATE NMR" => Ok(ExperimentalTechnique::SolidStateNmr),
            "SOLUTION NMR" => Ok(ExperimentalTechnique::SolutionNmr),
            "SOLUTION SCATTERING" => Ok(ExperimentalTechnique::SolutionScattering),
            _ => Err(format!("Unknown experimental result {}", inp)),
        }
    }
}

/// Represents keys of CMPND and SOURCE records
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

/// Represents a modification made to this pdb entry.
#[derive(Debug, Clone)]
pub struct Revdat {
    pub modification_number: u32,
    pub modification_date: NaiveDate,
    pub idcode: String,
    pub modification_type: ModificationType,
    pub modification_detail: Vec<String>,
}

/// modification type of REVDAT record
#[derive(Debug, Clone)]
pub enum ModificationType {
    /// initial release of the entry. Indicated as 0
    /// in a REVDAT record
    InitialRelease,
    /// modifications other than initial release
    /// Indicated with 1 in a REVDAT record.
    OtherModification,
    /// modification type other than 0 or 1
    UnknownModification,
}

/// Serial Number Type of a JRNL REFN record
#[derive(Debug, Clone, PartialEq)]
pub enum SerialNumber {
    Issn,
    Essn,
}

/// contains HEADER recor information
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

/// result of a TITLE record
#[derive(Debug, Clone, Default)]
pub struct Title {
    pub title: String,
}

/// contains pdb entry ids which removed
/// this one from PDB
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

/// if this entry is a part of bigger
/// structure, this struct holds ids of other
/// parts of the bigger structure
#[derive(Debug, Clone, Default)]
pub struct Split {
    pub id_codes: Vec<String>,
}

/// fallacies of this entry
#[derive(Debug, Clone, Default)]
pub struct Caveat {
    pub id_code: String,
    pub comment: String,
}

/// pdb entry ids made obsolete by this entry
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

/// model type of the entry
#[derive(Debug, Clone, Default)]
pub struct Mdltyp {
    pub structural_annotation: Vec<String>,
}

/// collection of revisions
#[derive(Debug, Clone, Default)]
pub struct Revdats {
    pub revdat: Vec<Revdat>,
}

/// collection of tokens in a CMPND record
#[derive(Debug, Clone, Default)]
pub struct Cmpnd {
    pub tokens: Vec<Token>,
}

/// collection of tokens in a SOURCE record
#[derive(Debug, Clone, Default)]
pub struct Source {
    pub tokens: Vec<Token>,
}

/// keywords related to the entry
#[derive(Debug, Clone, Default)]
pub struct Keywds {
    pub keywords: Vec<String>,
}

/// author collection
#[derive(Debug, Clone, Default)]
pub struct Authors {
    pub authors: Vec<Author>,
}

/// journal author collection
#[derive(Debug, Clone, Default)]
pub struct JournalAuthors {
    pub authors: Vec<Author>,
}

/// journal title
#[derive(Debug, Clone, Default)]
pub struct JournalTitle {
    pub title: String,
}

/// journal editor collection
#[derive(Debug, Clone, Default)]
pub struct JournalEditors {
    pub name: Vec<Author>,
}

/// journal reference
#[derive(Debug, Clone, Default)]
pub struct JournalReference {
    pub publication_name: String,
    pub volume: Option<u32>,
    pub page: Option<u32>,
    pub year: Option<u32>,
}

/// journal Citation fields
#[derive(Debug, Clone, Default)]
pub struct JournalCitation {
    pub serial_type: Option<SerialNumber>,
    pub serial: Option<String>,
}

/// journal publication fields
#[derive(Debug, Clone, Default)]
pub struct JournalPublication {
    pub publication: String,
}

/// journal PubMed id
#[derive(Debug, Clone, Default)]
pub struct JournalPubMedId {
    pub id: u32,
}

/// digital object identifier of related e-pub
#[derive(Debug, Clone, Default)]
pub struct JournalDoi {
    pub id: String,
}

/// experimanetal techniques used for exploring
/// structure of this entry
#[derive(Debug, Clone, Default)]
pub struct Experimental {
    pub techniques: Vec<ExperimentalTechnique>,
}

/// number of models in this file
#[derive(Debug, Clone, Default)]
pub struct Nummdl {
    pub num: u32,
}

/// cross references to other sequence databases
#[derive(Debug, Clone, Default)]
pub struct Dbref {
    pub idcode: String,
    pub chain_id: char,
    pub seq_begin: u32,
    pub initial_sequence: Option<char>,
    pub seq_end: u32,
    pub ending_sequence: Option<char>,
    pub database: String,
    pub db_accession: String,
    pub db_idcode: String,
    pub db_seq_begin: u32,
    pub idbns_begin: Option<char>,
    pub db_seq_end: u32,
    pub dbins_end: Option<char>,
}

/// main enum unifying all record parser results.
/// all sub parsers return a cariant of this
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
    JournalAuthors(JournalAuthors),
    JournalTitle(JournalTitle),
    JournalEditors(JournalEditors),
    JournalReference(JournalReference),
    JournalCitation(JournalCitation),
    JournalPublication(JournalPublication),
    JournalPubMedId(JournalPubMedId),
    JournalDoi(JournalDoi),
    Experimental(Experimental),
    Nummdl(Nummdl),
    Authors(Authors),
    Dbref(Dbref),
}
