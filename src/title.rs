/*!
Contains parsers related to [Title](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#TITLE)
records.The TITLE record contains a title for the experiment or analysis that is represented in the entry.
*/
use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, named, opt, take,
};

use crate::make_line_folder;
use std::marker::PhantomData;

#[allow(dead_code)]
struct TitleLine;

named!(
    title_line_parser<Continuation<TitleLine>>,
    do_parse!(
        title
            >> take!(2)
            >> cont: opt!(twodigit_integer)
            >> space1
            >> tit: title_parser
            >> space0
            >> line_ending
            >> (Continuation::<TitleLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: tit,
                phantom: PhantomData,
            })
    )
);

make_line_folder!(title_line_folder, title_line_parser, TitleLine);

named!( #[doc=r#"Parses title record which is a continuation record. This record may span multi lines. There is
only one title record per pdb file. 
If succesfull returns [Record](../ast/types/enum.Record.html) variant containing [Title](../ast/types/struct.Title.html) instance

Record structure is given below

| COLUMNS    | DATA  TYPE    | FIELD        | DEFINITION                                |
|------------|---------------|--------------|-------------------------------------------|
| 1 -  6     | Record name   | TITLE        |                                           |
| 9 - 10     | Continuation  | continuation | Allows concatenation of multiple records. |
| 11 - 80    | String        | title        | Title of the  experiment.                 |
"#], 
    pub title_record_parser<Record>,
    map!(
        title_line_folder,
        |title: Vec<u8>| if let Ok(res) = String::from_utf8(title) {
            println!("Title {:?}", res);
            Record::Title(Title { title: res })
        } else {
            Record::Title(Title {
                title: "".to_owned(),
            })
        }
    )
);
