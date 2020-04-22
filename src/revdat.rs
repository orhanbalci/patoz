/*!
Contains parsers related to [Revdat](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#REVDAT) records.

REVDAT records contain a history of the modifications made to an entry since its release.
*/
use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{line_ending, space0, space1},
    do_parse, many1, map, named, opt, take,
};

use itertools::Itertools;

use std::{str, str::FromStr};

#[allow(dead_code)]
#[derive(Debug)]
struct RevdatLine {
    modification_number: u32,
    continuation: u32,
    rest: String,
}

named!(
    revdat_line_parser<RevdatLine>,
    do_parse!(
        revdat
            >> take!(1)
            >> modification_number: threedigit_integer
            >> cont: opt!(twodigit_integer)
            >> rest: till_line_ending
            >> line_ending
            >> (RevdatLine {
                modification_number,
                continuation: if let Some(cc) = cont { cc } else { 0 },
                rest: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
            })
    )
);

named!(
    revdat_line_folder<Vec<RevdatLine>>,
    map!(many1!(revdat_line_parser), |revdat: Vec<RevdatLine>| {
        revdat
            .into_iter()
            .group_by(|a| a.modification_number)
            .into_iter()
            .map(|(k, v)| RevdatLine {
                modification_number: k,
                continuation: 0,
                rest: String::from_utf8(v.fold(Vec::new(), |accu: Vec<u8>, sr: RevdatLine| {
                    accu.into_iter().chain(sr.rest.into_bytes()).collect()
                }))
                .unwrap(),
            })
            .collect::<Vec<_>>()
    })
);

named!(
    #[doc=r#"
Parses Revdat record which is a multiline continuation record.
If successfull returns [Record](../ast/types/enum.Record.html) variant containing [Revdats](../ast/types/struct.Revdats.html) instance.

## Record Structure

| COLUMNS  | DATA  TYPE     | FIELD         | DEFINITION                                    |
|----------|----------------|---------------|-----------------------------------------------|
| 1 -  6   | Record name    | REVDAT        |                                               |
| 8 - 10   | Integer        | modNum        | Modification number.                          |
| 11 - 12  | Continuation   | continuation  | Allows concatenation of multiple records.     |
| 14 - 22  | Date           | modDate       | Date of modification (or release  for         |
|          |                |               | new entries)  in DD-MMM-YY format. This is    |
|          |                |               | not repeated on continued lines.              |
| 24 - 27  | IDCode         | modId         | ID code of this entry. This is not repeated on|
|          |                |               | continuation lines.                           |
| 32       | Integer        | modType       | An integer identifying the type of            |
|          |                |               | modification. For all  revisions, the         |
|          |                |               | modification type is listed as 1              |
| 40 - 45  | LString(6)     | record        | Modification detail.                          |
| 47 - 52  | LString(6)     | record        | Modification detail.                          |
| 54 - 59  | LString(6)     | record        | Modification detail.                          |
| 61 - 66  | LString(6)     | record        | Modification detail.                          |

    "#],
    pub revdat_record_parser<Record>,
    map! (map!(revdat_line_folder, |revdat: Vec<RevdatLine>| {
        revdat
            .into_iter()
            .map(|r: RevdatLine| {
                let input = r.rest.into_bytes();
                let single_modification_parser_result = revdat_inner_parser(input.as_slice());
                match single_modification_parser_result {
                    Ok((_, mut single_revdat_record)) => {
                        single_revdat_record.modification_number = r.modification_number;                       
                        single_revdat_record
                    }
                    _ => Revdat {
                        modification_number: 0,
                        modification_date: chrono::naive::MIN_DATE,
                        idcode: String::new(),
                        modification_type: ModificationType::InitialRelease,
                        modification_detail: Vec::new(),
                    },
                }
            })
            .collect()
    }), |r : Vec<Revdat>| { Record::Revdats(Revdats{revdat : r})})
);

named!(
    revdat_inner_parser<Revdat>,
    do_parse!(
        space0
            >> modification_date: date_parser
            >> space1
            >> idcode: alphanum_word
            >> space1
            >> modification_type: modification_type_parser
            >> space1
            >> modification_detail: idcode_list
            >> (Revdat {
                modification_number: 0,
                modification_date,
                idcode,
                modification_type,
                modification_detail,
            })
    )
);
#[cfg(test)]
mod test {

    #[test]
    fn revdat() {
        let res =  super::revdat_record_parser(
            r#"REVDAT   7   13-JUL-11 1BXO    1       VERSN                                    
REVDAT   6   24-FEB-09 1BXO    1       VERSN                                    
REVDAT   5   01-APR-03 1BXO    1       JRNL                                     
REVDAT   4   26-SEP-01 1BXO    3       ATOM   CONECT                            
REVDAT   3   24-JAN-01 1BXO    3       ATOM                                     
REVDAT   2   22-DEC-99 1BXO    4       HEADER COMPND REMARK JRNL                
REVDAT   2 2                           ATOM   SOURCE SEQRES                     
REVDAT   1   14-OCT-98 1BXO    0                                                                            
"#
                .as_bytes(),
        );
        match res {
            Ok((_, _rest)) => assert!(true),
            Err(_err) => assert!(false),
        }
    }
}
