/*!
Contains parsers related to [Sprsde](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SPRSDE) records.
The SPRSDE records contain a list of the ID codes of entries that were made obsolete by the given coordinate entry and removed from the PDB release set.
*/
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
#[doc=r#"Parses sprsde record which is a multiline continuation record. 
If succesfull returns [Record](../ast/types/enum.Record.html) variant containing [SPRSDE](../ast/types/struct.Sprsde.html) instance.

Record layout :

| COLUMNS    | DATA TYPE     | FIELD         | DEFINITION                                |
|------------|---------------|---------------|-------------------------------------------|
|  1 -  6    | Record name   | SPRSDE        |                                           |
|  9 - 10    | Continuation  | continuation  | Allows for multiple ID codes.             |
| 12 - 20    | Date          | sprsdeDate    | Date this entry superseded the listed     |
|            |               |               | entries. This field is not copied on      |
|            |               |               | continuations.                            |
| 22 - 25    | IDcode        | idCode        | ID code of this entry. This field is  not |
|            |               |               | copied on continuations.                  |
| 32 - 35    | IDcode        | sIdCode       | ID code of a superseded entry.            |
| 37 - 40    | IDcode        | sIdCode       | ID code of a superseded entry.            |
| 42 - 45    | IDcode        | sIdCode       | ID code of a superseded entry.            |
| 47 - 50    | IDcode        | sIdCode       | ID code of a superseded entry.            |
| 52 - 55    | IDcode        | sIdCode       | ID code of a superseded entry.            |
| 57 - 60    | IDcode        | sIdCode       | ID code of a superseded entry.            |
| 62 - 65    | IDcode        | sIdCode       | ID code of a superseded entry.            |
| 67 - 70    | IDcode        | sIdCode       | ID code of a superseded entry.            |
| 72 - 75    | IDcode        | sIdCode       | ID code of a superseded entry.            |
"#],
     pub sprsde_record_parser<Record>,
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
