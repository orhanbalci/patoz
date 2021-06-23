use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{anychar, line_ending, space1},
    do_parse, named, opt, tag,
};

named!(
    pub dbref_record_parser<Record>,
    do_parse!(
        dbref
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
        >> db_accession : alphanum_word
        >> space1
        >> db_idcode : db_id_code_parser_len
        >> tag!(" ")
        >> db_seq_begin : fivedigit_integer
        >> idbns_begin : opt!(anychar)
        >> tag!(" ")
        >> db_seq_end  : fivedigit_integer
        >> dbins_end : opt!(anychar)
        >> till_line_ending
        >> line_ending
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

#[cfg(test)]
mod test {
    use super::{super::Record, dbref_record_parser};
    #[test]
    pub fn dbref() {
        if let Ok((_, Record::Dbref(res))) = dbref_record_parser(
            r#"DBREF  2JHQ A    1   226  UNP    Q9KPK8   UNG_VIBCH        1    226  
"#
            .as_bytes(),
        ) {
            assert_eq!(res.idcode, "2JHQ");
        } else {
            assert!(false);
        }
    }
}
