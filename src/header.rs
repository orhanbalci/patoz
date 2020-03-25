use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{line_ending, multispace1, space0},
    do_parse, map, named, take_str,
};

named!(
    pub (crate) header_parser<Record>,
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
