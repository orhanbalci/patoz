use super::{entity::*, primitive::*};
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, named, opt,
};

use crate::make_line_folder;

use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct KeywdsLine;

named!(
    keywds_line_parser<Continuation<KeywdsLine>>,
    do_parse!(
        keywds
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<KeywdsLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(keywds_line_folder, keywds_line_parser, KeywdsLine);

named!(
    pub (crate) keywds_parser<Record>,
    map!(keywds_line_folder, |v: Vec<u8>| chain_value_parser(
        v.as_slice()
    )
    .map(|res| Record::Keywds { keywords: res.1 })
    .expect("Can not parse keywds record"))
);
