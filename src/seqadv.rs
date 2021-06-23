use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{anychar, space1},
    do_parse, named, opt, tag,
};

named!(
    pub seqadv_record_parser<Record>,
    do_parse!(
        seqadv
        >> space1
        >> idcode : idcode_parser_len
        >> space1
        >> conflicting_residue : residue_parser
        >> space1
        >> chain_id : anychar
        >> tag!(" ")
        >> sequence_number : fourdigit_integer
        >> insertion_code : opt!(anychar)
        >> space1
        >> database : alphanum_word
        >> space1
        >> db_accession : alphanum_word
        >> space1
        >> sequence_db_residue : opt!(residue_parser)
        >> space1
        >> sequence_db_sequence_number : opt!(integer)
        >> space1
        >> conflict : alphanum_word_with_spaces_inside
        >> till_line_ending
        >> (Record::Seqadv(Seqadv{
            idcode,
            conflicting_residue,
            chain_id,
            sequence_number,
            insertion_code,
            database,
            db_accession,
            sequence_db_residue,
            sequence_db_sequence_number,
            conflict
        }))
    )
);
