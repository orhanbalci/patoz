use super::{ast::types::*, primitive::*};
use nom::{
    character::complete::{anychar, line_ending, space1},
    do_parse, named, opt,
};

named!(
   pub modres_record_parser<Record>,
    do_parse!(
        modres
            >> space1
           >> idcode : idcode_parser_len
           >> space1
           >> residue_name : residue_parser
           >> space1
           >> chain_id : anychar
           >> space1
           >> sequence_number : integer
           >> insertion_code : opt!(anychar) // TODO this should be none empty char.
           >> space1
           >> standart_residue_name : residue_parser
           >> space1
           >> comment : alphanum_word_with_spaces_inside
           >> line_ending
           >> (Record::Modres(
               Modres{
                    idcode,
                    residue_name,
                    chain_id,
                    sequence_number,
                    insertion_code,
                    standart_residue_name,
                    comment
               }
           ))
    )
);

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test1() {
        match super::modres_record_parser(
            r#"MODRES 2R0L ASN A   74  ASN  GLYCOSYLATION SITE  
"#
            .as_bytes(),
        ) {
            Ok((_, Record::Modres(res))) => {
                assert_eq!(res.idcode, "2R0L");
                assert_eq!(res.residue_name, "ASN");
                assert_eq!(res.chain_id, 'A');
                assert_eq!(res.sequence_number, 74);
                assert_eq!(res.insertion_code, Some(' '));
                assert_eq!(res.standart_residue_name, "ASN");
            }
            Ok((_, _)) => {
                println!("Unexpected record type");
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
}
