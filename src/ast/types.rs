use chrono::NaiveDate;
use std::str::FromStr;

///Holds name of an author utilized by multiple
///parsers such as author and journal author parsers
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Author(pub String);

/// Experimental techniques utilized in obtaining
/// structure data
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    MoleculeId(u32),
    Molecule(String),
    Chain {
        identifiers: Vec<String>,
    },
    Fragment(String),
    Synonym {
        synonyms: Vec<String>,
    },
    Ec {
        commission_numbers: Vec<String>,
    },
    Engineered(bool),
    Mutation(bool),
    OtherDetails(String),
    Synthetic(String),
    OrganismScientific(String),
    OrganismCommon {
        organisms: Vec<String>,
    },
    OrganismTaxId {
        id: Vec<u32>,
    },
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
    Gene {
        gene: Vec<String>,
    },
    ExpressionSystem(String),
    ExpressionSystemCommon {
        systems: Vec<String>,
    },
    ExpressionSystemTaxId {
        id: Vec<u32>,
    },
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
    /// a key this parser does not know, or a value not matching its key's type
    Other {
        key: String,
        value: String,
    },
}

/// Represents a modification made to this pdb entry.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Revdat {
    pub modification_number: u32,
    pub modification_date: NaiveDate,
    pub idcode: String,
    pub modification_type: ModificationType,
    pub modification_detail: Vec<String>,
}

/// modification type of REVDAT record
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum SerialNumber {
    Issn,
    Essn,
}

/// contains HEADER recor information
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
            deposition_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            id_code: String::default(),
        }
    }
}

/// result of a TITLE record
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Title {
    pub title: String,
}

/// contains pdb entry ids which removed
/// this one from PDB
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Obslte {
    pub replacement_date: NaiveDate,
    pub id_code: String,
    pub replacement_ids: Vec<String>,
}

impl std::default::Default for Obslte {
    fn default() -> Self {
        Obslte {
            replacement_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            id_code: String::default(),
            replacement_ids: Vec::new(),
        }
    }
}

/// if this entry is a part of bigger
/// structure, this struct holds ids of other
/// parts of the bigger structure
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Split {
    pub id_codes: Vec<String>,
}

/// fallacies of this entry
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Caveat {
    pub id_code: String,
    pub comment: String,
}

/// pdb entry ids made obsolete by this entry
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone)]
pub struct Sprsde {
    pub sprsde_date: NaiveDate,
    pub id_code: String,
    pub superseeded: Vec<String>,
}

impl std::default::Default for Sprsde {
    fn default() -> Self {
        Sprsde {
            sprsde_date: NaiveDate::from_ymd_opt(1900, 1, 1).unwrap(),
            superseeded: Vec::new(),
            id_code: String::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Seqres {
    pub chain_id: Option<char>,
    pub num_res: u32,
    pub residues: Vec<String>,
}

/// model type of the entry
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Mdltyp {
    pub structural_annotation: Vec<String>,
}

/// collection of revisions
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Revdats {
    pub revdat: Vec<Revdat>,
}

/// collection of tokens in a CMPND record
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Cmpnd {
    pub tokens: Vec<Token>,
}

/// collection of tokens in a SOURCE record
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Source {
    pub tokens: Vec<Token>,
}

/// keywords related to the entry
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Keywds {
    pub keywords: Vec<String>,
}

/// author collection
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Authors {
    pub authors: Vec<Author>,
}

/// journal author collection
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct JournalAuthors {
    pub authors: Vec<Author>,
}

/// journal title
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct JournalTitle {
    pub title: String,
}

/// journal editor collection
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct JournalEditors {
    pub name: Vec<Author>,
}

/// journal reference
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct JournalReference {
    pub publication_name: String,
    pub volume: Option<u32>,
    pub page: Option<u32>,
    pub year: Option<u32>,
}

/// journal Citation fields
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct JournalCitation {
    pub serial_type: Option<SerialNumber>,
    pub serial: Option<String>,
}

/// journal publication fields
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct JournalPublication {
    pub publication: String,
}

/// journal PubMed id
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct JournalPubMedId {
    pub id: u32,
}

/// digital object identifier of related e-pub
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct JournalDoi {
    pub id: String,
}

/// experimanetal techniques used for exploring
/// structure of this entry
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Experimental {
    pub techniques: Vec<ExperimentalTechnique>,
}

/// number of models in this file
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Nummdl {
    pub num: u32,
}

/// cross references to other sequence databases
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Dbref {
    pub idcode: String,
    pub chain_id: char,
    pub seq_begin: i32,
    pub initial_sequence: Option<char>,
    pub seq_end: i32,
    pub ending_sequence: Option<char>,
    pub database: String,
    pub db_accession: String,
    pub db_idcode: String,
    pub db_seq_begin: i32,
    pub idbns_begin: Option<char>,
    pub db_seq_end: i32,
    pub dbins_end: Option<char>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Dbref1 {
    pub idcode: String,
    pub chain_id: char,
    pub seq_begin: i32,
    pub initial_sequence: Option<char>,
    pub seq_end: i32,
    pub ending_sequence: Option<char>,
    pub database: String,
    pub db_idcode: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Dbref2 {
    pub idcode: String,
    pub chain_id: char,
    pub db_accession: String,
    pub db_seq_begin: i32,
    pub db_seq_end: i32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Seqadv {
    pub idcode: String,
    pub conflicting_residue: String,
    pub chain_id: char,
    /// residue number in the structure, `None` for deleted residues
    pub sequence_number: Option<i32>,
    pub insertion_code: Option<char>,
    pub database: String,
    pub db_accession: String,
    pub sequence_db_residue: Option<String>,
    pub sequence_db_sequence_number: Option<u32>,
    pub conflict: String,
}

/// residue modification record
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default)]
pub struct Modres {
    pub idcode: String,
    pub residue_name: String,
    pub chain_id: char,
    pub sequence_number: i32,
    pub insertion_code: Option<char>,
    pub standart_residue_name: String,
    pub comment: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// an atom of an ATOM or HETATM record
pub struct Atom {
    pub serial: u32,
    pub name: String,
    pub alt_loc: Option<char>,
    pub residue_name: String,
    pub chain_id: char,
    pub residue_seq: i32,
    pub insertion_code: Option<char>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub occupancy: f64,
    pub temp_factor: f64,
    pub element: Option<String>,
    pub charge: Option<i8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// anisotropic temperature factors of an atom, scaled by 10^4
pub struct Anisou {
    pub serial: u32,
    pub name: String,
    pub alt_loc: Option<char>,
    pub residue_name: String,
    pub chain_id: char,
    pub residue_seq: i32,
    pub insertion_code: Option<char>,
    pub u11: i32,
    pub u22: i32,
    pub u33: i32,
    pub u12: i32,
    pub u13: i32,
    pub u23: i32,
    pub element: Option<String>,
    pub charge: Option<i8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// end of a chain. Old entries may leave all fields blank
pub struct Ter {
    pub serial: Option<u32>,
    pub residue_name: String,
    pub chain_id: char,
    pub residue_seq: Option<i32>,
    pub insertion_code: Option<char>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// start of a model in a multi model entry
pub struct Model {
    pub serial: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// record counts of the file, used as a checksum
pub struct Master {
    pub num_remark: u32,
    pub num_het: u32,
    pub num_helix: u32,
    pub num_sheet: u32,
    pub num_site: u32,
    pub num_xform: u32,
    /// number of ATOM and HETATM records
    pub num_coord: u32,
    pub num_ter: u32,
    pub num_conect: u32,
    pub num_seq: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// identifies a residue by its name and position in a chain
pub struct ResidueRef {
    pub residue_name: String,
    pub chain_id: char,
    pub residue_seq: i32,
    pub insertion_code: Option<char>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// bonds of an atom listed in a CONECT record
pub struct Conect {
    pub serial: u32,
    pub bonded: Vec<u32>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// disulfide bond between two cysteines
pub struct Ssbond {
    pub serial: u32,
    pub residue1: ResidueRef,
    pub residue2: ResidueRef,
    pub symmetry1: String,
    pub symmetry2: String,
    pub length: Option<f64>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// connectivity between two atoms not implied by standard residues
pub struct Link {
    pub name1: String,
    pub alt_loc1: Option<char>,
    pub residue1: ResidueRef,
    pub name2: String,
    pub alt_loc2: Option<char>,
    pub residue2: ResidueRef,
    pub symmetry1: String,
    pub symmetry2: String,
    pub length: Option<f64>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// cis peptide bond between two residues
pub struct Cispep {
    pub serial: u32,
    pub residue1: ResidueRef,
    pub residue2: ResidueRef,
    /// model number, 0 for single model entries
    pub model: u32,
    /// omega angle in degrees
    pub angle: Option<f64>,
}

/// main enum unifying all record parser results.
/// all sub parsers return a variant of this
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
    Dbref1(Dbref1),
    Dbref2(Dbref2),
    Seqadv(Seqadv),
    Modres(Modres),
    Remark,
    Model(Model),
    Atom(Atom),
    Anisou(Anisou),
    Ter(Ter),
    Hetatm(Atom),
    Endmdl,
    Ssbond(Ssbond),
    Link(Link),
    Cispep(Cispep),
    Conect(Conect),
    Master(Master),
    End,
    /// a line no record parser recognized, kept verbatim
    Unknown(String),
}
