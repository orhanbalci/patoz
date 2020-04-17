/*!
Contains parsers related to [Keywds](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#KEYWDS) records.
The KEYWDS record contains a set of terms relevant to the entry.
*/
use super::{ast::types::*, primitive::*};
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
    #[doc=r#"Parses KEYWDS record which is a multiline continuation record. Contains comma-seperated list of  keywords relevant to pdb entry.If successfull returns [Record](../ast/types/enum.Record.html) variant containing [KEYWDS](../ast/types/struct.Keywds.html) instance.


 Record structure :

| COLUMNS | DATA  TYPE   | FIELD        | DEFINITION                                   |
|---------|--------------|--------------|----------------------------------------------|
| 1 -  6  | Record name  | KEYWDS       |                                              |
| 9 - 10  | Continuation | continuation | Allows concatenation of records if necessary.|
| 11 - 79 | List         | keywds       | Comma-separated list of keywords relevant    |
|         |              |              | to the entry.                                |

 "#],
    pub keywds_parser<Record>,
    map!(keywds_line_folder, |v: Vec<u8>| keywds_value_parser(
        v.as_slice()
    )
    .map(|res| Record::Keywds (Keywds{ keywords: res.1 }))
    .expect("Can not parse keywds record"))
);
