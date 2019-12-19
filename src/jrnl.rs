use super::entity::*;
use super::primitive::*;
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, named, opt,
};

use crate::author::author_list_parser;

use std::{marker::PhantomData, str, str::FromStr};

use crate::make_line_folder;

#[allow(dead_code)]
struct JrnlAuthorLine;

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
