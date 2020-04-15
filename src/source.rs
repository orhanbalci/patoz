/*!
Contains parsers related to [Source](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SOURCE) records.
The SOURCE record specifies the biological or chemical source of each molecule in this entry..
*/
use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, named, opt,
};

use super::compnd::tokens_parser;
use crate::make_line_folder;

use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct SourceLine;

named!(
    source_line_parser<Continuation<SourceLine>>,
    do_parse!(
        source
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<SourceLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(source_line_folder, source_line_parser, SourceLine);

named!(
#[doc=r#"parses source record which is a multiline continuation record. Contains a list of comma separated predefined key-value pairs.
Predefined keys are called tokens and can be found in [Token](../ast/types/enum.Token.html)
If succesfull returns [Record](../ast/types/enum.Record.html) variant containing [SOURCE](../ast/types/struct.Source.html) instance.
Record layout :
   
| COLUMNS   | DATA  TYPE     | FIELD         | DEFINITION                               |
|-----------|----------------|---------------|------------------------------------------|
| 1 -  6    | Record name    | "SOURCE"      |                                          |
| 8 - 10    | Continuation   | continuation  | Allows concatenation of multiple records.|
| 11 - 79   | Specification  | srcName       | Identifies the source of the             |
|           | List           |               | macromolecule in a  token: value format. |
    "#],
    pub source_token_parser<Record>,
    map!(
        source_line_folder,
        |v: Vec<u8>| tokens_parser(v.as_slice()).map(|res| Record::Source(Source{tokens : res.1})).expect("Can not parse source record")
    )
);

#[cfg(test)]
mod test {

    #[test]
    fn source() {
        if let Ok((_, _res)) = super::source_token_parser(br#"SOURCE    MOL_ID: 1;                                                            
SOURCE   2 ORGANISM_SCIENTIFIC: CRAMBE HISPANICA SUBSP ABYSSINICA;                                             
SOURCE   3 STRAIN: SUBSP ABYSSINICA  
"#){
assert!(true)
        }else{
            assert!(false)
        }
    }
}
