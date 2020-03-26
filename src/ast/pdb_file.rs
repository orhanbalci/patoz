use super::types::*;
use std::iter::Iterator;
pub struct PdbFile<I> {
    records: I,
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
    pub fn title(&mut self) -> Option<Title> {
        self.records
            .find(|s| match s {
                Record::Title(_) => true,
                _ => false,
            })
            .map(|r| match r {
                Record::Title(tit) => tit.clone(),
                _ => Title::default(),
            })
    }

    pub fn header(&mut self) -> Option<Header> {
        self.records
            .find(|s| match s {
                Record::Header(_) => true,
                _ => false,
            })
            .map(|r| match r {
                Record::Header(header) => header.clone(),
                _ => Header::default(),
            })
    }

    pub fn obslte(&mut self) -> Option<Obslte> {
        self.records
            .find(|s| match s {
                Record::Obslte(_) => true,
                _ => false,
            })
            .map(|r| match r {
                Record::Obslte(obslte) => obslte.clone(),
                _ => Obslte::default(),
            })
    }

    pub fn caveat(&mut self) -> Option<Caveat> {
        self.records
            .find(|s| match s {
                Record::Caveat(_) => true,
                _ => false,
            })
            .map(|r| match r {
                Record::Caveat(caveat) => caveat.clone(),
                _ => Caveat::default(),
            })
    }

    pub fn sprsde(&mut self) -> Option<Sprsde> {
        self.records
            .find(|s| match s {
                Record::Sprsde(_) => true,
                _ => false,
            })
            .map(|r| match r {
                Record::Sprsde(sprsde) => sprsde.clone(),
                _ => Sprsde::default(),
            })
    }

    pub fn mdltyp(&mut self) -> Option<Mdltyp> {
        self.records
            .find(|s| match s {
                Record::Mdltyp(_) => true,
                _ => false,
            })
            .map(|r| match r {
                Record::Mdltyp(mdltyp) => mdltyp.clone(),
                _ => Mdltyp::default(),
            })
    }

    pub fn revdats(&mut self) -> Option<Revdats> {
        self.records
            .find(|s| match s {
                Record::Revdats(_) => true,
                _ => false,
            })
            .map(|r| match r {
                Record::Revdats(revdat) => revdat.clone(),
                _ => Revdats::default(),
            })
    }

    pub fn cmpnd(&mut self) -> Option<Cmpnd> {
        self.records
            .find(|s| match s {
                Record::Cmpnd(_) => true,
                _ => false,
            })
            .map(|r| match r {
                Record::Cmpnd(cmpnd) => cmpnd.clone(),
                _ => Cmpnd::default(),
            })
    }

    pub fn source(&mut self) -> Option<Source> {
        self.records
            .find(|s| match s {
                Record::Source(_) => true,
                _ => false,
            })
            .map(|r| match r {
                Record::Source(src) => src.clone(),
                _ => Source::default(),
            })
    }

    pub fn keywds(&mut self) -> Option<Keywds> {
        self.records
            .find(|s| match s {
                Record::Keywds(_) => true,
                _ => false,
            })
            .map(|r| match r {
                Record::Keywds(kw) => kw.clone(),
                _ => Keywds::default(),
            })
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_pdb_file_constructor() {
        use super::{super::types::Record, *};
        let a = vec![Record::Title(Title {
            title: "a".to_owned(),
        })];
        let pf = PdbFile::new(a.iter());
        let t = pf.header().title();
        assert_eq!(t.unwrap().title, "a".to_owned());
    }
}
