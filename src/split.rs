use super::{entity::*, primitive::*};
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, named, opt, take,
};

use crate::make_line_folder;
use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct SplitLine;

named!(
    split_line_parser<Continuation<SplitLine>>,
    do_parse!(
        split
            >> take!(2)
            >> cont: opt!(twodigit_integer)
            >> space1
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<SplitLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(split_line_folder, split_line_parser, SplitLine);

named!(
    split_parser<Record>,
    do_parse!(space0 >> ids: idcode_list >> (Record::Split { id_codes: ids }))
);

named!(
    pub (crate) split_record_parser<Record>,
    map!(split_line_folder, |split: Vec<u8>| {
        if let Ok((_, res)) = split_parser(split.as_slice()) {
            res
        } else {
            Record::Split {
                id_codes: Vec::new(),
            }
        }
    })
);
