use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{anychar, space1},
    do_parse, named, opt, tag,
};

named!(
    pub dbref1_record_parser<Record>,
    do_parse!(
        dbref1
        >> space1
        >> idcode : idcode_parser_len
        >> space1
        >> chain_id : anychar
        >> tag!(" ")
        >> seq_begin : fourdigit_integer
        >> initial_sequence : opt!(anychar)
        >> tag!(" ")
        >> seq_end : fourdigit_integer
        >> ending_sequence : opt!(anychar)
        >> space1
        >> database : alphanum_word
        >> space1
        >> db_idcode : db_id_code_parser_len
        >> till_line_ending
        >> (Record::Dbref1(
            Dbref1{
                idcode,
                chain_id,
                seq_begin,
                initial_sequence,
                seq_end,
                ending_sequence,
                database,
                db_idcode
            }
            ))
    )
);

named!(
    pub dbref2_record_parser<Record>,
    do_parse!(
        dbref2
        >> space1
        >> idcode : idcode_parser_len
        >> space1
        >> chain_id : anychar
        >> space1
        >> db_accession : alphanum_word
        >> five_space
        >> seq_begin : fivedigit_integer
        >> space1
        >> seq_end : fivedigit_integer
        >> till_line_ending
        >>(Record::Dbref2(Dbref2{
        idcode, chain_id, db_accession, seq_begin, seq_end})
        )
    )
);
