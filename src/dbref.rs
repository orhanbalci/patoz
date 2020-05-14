use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{anychar, space1},
    do_parse, named,
};

named!(
    pub dbref_record_parser<Record>,
    do_parse!(
        dbref
        >> space1
        >> idcode : alphanum_word
        >> space1
        >> chain_id : anychar
        >> space1
        >> seq_begin : fourdigit_integer
        >> space1
        >> initial_sequence : anychar
        >> space1 
        >> seq_end : fourdigit_integer
        >> space1
        >> ending_sequence : anychar
        >> space1
        >> database : alphanum_word
        >> space1
        >> db_accession : alphanum_word
        >> space1
        >> db_idcode : alphanum_word
        >> space1 
        >> db_seq_begin : fourdigit_integer
        >> space1
        >> idbns_begin : anychar
        >> space1
        >> db_seq_end  : fourdigit_integer
        >> space1
        >> dbins_end : fourdigit_integer
        >> till_line_ending
        >> (Record::Dbref(
            Dbref{
                idcode,
                chain_id,
                seq_begin,
                initial_sequence,
                seq_end,
                ending_sequence,
                database,
                db_accession,
                db_idcode,
                db_seq_begin,
                idbns_begin,
                db_seq_end,
                dbins_end,
            }
            ))
    )
);
