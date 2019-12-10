use super::{entity::*, primitive::*};
use nom::{
    character::complete::{anychar, space1},
    do_parse, many0, map, named, opt,
};

use itertools::Itertools;

#[allow(dead_code)]
struct SeqresLine {
    serial_number: u32,
    chain_id: Option<char>,
    num_res: u32,
    residues: Vec<String>,
}

named!(
    seqres_line_parser<SeqresLine>,
    do_parse!(
        seqres
            >> space1
            >> serial_number: integer
            >> space1
            >> chain_id: opt!(anychar)
            >> space1
            >> num_res: integer
            >> residues: residue_list_parser
            >> (SeqresLine {
                serial_number,
                chain_id,
                num_res,
                residues,
            })
    )
);

named!(seqres_parser<Vec<SeqresLine>>, many0!(seqres_line_parser));

named!(
    seqres_record_parser<Vec<Record>>,
    map!(seqres_parser, |seqres: Vec<SeqresLine>| {
        seqres
            .into_iter()
            .group_by(|a| a.chain_id)
            .into_iter()
            .map(|(k, v)| Record::Seqres {
                chain_id: k,
                residues: v.fold(Vec::new(), |v: Vec<String>, sr: SeqresLine| {
                    v.into_iter().chain(sr.residues).collect()
                }),
            })
            .collect::<Vec<_>>()
    })
);
