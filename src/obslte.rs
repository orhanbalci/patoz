/*!
Contains parsers related to [Obslte](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#OBSLTE)
records. Obslte record indicates that this entry is removed from PDB and replaced with another entry.
*/
use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, named, opt, take,
};

use crate::make_line_folder;
use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct ObslteLine;

named!(
    obslte_line_parser<Continuation<ObslteLine>>,
    do_parse!(
        obslte
            >> take!(2)
            >> cont: opt!(twodigit_integer)
            >> space1
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<ObslteLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(obslte_line_folder, obslte_line_parser, ObslteLine);

named!(
    obslte_parser<Record>,
    do_parse!(
        space0
            >> cont_date: date_parser
            >> space0
            >> ids: idcode_list
            >> (Record::Obslte(Obslte {
                replacement_date: cont_date,
                replacement_ids: ids
            }))
    )
);

named!( #[doc=r#""#],
    pub obslte_record_parser<Record>,
    map!(obslte_line_folder, |obslte: Vec<u8>| {
        println!("{}", str::from_utf8(obslte.as_slice()).unwrap());
        if let Ok((_, res)) = obslte_parser(obslte.as_slice()) {
            res
        } else {
            println!("Obslte parser error");
            Record::Obslte(Obslte {
                replacement_date: chrono::naive::MIN_DATE,
                replacement_ids: Vec::new(),
            })
        }
    })
);
