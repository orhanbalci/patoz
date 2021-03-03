/*!
Contains parsers related to [Header](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#HEADER)
records. Header record gives information about identity of this pdb file.
*/
use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{line_ending, multispace1, space0},
    do_parse, map, named, take_str,
};

named!(#[doc=r#"Parses a line of [Header](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#HEADER) record.
This type of record is neither separated to multi lines nor repeated. There is just single line of unique header record in a pdb file.
If succesfull returns [Record](../ast/types/enum.Record.html) variant containing [Header](../ast/types/struct.Header.html) instance

Record structure :

| COLUMNS      | DATA  TYPE   | FIELD          | DEFINITION                                |
|--------------|--------------|----------------|-------------------------------------------|
| 1 -  6       | Record name  | HEADER         |                                           |
| 11 - 50      | String(40)   | classification | Classifies the molecule(s).               |
| 51 - 59      | Date         | depDate        | Deposition date. This is the date the     |
|              |              |                | coordinates  were received at the PDB.    |
| 63 - 66      | IDcode       | idCode         | This identifier is unique within the PDB. |
"#],

    pub header_parser<Record>,
    do_parse!(
        header
            >> multispace1
            >> classification_p: map!(take_str!(40), str::trim)
            >> deposition_date_p: date_parser
            >> multispace1
            >> id_code_p: take_str!(4)
            >> space0
            >> line_ending
            >> (Record::Header (Header{
                classification: classification_p.to_string(),
                deposition_date: deposition_date_p,
                id_code: id_code_p.to_string()
            }))
    )
);
