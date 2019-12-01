use super::primitive::*;
use nom::character::complete::space0;
use nom::{do_parse, named};

named!(
    nummdl_parser<u32>,
    do_parse!(nummdl >> space0 >> model_number: integer >> (model_number))
);
