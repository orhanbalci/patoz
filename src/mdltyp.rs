/*!
Contains parsers related to [Mdltyp](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#MDLTYP) records.

The MDLTYP record contains additional annotation pertinent to the coordinates presented in the entry.
*/
use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, named, opt,
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
    mdltyp_parser<Record>,
    do_parse!(
        space0
            >> structural_annotation: structural_annotation_list_parser
            >> (Record::Mdltyp(Mdltyp {
                structural_annotation,
            }))
    )
);

named!(
#[doc=r#"
Parses MDLTYP record which is a continuation record which can span multi lines. Contains `;` separated list of annotations. If successfull returns [Record](../ast/types/enum.Record.html) variant that contains [Mdltyp](../ast/types/struct.Mdltyp.html)

*Record Structure:*

| COLUMNS   | DATA TYPE     | FIELD        | DEFINITION                                 |
|-----------|---------------|--------------|--------------------------------------------|
| 1 -  6    | Record name   | MDLTYP       |                                            |
| 9 - 10    | Continuation  | continuation | Allows concatenation of multiple records.  |
| 11 - 80   | SList         | comment      | Free Text providing  additional structural |
|           |               |              | annotation.                                |
"#],
    pub mdltyp_record_parser<Record>,
    map!(mdltyp_line_folder, |mdltyp: Vec<u8>| {
        if let Ok((_, res)) = mdltyp_parser(mdltyp.as_slice()) {
            res
        } else {
            Record::Mdltyp(Mdltyp {
                structural_annotation : Vec::new()
            })
        }
    })
);
