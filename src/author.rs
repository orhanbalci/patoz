use super::{entity::*, primitive::*};
use nom::{
    bytes::complete::take_while,
    character::{
        complete::{line_ending, space0, space1},
        is_alphanumeric, is_space,
    },
    do_parse, fold_many1, map, map_res, named, opt, separated_list, tag, Err,
};

use crate::make_line_folder;

use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct AuthorLine;

named!(
    author_line_parser<Continuation<AuthorLine>>,
    do_parse!(
        author
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<AuthorLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(author_line_folder, author_line_parser, AuthorLine);

named!(
    author_value_parser<Author>,
    map_res!(
        map_res!(
            map_res!(
                take_while(|s| {
                    is_alphanumeric(s)
                        || is_space(s)
                        || char::from(s) == '.'
                        || char::from(s) == '\''
                }),
                str::from_utf8
            ),
            str::FromStr::from_str
        ),
        |s: String| {
            println!("{}", s);
            Result::Ok::<Author, Err<String>>(Author(String::from_str(s.trim()).unwrap()))
        }
    )
);

named!(
    pub (crate) author_list_parser<Vec<Author>>,
    separated_list!(tag!(","), author_value_parser)
);

named!(
    pub (crate) author_record_parser<Record>,
    map!(author_line_folder, |v: Vec<u8>| {
        println!("{}", str::from_utf8(&v).unwrap());
        author_list_parser(v.as_slice())
            .map(|res| Record::Authors { authors: res.1 })
            .expect("Can not parse author record")
    })
);
