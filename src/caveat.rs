/*!
Contains parsers related to  [Caveat](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#CAVEAT) records.
CAVEAT warns of errors and unresolved issues in the entry. Use caution when using an entry containing this record.
*/
use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, named, opt, take,
};

use crate::make_line_folder;
use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct CaveatLine;

named!(
    caveat_line_parser<Continuation<CaveatLine>>,
    do_parse!(
        caveat
            >> take!(2)
            >> cont: opt!(twodigit_integer)
            >> space1
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<CaveatLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(caveat_line_folder, caveat_line_parser, CaveatLine);

named!(
    caveat_parser<Record>,
    do_parse!(
        space0
            >> id_code: alphanum_word
            >> space0
            >> comment: alphanum_word_with_spaces_inside
            >> space0
            >> (Record::Caveat(Caveat {
                id_code: id_code,
                comment: comment,
            }))
    )
);

named!(#[doc = r#"Parses CAVEAT records. It is a continuation type of record which can span multi lines.
There is only one CAVEAT record per pdb file. If successfull  returns [Record](../ast/types/enum.Record.html) variant 
containing [Caveat](../ast/types/struct.Split.html) instance. Structure of the record is : 

| COLUMNS   | DATA  TYPE    | FIELD        | DEFINITION                                   |
|-----------|---------------|--------------|----------------------------------------------|
|   1 - 6   | Record name   | CAVEAT       |                                              |
|  9 - 10   | Continuation  | continuation | Allows concatenation of multiple records.    |
| 12 - 15   | IDcode        | idCode       | PDB ID code of this entry.                   |
| 20 - 79   | String        | comment      | Free text giving the reason for the  CAVEAT. |
"#],
    pub  caveat_record_parser<Record>,
    map!(caveat_line_folder, |caveat: Vec<u8>| {
        if let Ok((_, res)) = caveat_parser(caveat.as_slice()) {
            res
        } else {
            Record::Caveat(Caveat::default())
        }
    })
);
