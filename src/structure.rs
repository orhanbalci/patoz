/*!
A macromolecular structure independent of the file format it was read
from: entry metadata, entities, and models made of chains, residues and
atoms.

```
let content = std::fs::read_to_string("res/1BYI.pdb").unwrap();
let structure = patoz::parse(&content).structure();
let model = &structure.models[0];
let atoms: usize = model.chains.iter().flat_map(|c| &c.residues).map(|r| r.atoms.len()).sum();
assert_eq!(atoms, 2285);
```
*/
use crate::{
    ast::{pdb_file::*, types::*},
    remark::{assemblies_remark, resolution_remark},
};
use chrono::NaiveDate;
use std::collections::HashMap;

/// A macromolecular structure.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Structure {
    pub id: Option<String>,
    pub title: Option<String>,
    pub classification: Option<String>,
    pub deposition_date: Option<NaiveDate>,
    pub experimental_methods: Vec<ExperimentalTechnique>,
    /// resolution in angstroms, `None` if unknown or not applicable
    pub resolution: Option<f64>,
    pub cell: Option<Cryst1>,
    pub entities: Vec<Entity>,
    pub models: Vec<Model>,
    pub disulfides: Vec<Ssbond>,
    pub links: Vec<Link>,
    pub assemblies: Vec<BiologicalAssembly>,
}

/// A distinct molecule of the structure, which may occur in several chains.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub id: u32,
    pub description: Option<String>,
    /// chains the entity occurs in
    pub chains: Vec<String>,
    pub kind: EntityKind,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum EntityKind {
    /// a polymer with the residue names of its full sequence
    Polymer {
        sequence: Vec<String>,
    },
    /// a ligand or ion, identified by its chemical component id
    NonPolymer {
        component: String,
    },
    Water,
}

/// One model of the structure. X-ray structures have a single model,
/// NMR structures usually several.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Model {
    pub serial: u32,
    /// chain segments in file order, see [Chain]
    pub chains: Vec<Chain>,
}

/// A consecutive run of residues sharing a chain id. A polymer chain ends
/// with a TER record; ligands and water of the same chain id usually follow
/// all polymer chains as separate segments, so a chain id may occur in
/// several segments.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Chain {
    pub id: String,
    /// whether the segment is terminated by a TER record, as polymers are
    pub terminated: bool,
    pub residues: Vec<Residue>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Residue {
    pub name: String,
    pub seq: i32,
    pub insertion_code: Option<char>,
    /// read from HETATM records
    pub hetero: bool,
    pub atoms: Vec<Atom>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Atom {
    pub serial: u32,
    pub name: String,
    pub alt_loc: Option<char>,
    pub element: Option<String>,
    pub charge: Option<i8>,
    pub position: [f64; 3],
    pub occupancy: f64,
    pub b_factor: f64,
    /// anisotropic temperature factors U11, U22, U33, U12, U13, U23 scaled by
    /// 10^4
    pub anisou: Option<[i32; 6]>,
}

impl Structure {
    /// Builds a structure from parsed pdb records. SIGATM and SIGUIJ
    /// records, removed from the v3.3 specification, are not kept.
    pub fn from_pdb(pdb: &PdbFile<Vec<Record>>) -> Structure {
        let records = pdb.records();
        let mut structure = Structure {
            models: models(records),
            assemblies: pdb.biological_assemblies().unwrap_or_default(),
            resolution: pdb.resolution().flatten(),
            ..Default::default()
        };
        for record in records {
            match record {
                Record::Header(h) => {
                    structure.id = Some(h.id_code.clone()).filter(|i| !i.is_empty());
                    structure.classification = Some(h.classification.clone());
                    structure.deposition_date = Some(h.deposition_date);
                }
                Record::Title(t) => structure.title = Some(t.title.clone()),
                Record::Experimental(e) => structure.experimental_methods = e.techniques.clone(),
                Record::Cryst1(c) => structure.cell = Some(c.clone()),
                Record::Ssbond(s) => structure.disulfides.push(s.clone()),
                Record::Link(l) => structure.links.push(l.clone()),
                _ => {}
            }
        }
        structure.entities = entities(records, &structure.models);
        structure
    }

    /// Converts the structure to pdb records, e.g. for [crate::write].
    pub fn to_pdb(&self) -> PdbFile<Vec<Record>> {
        let mut records = Vec::new();
        if self.id.is_some() || self.classification.is_some() || self.deposition_date.is_some() {
            records.push(Record::Header(Header {
                classification: self.classification.clone().unwrap_or_default(),
                deposition_date: self.deposition_date.unwrap_or_default(),
                id_code: self.id.clone().unwrap_or_default(),
            }));
        }
        if let Some(title) = &self.title {
            records.push(Record::Title(Title {
                title: title.clone(),
            }));
        }
        let polymers: Vec<&Entity> = self
            .entities
            .iter()
            .filter(|e| matches!(e.kind, EntityKind::Polymer { .. }))
            .collect();
        if !polymers.is_empty() {
            let tokens = polymers
                .iter()
                .enumerate()
                .flat_map(|(i, e)| {
                    let mut tokens = vec![Token::MoleculeId(i as u32 + 1)];
                    tokens.extend(e.description.clone().map(Token::Molecule));
                    tokens.push(Token::Chain {
                        identifiers: e.chains.clone(),
                    });
                    tokens
                })
                .collect();
            records.push(Record::Cmpnd(Cmpnd { tokens }));
        }
        if !self.experimental_methods.is_empty() {
            records.push(Record::Experimental(Experimental {
                techniques: self.experimental_methods.clone(),
            }));
        }
        if self.models.len() > 1 {
            records.push(Record::Nummdl(Nummdl {
                num: self.models.len() as u32,
            }));
        }
        if !self.experimental_methods.is_empty() || self.resolution.is_some() {
            records.push(resolution_remark(self.resolution));
        }
        if !self.assemblies.is_empty() {
            records.push(assemblies_remark(&self.assemblies));
        }
        for entity in &polymers {
            if let EntityKind::Polymer { sequence } = &entity.kind {
                for chain in &entity.chains {
                    records.push(Record::Seqres(Seqres {
                        chain_id: chain.chars().next().filter(|c| *c != ' '),
                        num_res: sequence.len() as u32,
                        residues: sequence.clone(),
                    }));
                }
            }
        }
        for entity in &self.entities {
            if let (EntityKind::NonPolymer { component }, Some(name)) =
                (&entity.kind, &entity.description)
            {
                records.push(Record::Hetnam(Hetnam {
                    het_id: component.clone(),
                    name: name.clone(),
                }));
            }
        }
        records.extend(self.disulfides.iter().cloned().map(Record::Ssbond));
        records.extend(self.links.iter().cloned().map(Record::Link));
        records.extend(self.cell.clone().map(Record::Cryst1));
        for model in &self.models {
            if self.models.len() > 1 {
                records.push(Record::Model(crate::ast::types::Model {
                    serial: model.serial,
                }));
            }
            model_records(model, &mut records);
            if self.models.len() > 1 {
                records.push(Record::Endmdl);
            }
        }
        records.push(Record::End);
        records.to_pdb_file()
    }
}

impl PdbFile<Vec<Record>> {
    /// Builds a format independent [Structure] of the entry.
    pub fn structure(&self) -> Structure {
        Structure::from_pdb(self)
    }
}

fn residue_key(a: &crate::ast::types::Atom) -> (&str, i32, Option<char>) {
    (&a.residue_name, a.residue_seq, a.insertion_code)
}

/// Groups coordinate records into models, chain segments and residues.
fn models(records: &[Record]) -> Vec<Model> {
    let mut models = Vec::new();
    let mut model = Model {
        serial: 1,
        chains: Vec::new(),
    };
    // a TER closes the current segment
    let mut open = false;
    for record in records {
        match record {
            Record::Model(m) => {
                model = Model {
                    serial: m.serial,
                    chains: Vec::new(),
                };
                open = false;
            }
            Record::Endmdl => {
                models.push(std::mem::take(&mut model));
                open = false;
            }
            Record::Ter(_) => {
                if let Some(chain) = model.chains.last_mut().filter(|_| open) {
                    chain.terminated = true;
                }
                open = false;
            }
            Record::Atom(a) | Record::Hetatm(a) => {
                let chain_id = a.chain_id.to_string();
                let continues_chain = open && model.chains.last().is_some_and(|c| c.id == chain_id);
                if !continues_chain {
                    model.chains.push(Chain {
                        id: chain_id,
                        terminated: false,
                        residues: Vec::new(),
                    });
                    open = true;
                }
                let chain = model.chains.last_mut().unwrap();
                let hetero = matches!(record, Record::Hetatm(_));
                let same_residue = chain.residues.last().is_some_and(|r| {
                    (r.name.as_str(), r.seq, r.insertion_code) == residue_key(a)
                        && r.hetero == hetero
                });
                if !same_residue {
                    chain.residues.push(Residue {
                        name: a.residue_name.clone(),
                        seq: a.residue_seq,
                        insertion_code: a.insertion_code,
                        hetero,
                        atoms: Vec::new(),
                    });
                }
                chain.residues.last_mut().unwrap().atoms.push(Atom {
                    serial: a.serial,
                    name: a.name.clone(),
                    alt_loc: a.alt_loc,
                    element: a.element.clone(),
                    charge: a.charge,
                    position: [a.x, a.y, a.z],
                    occupancy: a.occupancy,
                    b_factor: a.temp_factor,
                    anisou: None,
                });
            }
            Record::Anisou(u) => {
                let atom = model
                    .chains
                    .last_mut()
                    .and_then(|c| c.residues.last_mut())
                    .and_then(|r| r.atoms.last_mut())
                    .filter(|a| a.serial == u.serial);
                if let Some(atom) = atom {
                    atom.anisou = Some([u.u11, u.u22, u.u33, u.u12, u.u13, u.u23]);
                }
            }
            _ => {}
        }
    }
    if !model.chains.is_empty() {
        models.push(model);
    }
    models
}

/// Coordinate records of a model: atoms, their ANISOU records and a TER
/// after each terminated chain segment, numbered after its last atom.
fn model_records(model: &Model, records: &mut Vec<Record>) {
    let chain_id = |chain: &Chain| chain.id.chars().next().unwrap_or(' ');
    for chain in &model.chains {
        for residue in &chain.residues {
            for atom in &residue.atoms {
                let record = crate::ast::types::Atom {
                    serial: atom.serial,
                    name: atom.name.clone(),
                    alt_loc: atom.alt_loc,
                    residue_name: residue.name.clone(),
                    chain_id: chain_id(chain),
                    residue_seq: residue.seq,
                    insertion_code: residue.insertion_code,
                    x: atom.position[0],
                    y: atom.position[1],
                    z: atom.position[2],
                    occupancy: atom.occupancy,
                    temp_factor: atom.b_factor,
                    element: atom.element.clone(),
                    charge: atom.charge,
                };
                if let Some([u11, u22, u33, u12, u13, u23]) = atom.anisou {
                    let anisou = Anisou {
                        serial: atom.serial,
                        name: atom.name.clone(),
                        alt_loc: atom.alt_loc,
                        residue_name: residue.name.clone(),
                        chain_id: chain_id(chain),
                        residue_seq: residue.seq,
                        insertion_code: residue.insertion_code,
                        u11,
                        u22,
                        u33,
                        u12,
                        u13,
                        u23,
                        element: atom.element.clone(),
                        charge: atom.charge,
                    };
                    records.push(if residue.hetero {
                        Record::Hetatm(record)
                    } else {
                        Record::Atom(record)
                    });
                    records.push(Record::Anisou(anisou));
                } else {
                    records.push(if residue.hetero {
                        Record::Hetatm(record)
                    } else {
                        Record::Atom(record)
                    });
                }
            }
        }
        if chain.terminated {
            let last = chain.residues.last();
            records.push(Record::Ter(Ter {
                serial: last.and_then(|r| r.atoms.last()).map(|a| a.serial + 1),
                residue_name: last.map(|r| r.name.clone()).unwrap_or_default(),
                chain_id: chain_id(chain),
                residue_seq: last.map(|r| r.seq),
                insertion_code: last.and_then(|r| r.insertion_code),
            }));
        }
    }
}

const WATER: [&str; 2] = ["HOH", "DOD"];

/// Derives entities the way wwPDB does: polymers from COMPND molecules
/// (or identical SEQRES sequences when COMPND is missing), then one entity
/// per ligand component outside polymer chains, then water.
fn entities(records: &[Record], models: &[Model]) -> Vec<Entity> {
    let mut sequences: Vec<(String, Vec<String>)> = Vec::new();
    for record in records {
        if let Record::Seqres(s) = record {
            let chain = s.chain_id.unwrap_or(' ').to_string();
            sequences.push((chain, s.residues.clone()));
        }
    }
    let sequence_of = |chain: &str| {
        sequences
            .iter()
            .find(|(c, _)| c == chain)
            .map(|(_, s)| s.clone())
    };
    let mut entities: Vec<Entity> = Vec::new();
    for record in records {
        if let Record::Cmpnd(c) = record {
            for token in &c.tokens {
                match token {
                    Token::MoleculeId(_) => entities.push(Entity {
                        id: entities.len() as u32 + 1,
                        description: None,
                        chains: Vec::new(),
                        kind: EntityKind::Polymer {
                            sequence: Vec::new(),
                        },
                    }),
                    Token::Molecule(name) => {
                        if let Some(e) = entities.last_mut() {
                            e.description = Some(name.clone());
                        }
                    }
                    Token::Chain { identifiers } => {
                        if let Some(e) = entities.last_mut() {
                            e.chains = identifiers.clone();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    // molecules without a sequence are not polymers
    entities.retain_mut(|e| {
        let sequence = e.chains.first().and_then(|c| sequence_of(c));
        match sequence {
            Some(sequence) => {
                e.kind = EntityKind::Polymer { sequence };
                true
            }
            None => false,
        }
    });
    if entities.is_empty() {
        for (chain, sequence) in &sequences {
            let existing = entities
                .iter_mut()
                .find(|e| matches!(&e.kind, EntityKind::Polymer { sequence: s } if s == sequence));
            match existing {
                Some(e) => e.chains.push(chain.clone()),
                None => entities.push(Entity {
                    id: 0,
                    description: None,
                    chains: vec![chain.clone()],
                    kind: EntityKind::Polymer {
                        sequence: sequence.clone(),
                    },
                }),
            }
        }
    }
    let names: HashMap<&str, &str> = records
        .iter()
        .filter_map(|r| match r {
            Record::Hetnam(h) => Some((h.het_id.as_str(), h.name.as_str())),
            _ => None,
        })
        .collect();
    let polymer_chains: Vec<String> = entities.iter().flat_map(|e| e.chains.clone()).collect();
    let mut ligands: Vec<Entity> = Vec::new();
    let mut water: Option<Entity> = None;
    for chain in models.iter().take(1).flat_map(|m| &m.chains) {
        // residues of a terminated segment belong to the polymer, including
        // modified residues read from HETATM records
        if chain.terminated {
            continue;
        }
        let polymer_chain = polymer_chains.contains(&chain.id);
        for residue in chain.residues.iter().filter(|r| r.hetero || !polymer_chain) {
            let is_water = WATER.contains(&residue.name.as_str());
            let entity = if is_water {
                water.get_or_insert_with(|| Entity {
                    id: 0,
                    description: Some("water".to_owned()),
                    chains: Vec::new(),
                    kind: EntityKind::Water,
                })
            } else {
                let index = ligands.iter().position(|e| {
                    matches!(&e.kind, EntityKind::NonPolymer { component } if *component == residue.name)
                });
                let index = index.unwrap_or_else(|| {
                    ligands.push(Entity {
                        id: 0,
                        description: names.get(residue.name.as_str()).map(|n| n.to_string()),
                        chains: Vec::new(),
                        kind: EntityKind::NonPolymer {
                            component: residue.name.clone(),
                        },
                    });
                    ligands.len() - 1
                });
                &mut ligands[index]
            };
            if !entity.chains.contains(&chain.id) {
                entity.chains.push(chain.id.clone());
            }
        }
    }
    entities.extend(ligands);
    entities.extend(water);
    for (i, entity) in entities.iter_mut().enumerate() {
        entity.id = i as u32 + 1;
    }
    entities
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{parse, write};
    use std::path::PathBuf;

    const TWO_CHAINS: &str = "\
HEADER    HYDROLASE                               20-APR-99   1ABC
COMPND    MOL_ID: 1;
COMPND   2 MOLECULE: LYSOZYME;
COMPND   3 CHAIN: A, B
SEQRES   1 A    2  MET ALA
SEQRES   1 B    2  MET ALA
HETNAM     SO4 SULFATE ION
ATOM      1  N   MET A   1      11.104   6.134  -6.504  1.00 22.33           N
ANISOU    1  N   MET A   1     2406   2692   3374    -14   -196    -84       N
ATOM      2  CA  ALA A   2      11.639   6.071  -5.147  1.00 21.02           C
TER       3      ALA A   2
ATOM      4  N   MET B   1       1.104   6.134  -6.504  1.00 22.33           N
ATOM      5  CA  ALA B   2       1.639   6.071  -5.147  1.00 21.02           C
TER       6      ALA B   2
HETATM    7  S   SO4 B 101      10.000  10.000  10.000  1.00 30.00           S
HETATM    8  O   HOH A 201       5.000   5.000   5.000  1.00 20.00           O
HETATM    9  O   HOH B 202       6.000   6.000   6.000  1.00 20.00           O
END
";

    #[test]
    fn chain_segments() {
        let structure = parse(TWO_CHAINS).structure();
        assert_eq!(structure.id.as_deref(), Some("1ABC"));
        let [model] = &structure.models[..] else {
            panic!()
        };
        let segments: Vec<_> = model
            .chains
            .iter()
            .map(|c| (c.id.as_str(), c.terminated, c.residues.len()))
            .collect();
        assert_eq!(
            segments,
            [
                ("A", true, 2),
                ("B", true, 2),
                ("B", false, 1),
                ("A", false, 1),
                ("B", false, 1)
            ]
        );
        let atom = &model.chains[0].residues[0].atoms[0];
        assert_eq!(atom.position, [11.104, 6.134, -6.504]);
        assert_eq!(atom.anisou, Some([2406, 2692, 3374, -14, -196, -84]));
        assert!(model.chains[2].residues[0].hetero);
    }

    #[test]
    fn entities() {
        let structure = parse(TWO_CHAINS).structure();
        let entities: Vec<_> = structure
            .entities
            .iter()
            .map(|e| (e.id, e.description.as_deref(), e.chains.join(",")))
            .collect();
        assert_eq!(
            entities,
            [
                (1, Some("LYSOZYME"), "A,B".to_owned()),
                (2, Some("SULFATE ION"), "B".to_owned()),
                (3, Some("water"), "A,B".to_owned())
            ]
        );
        assert_eq!(
            structure.entities[0].kind,
            EntityKind::Polymer {
                sequence: vec!["MET".to_owned(), "ALA".to_owned()]
            }
        );
    }

    #[test]
    fn written_structure_reads_back() {
        let structure = parse(TWO_CHAINS).structure();
        let written = write(&structure.to_pdb());
        assert_eq!(parse(&written).structure(), structure);
    }

    #[test]
    fn multiple_models() {
        let pdb = parse(
            "MODEL        1
ATOM      1  N   MET A   1      -8.901   4.127  -0.555  1.00  0.00           N
TER       2      MET A   1
ENDMDL
MODEL        2
ATOM      1  N   MET A   1      -8.521   3.920  -0.325  1.00  0.00           N
TER       2      MET A   1
ENDMDL
",
        );
        let structure = pdb.structure();
        assert_eq!(structure.models.len(), 2);
        assert_eq!(structure.models[1].serial, 2);
        assert_eq!(
            structure.models[1].chains[0].residues[0].atoms[0].position,
            [-8.521, 3.920, -0.325]
        );
        let written = write(&structure.to_pdb());
        assert_eq!(parse(&written).structure(), structure);
    }

    #[test]
    fn coordinates_written_from_structure_match_file() {
        let is_coordinate = |l: &&str| {
            ["ATOM  ", "HETATM", "ANISOU", "TER"]
                .iter()
                .any(|r| l.starts_with(r))
        };
        for entry in ["1BXO", "1NLS", "1BYI"] {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("res")
                .join(format!("{}.pdb", entry));
            let content = std::fs::read_to_string(path).unwrap();
            let structure = parse(&content).structure();
            let written = write(&structure.to_pdb());
            assert_eq!(parse(&written).structure(), structure, "{}", entry);
            let original: Vec<_> = content
                .lines()
                .filter(is_coordinate)
                .map(|l| format!("{:<80}", l))
                .collect();
            let from_structure: Vec<_> = written.lines().filter(is_coordinate).collect();
            assert_eq!(from_structure, original, "{}", entry);
        }
    }
}
