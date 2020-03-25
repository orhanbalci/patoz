use super::{ast::types::Record, primitive::*};
use nom::{character::complete::space0, do_parse, named};

named!(
    pub (crate) nummdl_record_parser<Record>,
    do_parse!(nummdl >> space0 >> model_number: integer >> (Record::Nummdl { num: model_number }))
);
