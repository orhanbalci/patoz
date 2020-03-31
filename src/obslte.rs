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

named!( #[doc=r#"
Parses obslte records which is a continuation type of record. Continuation records are single records
that span multi lines. There is only one OBSLTE record for each pdb file which is optional.
If succesfull returns [Record](../ast/types/enum.Record.html) variant containing [Obslte](../ast/types/struct.Obslte.html) instance

Record structure : 

| COLUMNS    | DATA  TYPE    | FIELD         | DEFINITION                               |
|------------|---------------|---------------|------------------------------------------|
| 1 -  6     | Record name   | OBSLTE        |                                          |
| 9 - 10     | Continuation  | continuation  | Allows concatenation of multiple records |
| 12 - 20    | Date          | repDate       | Date that this entry was replaced.       |
| 22 - 25    | IDcode        | idCode        | ID code of this entry.                   |
| 32 - 35    | IDcode        | rIdCode       | ID code of entry that replaced this one. |
| 37 - 40    | IDcode        | rIdCode       | ID code of entry that replaced this one. |
| 42 - 45    | IDcode        | rIdCode       | ID code of entry  that replaced this one.|
| 47 - 50    | IDcode        | rIdCode       | ID code of entry that replaced this one. |
| 52 - 55    | IDcode        | rIdCode       | ID code of entry that replaced this one. |
| 57 - 60    | IDcode        | rIdCode       | ID code of entry that replaced this one. |
| 62 - 65    | IDcode        | rIdCode       | ID code of entry that replaced this one. |
| 67 - 70    | IDcode        | rIdCode       | ID code of entry that replaced this one. |
| 72 - 75    | IDcode        | rIdCode       | ID code of entry that replaced this one. |
"#],
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
