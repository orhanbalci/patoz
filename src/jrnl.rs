use super::entity::*;
use super::primitive::*;
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, named, opt, tag, take_str,
};

use crate::author::author_list_parser;

use std::{marker::PhantomData, str, str::FromStr};

use crate::make_line_folder;

#[allow(dead_code)]
struct JrnlAuthorLine;

#[allow(dead_code)]
struct JrnlTitleLine;

#[allow(dead_code)]
struct JrnlEditLine;

#[allow(dead_code)]
struct JrnlPublLine;

#[allow(dead_code)]
#[derive(Default, Clone)]
struct JrnlRefLine {
    continuation: u32,
    publication_name: String,
    volume: Option<u32>,
    page: Option<u32>,
    year: Option<u32>,
}

named!(
    jrnl_author_line_parser<Continuation<JrnlAuthorLine>>,
    do_parse!(
        jrnl >> space1
            >> auth
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<JrnlAuthorLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(
    jrnl_author_line_folder,
    jrnl_author_line_parser,
    JrnlAuthorLine
);

named!(
    pub (crate) jrnl_author_record_parser<Record>,
    map!(jrnl_author_line_folder, |jrnl_author: Vec<u8>| {
        author_list_parser(jrnl_author.as_slice())
            .map(|res| Record::JournalAuthors { authors: res.1 })
            .expect("Can not parse journal author record")
    })
);

named!(
    jrnl_title_line_parser<Continuation<JrnlTitleLine>>,
    do_parse!(
        jrnl >> space1
            >> titl
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<JrnlTitleLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(
    jrnl_title_line_folder,
    jrnl_title_line_parser,
    JrnlTitleLine
);

named!(
    pub (crate) jrnl_title_record_parser<Record>,
    map!(jrnl_title_line_folder, |jrnl_title: Vec<u8>| {
        Record::JournalTitle{ title : String::from(str::from_utf8(jrnl_title.as_slice()).unwrap())}
    })
);

named!(
    jrnl_edit_line_parser<Continuation<JrnlEditLine>>,
    do_parse!(
        jrnl >> space1
            >> edit
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<JrnlEditLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(jrnl_edit_line_folder, jrnl_edit_line_parser, JrnlEditLine);

named!(
    pub (crate) jrnl_edit_record_parser<Record>,
    map!(jrnl_edit_line_folder, |jrnl_edit: Vec<u8>| {
        if let Ok((_, res)) = author_list_parser(jrnl_edit.as_slice()) {
            Record::JournalEditors { name: res }
        } else {
            Record::JournalEditors { name: Vec::new() }
        }
    })
);

named!(
    jrnl_ref_line_parser<JrnlRefLine>,
    do_parse!(
        jrnl >> space1
            >> tag!("REF")
            >> space1
            >> cont: opt!(integer)
            >> publication_name: take_str!(28)
            >> opt!(take_str!(2))
            >> space0
            >> volume: opt!(integer)
            >> page: opt!(integer)
            >> year: opt!(integer)
            >> (JrnlRefLine {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                publication_name: publication_name.to_owned(),
                volume,
                page,
                year,
            })
    )
);

named!(
    jrnl_ref_line_folder<JrnlRefLine>,
    fold_many1!(
        jrnl_ref_line_parser,
        JrnlRefLine::default(),
        |acc: JrnlRefLine, item: JrnlRefLine| {
            JrnlRefLine {
                continuation: acc.continuation,
                publication_name: acc.publication_name + &item.publication_name,
                page: acc.page.or(item.page),
                volume: acc.volume.or(item.volume),
                year: acc.year.or(item.year),
            }
        }
    )
);

named!(
    pub (crate) jrnl_ref_record_parser<Record>,
    map!(jrnl_ref_line_folder, |jrnl_ref : JrnlRefLine| { Record::JournalReference{
        publication_name : jrnl_ref.publication_name,
        volume : jrnl_ref.volume,
        page : jrnl_ref.page,
        year : jrnl_ref.year,
    }})
);

named!(
    jrnl_publ_line_parser<Continuation<JrnlPublLine>>,
    do_parse!(
        jrnl >> space1
            >> tag!("PUBL")
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<JrnlPublLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(jrnl_publ_line_folder, jrnl_publ_line_parser, JrnlPublLine);

named!(
    pub (crate) jrnl_publ_record_parser<Record>,
    map!(jrnl_publ_line_folder, |jrnl_publ: Vec<u8>| {
        Record::JournalPublication{ publication : String::from(str::from_utf8(jrnl_publ.as_slice()).unwrap())}
    })
);

#[cfg(test)]
mod test {
    use super::jrnl_title_record_parser;
    use crate::entity::Record;

    #[test]
    fn test_jrnl_title() {
        let res = jrnl_title_record_parser(
            r#"JRNL        TITL   THE CRYSTAL STRUCTURE OF  HUMAN DEOXYHAEMOGLOBIN AT           
JRNL        TITL 2 1.74 A RESOLUTION      
"#
            .as_bytes(),
        );

        match res {
            Ok((_, r)) => {
                if let Record::JournalTitle { title: name } = r {
                    assert_eq!(
                        name,
                        "THE CRYSTAL STRUCTURE OF  HUMAN DEOXYHAEMOGLOBIN AT 1.74 A RESOLUTION"
                    );
                }
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
}
