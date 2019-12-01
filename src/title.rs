use super::entity::*;
use super::primitive::*;
use nom::character::complete::{line_ending, space0, space1};
use nom::{do_parse, fold_many1, map, named, opt, take};

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
            >> tit: alphanum_word_with_spaces_inside
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

named!(
    pub (crate) title_record_parser<Record>,
    map!(
        title_line_folder,
        |title: Vec<u8>| if let Ok(res) = String::from_utf8(title) {
            println!("Title {:?}", res);
            Record::Title { title: res }
        } else {
            Record::Title {
                title: "".to_owned(),
            }
        }
    )
);
