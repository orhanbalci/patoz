use super::types::*;
use std::iter::Iterator;

macro_rules! impl_record_filter {
    ($fn_name : ident -> $match_type: ident -> $ret_type :ident ) => {
        pub fn $fn_name(&mut self) -> Option<$ret_type> {
             self.records
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

impl<'a> ToPdbFile for &'a Vec<Record> {
    type I = std::slice::Iter<'a, Record>;
    fn to_pdb_file(self) -> PdbFile<Self::I> {
        PdbFile {
            records: self.iter(),
        }
    }
}

impl<'a, I> PdbFile<I>
where
    I: Iterator<Item = &'a Record>,
{
    pub fn new(records: I) -> Self {
        PdbFile { records }
    }

    pub fn header(self) -> PdbHeader<I> {
        PdbHeader {
            records: self.records,
        }
    }
}

pub struct PdbHeader<I> {
    records: I,
}

impl<'a, I> PdbHeader<I>
where
    I: Iterator<Item = &'a Record>,
{
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

    pub fn journal(self) -> PdbJournal<I> {
        PdbJournal {
            records: self.records,
        }
    }
}

pub struct PdbJournal<I> {
    records: I,
}

impl<'a, I> PdbJournal<I>
where
    I: Iterator<Item = &'a Record>,
{
    impl_record_filter!(authors -> JournalAuthors -> JournalAuthors);
    impl_record_filter!(title -> JournalTitle -> JournalTitle);
    impl_record_filter!(editors -> JournalEditors -> JournalEditors);
    impl_record_filter!(reference -> JournalReference -> JournalReference);
    impl_record_filter!(citation -> JournalCitation -> JournalCitation);
    impl_record_filter!(publication -> JournalPublication -> JournalPublication);
    impl_record_filter!(pubmedid -> JournalPubMedId -> JournalPubMedId);
    impl_record_filter!(doi -> JournalDoi -> JournalDoi);
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
        let tit = a.to_pdb_file().header().title();
        assert_eq!(tit.unwrap().title, "a".to_owned());
        assert_eq!(a.to_pdb_file().header().nummdl().unwrap().num, 1);
    }
}
