use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, named, opt,
};

use crate::make_line_folder;

use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct SprsdeLine;

named!(
    sprsde_line_parser<Continuation<SprsdeLine>>,
    do_parse!(
        sprsde
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<SprsdeLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(sprsde_line_folder, sprsde_line_parser, SprsdeLine);

named!(
    sprsde_parser<Record>,
    do_parse!(
        space0
            >> sprsde_date: date_parser
            >> space1
            >> id_code: alphanum_word
            >> space1
            >> superseeded: idcode_list
            >> (Record::Sprsde(Sprsde {
                sprsde_date,
                id_code,
                superseeded,
            }))
    )
);

named!(
    pub (crate) sprsde_record_parser<Record>,
    map!(sprsde_line_folder, |sprsde: Vec<u8>| {
        if let Ok((_, res)) = sprsde_parser(sprsde.as_slice()) {
            res
        } else {
            Record::Sprsde(Sprsde {
                sprsde_date: chrono::naive::MIN_DATE,
                id_code: String::new(),
                superseeded: Vec::new(),
            })
        }
    })
);
