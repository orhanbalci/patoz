use super::{ast::types::*, primitive::*};
use nom::{character::complete::line_ending, do_parse, named};

named!(
    pub remark_record_parser<Record>,
    do_parse!(
        remark
        >> till_line_ending
        >> line_ending
        >> (Record::Remark)));
