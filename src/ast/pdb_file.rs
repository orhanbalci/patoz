use super::types::*;
use std::iter::Iterator;

macro_rules! impl_record_filter {
    ($fn_name : ident -> $match_type: ident -> $ret_type :ident ) => {
        pub fn $fn_name(&mut self) -> Option<$ret_type> {
            self.records
                .iter()
                .find(|s| match s {
                    Record::$match_type(_) => true,
                    _ => false,
                })
                .map(|r| match r {
                    Record::$match_type(a) => a.clone(),
                    _ => $match_type::default(),
                })
        }
    };
}

pub struct PdbFile<I> {
    records: I,
}

pub trait ToPdbFile {
    type I;
    fn to_pdb_file(self) -> PdbFile<Self::I>;
}

impl ToPdbFile for Vec<Record> {
    type I = Vec<Record>;
    fn to_pdb_file(self) -> PdbFile<Self::I> {
        PdbFile { records: self }
    }
}

impl PdbFile<Vec<Record>> {
    /// all parsed records in file order
    pub fn records(&self) -> &[Record] {
        &self.records
    }

    pub fn header(&mut self) -> PdbHeader<&mut Vec<Record>> {
        PdbHeader {
            records: &mut self.records,
        }
    }

    pub fn primary(&mut self) -> PrimaryStructure<&mut Vec<Record>> {
        PrimaryStructure {
            records: &mut self.records,
        }
    }

    pub fn coordinates(&self) -> Coordinates<'_> {
        Coordinates {
            records: &self.records,
        }
    }

    fn remark(&self, number: u32) -> Option<&Remark> {
        self.records.iter().find_map(|r| match r {
            Record::Remark(r) if r.number == number => Some(r),
            _ => None,
        })
    }

    /// resolution in angstroms from REMARK 2. `Some(None)` when resolution
    /// is not applicable, e.g. for NMR entries; `None` if REMARK 2 is missing
    /// or not understood
    pub fn resolution(&self) -> Option<Option<f64>> {
        crate::remark::resolution(self.remark(2)?)
    }

    /// residues not located in the experiment, from REMARK 465
    pub fn missing_residues(&self) -> Option<Vec<MissingResidue>> {
        crate::remark::missing_residues(self.remark(465)?)
    }

    /// biological assemblies from REMARK 350
    pub fn biological_assemblies(&self) -> Option<Vec<BiologicalAssembly>> {
        crate::remark::biological_assemblies(self.remark(350)?)
    }

    /// record counts declared in the MASTER record
    pub fn master(&self) -> Option<&Master> {
        self.records.iter().find_map(|r| match r {
            Record::Master(m) => Some(m),
            _ => None,
        })
    }
}

/// coordinate section records of all models in file order
pub struct Coordinates<'a> {
    records: &'a [Record],
}

impl<'a> Coordinates<'a> {
    /// atoms of ATOM records
    pub fn atoms(&self) -> impl Iterator<Item = &'a Atom> {
        self.records.iter().filter_map(|r| match r {
            Record::Atom(a) => Some(a),
            _ => None,
        })
    }

    /// atoms of HETATM records
    pub fn hetero_atoms(&self) -> impl Iterator<Item = &'a Atom> {
        self.records.iter().filter_map(|r| match r {
            Record::Hetatm(a) => Some(a),
            _ => None,
        })
    }

    /// anisotropic temperature factors of ANISOU records
    pub fn anisou(&self) -> impl Iterator<Item = &'a Anisou> {
        self.records.iter().filter_map(|r| match r {
            Record::Anisou(a) => Some(a),
            _ => None,
        })
    }
}

pub struct PdbHeader<I> {
    records: I,
}

impl<'a> PdbHeader<&'a mut Vec<Record>> {
    impl_record_filter!(nummdl -> Nummdl -> Nummdl);
    impl_record_filter!(obslte -> Obslte -> Obslte);
    impl_record_filter!(caveat -> Caveat -> Caveat);
    impl_record_filter!(sprsde -> Sprsde -> Sprsde);
    impl_record_filter!(mdltyp -> Mdltyp -> Mdltyp);
    impl_record_filter!(revdats  -> Revdats -> Revdats);
    impl_record_filter!(cmpnd -> Cmpnd -> Cmpnd);
    impl_record_filter!(source -> Source -> Source);
    impl_record_filter!(authors -> Authors -> Authors);
    impl_record_filter!(title -> Title -> Title);
    impl_record_filter!(header -> Header -> Header);
    impl_record_filter!(keywds -> Keywds -> Keywds);
    impl_record_filter!(expdta ->  Experimental -> Experimental);

    pub fn journal(&'a mut self) -> PdbJournal<&'a mut Vec<Record>> {
        PdbJournal {
            records: self.records,
        }
    }
}
pub struct PdbJournal<I> {
    records: I,
}

impl PdbJournal<&mut Vec<Record>> {
    impl_record_filter!(authors -> JournalAuthors -> JournalAuthors);
    impl_record_filter!(title -> JournalTitle -> JournalTitle);
    impl_record_filter!(editors -> JournalEditors -> JournalEditors);
    impl_record_filter!(reference -> JournalReference -> JournalReference);
    impl_record_filter!(citation -> JournalCitation -> JournalCitation);
    impl_record_filter!(publication -> JournalPublication -> JournalPublication);
    impl_record_filter!(pubmedid -> JournalPubMedId -> JournalPubMedId);
    impl_record_filter!(doi -> JournalDoi -> JournalDoi);
}

pub struct PrimaryStructure<I> {
    records: I,
}

impl PrimaryStructure<&mut Vec<Record>> {
    impl_record_filter!(dbreference -> Dbref -> Dbref);
    impl_record_filter!(conflicts -> Seqadv -> Seqadv);
    impl_record_filter!(residues -> Seqres -> Seqres);

    /// residue sequences of all chains
    pub fn sequences(&self) -> Vec<Seqres> {
        self.records
            .iter()
            .filter_map(|r| match r {
                Record::Seqres(s) => Some(s.clone()),
                _ => None,
            })
            .collect()
    }
}
#[cfg(test)]
mod test {
    #[test]
    fn test_pdb_file_constructor() {
        use super::{super::types::Record, *};
        let a = vec![
            Record::Title(Title {
                title: "a".to_owned(),
            }),
            Record::Nummdl(Nummdl { num: 1 }),
        ];
        let mut parsed_pdb = a.to_pdb_file();
        let tit = parsed_pdb.header().title();
        assert_eq!(tit.unwrap().title, "a".to_owned());
        assert_eq!(parsed_pdb.header().nummdl().unwrap().num, 1);
    }
}
