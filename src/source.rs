use super::{entity::*, primitive::*};
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
    pub (crate) source_token_parser<Record>,
    map!(
        source_line_folder,
        |v: Vec<u8>| tokens_parser(v.as_slice()).map(|res| Record::Source{tokens : res.1}).expect("Can not parse source record")
    )
);

#[cfg(test)]
mod test {

    #[test]
    fn source() {
        if let Ok((_, res)) = super::source_token_parser(br#"SOURCE    MOL_ID: 1;                                                            
SOURCE   2 ORGANISM_SCIENTIFIC: CRAMBE HISPANICA SUBSP ABYSSINICA;                                             
SOURCE   3 STRAIN: SUBSP ABYSSINICA  
"#){
assert!(true)
        }else{
            assert!(false)
        }
    }
}
