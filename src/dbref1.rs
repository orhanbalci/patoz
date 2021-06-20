use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{anychar, line_ending, space0, space1},
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
        >> space1
        >> db_seq_begin : fivedigit_integer
        >> space1
        >> db_seq_end : fivedigit_integer
        >> till_line_ending
        >>(Record::Dbref2(Dbref2{
        idcode, chain_id, db_accession, db_seq_begin, db_seq_end})
        )
    )
);

named!(
    pub dbref_partial_parser<Record>,
    do_parse!(
        space0
        >> ref1 : dbref1_record_parser
        >> line_ending
        >> ref2 : dbref2_record_parser
        >> (
            match (ref1, ref2) {
                (Record::Dbref1(r1), Record::Dbref2(r2)) => Record::Dbref(merge_db_ref(r1,r2)),
                _ => Record::Dbref(Dbref::default())
            }
        )
    )
);

#[cfg(test)]
mod test {
    use crate::Record;

    #[test]
    pub fn dbref1() {
        use super::dbref_partial_parser;
        if let Ok((_, Record::Dbref(res))) = dbref_partial_parser(
            r#"DBREF1 1ABC A   61    322 UNIMES               UPI000148A153
DBREF2 1ABC A     MES00005880000                     61         322 
"#
            .as_bytes(),
        ) {
            assert_eq!(res.idcode, "1ABC");
        } else {
            assert!(false);
        }
    }
}
