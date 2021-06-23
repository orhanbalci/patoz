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

impl<'a> PdbJournal<&'a mut Vec<Record>> {
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

impl<'a> PrimaryStructure<&'a mut Vec<Record>> {
    impl_record_filter!(dbreference -> Dbref -> Dbref);
    impl_record_filter!(conflicts -> Seqadv -> Seqadv);
    impl_record_filter!(residues -> Seqres -> Seqres);
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
