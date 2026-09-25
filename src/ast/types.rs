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

impl ExperimentalTechnique {
    /// name of the technique as written in EXPDTA records
    pub fn name(&self) -> &'static str {
        match self {
            ExperimentalTechnique::XRayDiffraction => "X-RAY DIFFRACTION",
            ExperimentalTechnique::FiberDiffraction => "FIBER DIFFRACTION",
            ExperimentalTechnique::NeutronDiffraction => "NEUTRON DIFFRACTION",
            ExperimentalTechnique::ElectronCrystallography => "ELECTRON CRYSTALLOGRAPHY",
            ExperimentalTechnique::ElectronMicroscopy => "ELECTRON MICROSCOPY",
            ExperimentalTechnique::SolidStateNmr => "SOLID-STATE NMR",
            ExperimentalTechnique::SolutionNmr => "SOLUTION NMR",
            ExperimentalTechnique::SolutionScattering => "SOLUTION SCATTERING",
        }
    }
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
#[derive(Debug, Clone, PartialEq)]
pub struct Revdat {
    pub modification_number: u32,
    pub modification_date: NaiveDate,
    pub idcode: String,
    pub modification_type: ModificationType,
    pub modification_detail: Vec<String>,
}

/// modification type of REVDAT record
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ModificationType {
    /// initial release of the entry. Indicated as 0
    /// in a REVDAT record
    InitialRelease,
    /// modifications other than initial release
    /// Indicated with 1 in a REVDAT record.
    OtherModification,
    /// modification type other than 0 or 1
    UnknownModification(u32),
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
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Title {
    pub title: String,
}

/// contains pdb entry ids which removed
/// this one from PDB
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Split {
    pub id_codes: Vec<String>,
}

/// fallacies of this entry
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Caveat {
    pub id_code: String,
    pub comment: String,
}

/// pdb entry ids made obsolete by this entry
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Seqres {
    pub chain_id: Option<char>,
    pub num_res: u32,
    pub residues: Vec<String>,
}

/// model type of the entry
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mdltyp {
    pub structural_annotation: Vec<String>,
}

/// collection of revisions
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Revdats {
    pub revdat: Vec<Revdat>,
}

/// collection of tokens in a CMPND record
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cmpnd {
    pub tokens: Vec<Token>,
}

/// collection of tokens in a SOURCE record
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Source {
    pub tokens: Vec<Token>,
}

/// keywords related to the entry
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Keywds {
    pub keywords: Vec<String>,
}

/// author collection
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Authors {
    pub authors: Vec<Author>,
}

/// journal author collection
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JournalAuthors {
    pub authors: Vec<Author>,
}

/// journal title
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JournalTitle {
    pub title: String,
}

/// journal editor collection
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JournalEditors {
    pub name: Vec<Author>,
}

/// journal reference
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JournalReference {
    pub publication_name: String,
    pub volume: Option<u32>,
    pub page: Option<u32>,
    pub year: Option<u32>,
}

/// journal Citation fields
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JournalCitation {
    pub serial_type: Option<SerialNumber>,
    pub serial: Option<String>,
}

/// journal publication fields
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JournalPublication {
    pub publication: String,
}

/// journal PubMed id
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JournalPubMedId {
    pub id: u32,
}

/// digital object identifier of related e-pub
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct JournalDoi {
    pub id: String,
}

/// experimanetal techniques used for exploring
/// structure of this entry
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Experimental {
    pub techniques: Vec<ExperimentalTechnique>,
}

/// number of models in this file
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Nummdl {
    pub num: u32,
}

/// cross references to other sequence databases
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Dbref2 {
    pub idcode: String,
    pub chain_id: char,
    pub db_accession: String,
    pub db_seq_begin: i32,
    pub db_seq_end: i32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
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
#[derive(Debug, Clone, Default, PartialEq)]
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

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// helix between two residues
pub struct Helix {
    pub serial: u32,
    pub helix_id: String,
    pub start: ResidueRef,
    pub end: ResidueRef,
    /// helix class 1-10 as defined by the specification
    pub class: Option<u32>,
    pub comment: String,
    pub length: Option<u32>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// strand of a beta sheet
pub struct Sheet {
    pub strand: u32,
    pub sheet_id: String,
    pub num_strands: u32,
    pub start: ResidueRef,
    pub end: ResidueRef,
    /// sense relative to the previous strand: 0 first strand, 1 parallel,
    /// -1 anti-parallel
    pub sense: i32,
    /// hydrogen bond registration with the previous strand, absent for the
    /// first strand
    pub registration: Option<SheetRegistration>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// atoms of a strand and the previous strand forming a hydrogen bond
pub struct SheetRegistration {
    pub current_atom: String,
    pub current: ResidueRef,
    pub previous_atom: String,
    pub previous: ResidueRef,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// residues forming a site, e.g. a ligand binding site
pub struct Site {
    pub site_id: String,
    pub num_res: u32,
    pub residues: Vec<ResidueRef>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// a non-standard residue (heterogen) in the entry
pub struct Het {
    /// residue name is the het id
    pub residue: ResidueRef,
    pub num_het_atoms: u32,
    pub text: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// chemical name of a heterogen
pub struct Hetnam {
    pub het_id: String,
    pub name: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// synonyms of a heterogen's chemical name
pub struct Hetsyn {
    pub het_id: String,
    pub synonyms: Vec<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// chemical formula of a heterogen
pub struct Formul {
    pub component: u32,
    pub het_id: String,
    /// marked with `*` in the file, used for water
    pub water: bool,
    pub formula: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// unit cell parameters, space group and Z value
pub struct Cryst1 {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub alpha: f64,
    pub beta: f64,
    pub gamma: f64,
    pub space_group: String,
    pub z: Option<u32>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// transformation `x' = matrix * x + vector` given by three ORIGXn,
/// SCALEn or MTRIXn lines
pub struct Transformation {
    pub matrix: [[f64; 3]; 3],
    pub vector: [f64; 3],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// non-crystallographic symmetry operation
pub struct Mtrix {
    pub serial: u32,
    pub transformation: Transformation,
    /// true if coordinates for the copy are already in the entry
    pub given: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// consecutive REMARK lines with the same remark number
pub struct Remark {
    pub number: u32,
    /// text of each line after the remark number, columns 12-80
    pub lines: Vec<String>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// a residue listed in REMARK 465 as not located in the experiment
pub struct MissingResidue {
    /// model number for multi model entries
    pub model: Option<u32>,
    pub residue: ResidueRef,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// a biological assembly from REMARK 350
pub struct BiologicalAssembly {
    pub id: u32,
    pub author_determined_unit: Option<String>,
    pub software_determined_unit: Option<String>,
    pub parts: Vec<AssemblyPart>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
/// chains of an assembly and the operations generating their copies
pub struct AssemblyPart {
    pub chains: Vec<String>,
    pub operations: Vec<Transformation>,
}

/// main enum unifying all record parser results.
/// all sub parsers return a variant of this
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
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
    Remark(Remark),
    Cryst1(Cryst1),
    /// ORIGX1-3, transformation from orthogonal to submitted coordinates
    Origx(Transformation),
    /// SCALE1-3, transformation from orthogonal to fractional coordinates
    Scale(Transformation),
    /// MTRIX1-3
    Mtrix(Mtrix),
    Model(Model),
    Atom(Atom),
    Anisou(Anisou),
    /// SIGATM, removed from the v3.3 specification but found in older
    /// entries. Coordinate, occupancy and temperature factor fields hold
    /// their standard deviations
    Sigatm(Atom),
    /// SIGUIJ, removed from the v3.3 specification but found in older
    /// entries. Temperature factor fields hold their standard deviations
    Siguij(Anisou),
    Ter(Ter),
    Hetatm(Atom),
    Endmdl,
    Het(Het),
    Hetnam(Hetnam),
    Hetsyn(Hetsyn),
    Formul(Formul),
    Helix(Helix),
    Sheet(Sheet),
    Ssbond(Ssbond),
    Link(Link),
    Cispep(Cispep),
    Site(Site),
    Conect(Conect),
    Master(Master),
    End,
    /// a line no record parser recognized, kept verbatim
    Unknown(String),
}
