/*!
Contains parsers related to [Split](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SPLIT)
records.
The SPLIT record is used in instances where a specific entry composes part of a large macromolecular complex.
It will identify the PDB entries that are required to reconstitute a complete complex.
*/
use super::{ast::types::*, primitive::*};
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
    do_parse!(space0 >> ids: idcode_list >> (Record::Split(Split { id_codes: ids })))
);

named!(#[doc = r#"Parses SPLIT records. It is a continuation type of record which can span multi lines.
There is only one SPLIT record per pdb file. If successfull  returns [Record](../ast/types/enum.Record.html) variant 
containing [Split](../ast/types/struct.Split.html) instance.

Record structure : 

|COLUMNS  |  DATA TYPE    | FIELD        | DEFINITION                                |
|---------|---------------|--------------|------------------------------------------ |
| 1 -  6  |  Record  name | "SPLIT "     |                                           |
| 9 - 10  |  Continuation | continuation | Allows concatenation of multiple records. |
|12 - 15  |  IDcode       | idCode       | ID code of related entry.                 |
|17 - 20  |  IDcode       | idCode       | ID code of related entry.                 |
|22 - 25  |  IDcode       | idCode       | ID code of related entry.                 |
|27 – 30  |  IDcode       | idCode       | ID code of related entry.                 |
|32 - 35  |  IDcode       | idCode       | ID code of related entry.                 |
|37 - 40  |  IDcode       | idCode       | ID code of related entry.                 |
|42 - 45  |  IDcode       | idCode       | ID code of related entry.                 |
|47 - 50  |  IDcode       | idCode       | ID code of related entry.                 |
|52 - 55  |  IDcode       | idCode       | ID code of related entry.                 |
|57 - 60  |  IDcode       | idCode       | ID code of related entry.                 |
|62 - 65  |  IDcode       | idCode       | ID code of related entry.                 |
|67 - 70  |  IDcode       | idCode       | ID code of related entry.                 |
|72 - 75  |  IDcode       | idCode       | ID code of related entry.                 |
|77 - 80  |  IDcode       | idCode       | ID code of related entry.                 |
"#],
    pub split_record_parser<Record>,
    map!(split_line_folder, |split: Vec<u8>| {
        if let Ok((_, res)) = split_parser(split.as_slice()) {
            res
        } else {
            Record::Split(Split::default())
        }
    })
);
