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
struct JrnlRefLine {
    continuation: u32,
    publication_name: String,
    volume: u32,
    page: u32,
    year: u32,
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
    jrnl_author_record_parser<Vec<Author>>,
    map!(jrnl_author_line_folder, |jrnl_author: Vec<u8>| {
        if let Ok((_, res)) = author_list_parser(jrnl_author.as_slice()) {
            res
        } else {
            Vec::new()
        }
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
    pub (crate) jrnl_title_record_parser<String>,
    map!(jrnl_title_line_folder, |jrnl_title: Vec<u8>| {
        String::from(str::from_utf8(jrnl_title.as_slice()).unwrap())
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
    jrnl_edit_record_parser<Vec<Author>>,
    map!(jrnl_edit_line_folder, |jrnl_edit: Vec<u8>| {
        if let Ok((_, res)) = author_list_parser(jrnl_edit.as_slice()) {
            res
        } else {
            Vec::new()
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
            >> take_str!(2)
            >> space0
            >> volume: integer
            >> page: integer
            >> year: integer
            >> (JrnlRefLine {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                publication_name: publication_name.to_owned(),
                volume,
                page,
                year,
            })
    )
);

#[cfg(test)]
mod test {
    use super::jrnl_title_record_parser;

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
                assert_eq!(
                    r,
                    "THE CRYSTAL STRUCTURE OF  HUMAN DEOXYHAEMOGLOBIN AT 1.74 A RESOLUTION"
                );
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
}
