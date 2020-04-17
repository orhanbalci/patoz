/*!
Contains parsers related to [Author](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#AUTHOR) records.
The AUTHOR record contains the names of the people responsible for the contents of the entry.
*/
use super::{ast::types::*, primitive::*};
use nom::{
    bytes::complete::{tag, take_while},
    character::{
        complete::{line_ending, space0, space1},
        is_alphanumeric, is_space,
    },
    do_parse, fold_many1, map, map_res,
    multi::separated_list,
    named, opt, Err, IResult,
};

use crate::make_line_folder;

use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct AuthorLine;

named!(
    author_line_parser<Continuation<AuthorLine>>,
    do_parse!(
        author
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<AuthorLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(author_line_folder, author_line_parser, AuthorLine);

named!(
    author_value_parser<Author>,
    map_res!(
        map_res!(
            map_res!(
                take_while(|s| {
                    is_alphanumeric(s)
                        || is_space(s)
                        || char::from(s) == '.'
                        || char::from(s) == '\''
                        || char::from(s) == '-'
                }),
                str::from_utf8
            ),
            str::FromStr::from_str
        ),
        |s: String| {
            Result::Ok::<Author, Err<String>>(Author(String::from_str(s.trim()).unwrap()))
        }
    )
);

///parses , separated author names. If successfull returns list of
///[Authors](../ast/types/struct.Author.html)
pub fn author_list_parser(s: &[u8]) -> IResult<&[u8], Vec<Author>> {
    separated_list(tag(","), author_value_parser)(s)
}

named!(
#[doc=r#"Parses AUTHOR record which is a multiline continuation record. Contains comma-seperated list of author names. If successfull returns [Record](../ast/types/enum.Record.html) variant containing [AUTHORS](../ast/types/struct.Authors.html) instance.

Record structure :

| COLUMNS | DATA  TYPE   | FIELD        | DEFINITION                                   |
|---------|--------------|--------------|----------------------------------------------|
| 1 -  6  | Record name  | AUTHOR       |                                              |
| 9 - 10  | Continuation | continuation | Allows concatenation of multiple records.    |
| 11 - 79 | List         | authorList   | List of the author names, separated          |
|         |              |              | by commas.                                   |

"#],
    pub author_record_parser<Record>,
    map!(author_line_folder, |v: Vec<u8>| {
        author_list_parser(v.as_slice())
            .map(|res| Record::Authors(Authors { authors: res.1 }))
            .expect("Can not parse author record")
    })
);
