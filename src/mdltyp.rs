use super::{entity::*, primitive::*};
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, named, opt,
};

use crate::make_line_folder;

use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct MdltypLine;

named!(
    mdltyp_line_parser<Continuation<MdltypLine>>,
    do_parse!(
        mdltyp
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<MdltypLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(mdltyp_line_folder, mdltyp_line_parser, MdltypLine);

named!(
    mdltyp_record_parser<Record>,
    do_parse!(
        space0
            >> structural_annotation: structural_annotation_list_parser
            >> (Record::Mdltyp {
                structural_annotation,
            })
    )
);
