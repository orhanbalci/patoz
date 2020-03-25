use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{anychar, space1},
    do_parse, many0, map, named, opt,
};

use itertools::Itertools;

#[allow(dead_code)]
pub struct SeqresLine {
    serial_number: u32,
    chain_id: Option<char>,
    num_res: u32,
    residues: Vec<String>,
}
named!(#[doc=r#"Parses a line of [SEQRES](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQRES) record.

|COLUMNS    |   DATA TYPE     | FIELD      |  DEFINITION                                           |
|-----------|-----------------|------------|-------------------------------------------------------|
|1 -  6     |    Record name  |  SEQRES    |                                                       |
|8 - 10     |    Integer      |  serNum    |   Serial number of the SEQRES record for  the         |
|           |                 |            |   current  chain. Starts at 1 and increments          |
|           |                 |            |   by one  each line. Reset to 1 for each chain.       |
|12         |    Character    |  chainID   |   Chain identifier. This may be any single            |
|           |                 |            |   legal  character, including a blank which is        |
|           |                 |            |   is  used if there is only one chain.                |
|14 - 17    |    Integer      |  numRes    |   Number of residues in the chain.                    |
|           |                 |            |   This  value is repeated on every record.            |
|20 - 22    |    Residue name |  resName   |   Residue name.                                       |
|24 - 26    |    Residue name |  resName   |   Residue name.                                       |
|28 - 30    |    Residue name |  resName   |   Residue name.                                       |
|32 - 34    |    Residue name |  resName   |   Residue name.                                       |
|36 - 38    |    Residue name |  resName   |   Residue name.                                       |
|40 - 42    |    Residue name |  resName   |   Residue name.                                       |
|44 - 46    |    Residue name |  resName   |   Residue name.                                       |
|48 - 50    |    Residue name |  resName   |   Residue name.                                       |
|52 - 54    |    Residue name |  resName   |   Residue name.                                       |
|56 - 58    |    Residue name |  resName   |   Residue name.                                       |
|60 - 62    |    Residue name |  resName   |   Residue name.                                       |
|64 - 66    |    Residue name |  resName   |   Residue name.                                       |
|68 - 70    |    Residue name |  resName   |   Residue name.                                       |
"#],
    pub seqres_line_parser<SeqresLine>,
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
            .map(|(k, v)| {
                Record::Seqres(Seqres {
                    chain_id: k,
                    residues: v.fold(Vec::new(), |v: Vec<String>, sr: SeqresLine| {
                        v.into_iter().chain(sr.residues).collect()
                    }),
                })
            })
            .collect::<Vec<_>>()
    })
);
