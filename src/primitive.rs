use chrono::{
    format::{strftime::StrftimeItems, Parsed},
    NaiveDate,
};
use nom::{
    alt,
    branch::alt,
    bytes::complete::{tag, take_till, take_while},
    character::{
        complete::{alpha1, alphanumeric1, digit1, multispace1, space0, space1},
        is_alphanumeric, is_digit, is_space,
    },
    combinator::map_res,
    do_parse, fold_many0, map_res,
    multi::separated_list,
    named, separated_list, tag, take, take_str, IResult,
};
use std::{result::Result, str, str::FromStr};

#[macro_use]

macro_rules! make_tagger(
    ($fnname:ident) =>(
            pub fn $fnname(s : &[u8]) -> IResult<&[u8], &[u8]>{
                tag(stringify!($fnname).to_ascii_uppercase().as_str())(s)
            }
        );
    );

#[macro_export]
macro_rules! make_token_tagger(
    ($tokenname : ident) => (
            named!(
            pub $tokenname<()>,
            do_parse!(
                tag!(stringify!($tokenname).to_ascii_uppercase().as_str())
                >> tag!(":")
                >> ()
            )
        );
    );
);

#[macro_export]
macro_rules! make_token_parser(
    ($parser_name : ident, $tagger_name : ident, $value_parser : ident, $parse_val : ident, $ret_val : expr) => (
        named!(
            pub $parser_name<Token>,
            do_parse!(
                space0
                    >> $tagger_name
                    >> space1
                    >> $parse_val : $value_parser
                    >> space0
                    >> ($ret_val)
            )
        );
    );
);

#[macro_export]
macro_rules! make_line_folder (
    ($parser_name : ident, $line_parser : ident, $line_type : ty) => {
        named!(
            $parser_name<Vec<u8>>,
            fold_many1!(
                    $line_parser,
                    Vec::new(),
                    |acc : Vec<u8>, item : Continuation<$line_type>|{
                        let rem = if acc.len() > 0 { " ".to_owned() + &item.remaining }else{ item.remaining };
                        let trimmed =  rem.trim_end();
                        acc.into_iter().chain(trimmed.bytes()).collect()
                    }
                )
            );
    };
);

make_tagger!(master);
make_tagger!(header);
make_tagger!(obslte);
make_tagger!(title);
make_tagger!(split);
make_tagger!(caveat);
make_tagger!(compnd);
make_tagger!(source);
make_tagger!(keywds);
make_tagger!(expdta);
make_tagger!(nummdl);
make_tagger!(mdltyp);
make_tagger!(author);
make_tagger!(revdat);
make_tagger!(sprsde);
make_tagger!(seqres);
make_tagger!(jrnl);
make_tagger!(auth);
make_tagger!(end);
make_tagger!(titl);
make_tagger!(edit);

named!(
    pub twodigit_integer<u32>,
    map_res!(map_res!(take!(2), str::from_utf8), str::FromStr::from_str)
);

named!(
    pub threedigit_integer<u32>,
    map_res!(map_res!(take!(3), str::from_utf8), str::FromStr::from_str)
);

named!(
    pub integer<u32>,
    map_res!(map_res!(digit1, str::from_utf8), str::FromStr::from_str)
);

named!(
    pub integer_with_spaces<u32>,
    do_parse!(space0 >> res: integer >> space0 >> (res))
);

named!(
    pub integer_list<&[u8],Vec<u32>>,
    separated_list!(tag(","), integer_with_spaces)
);

named!(
    pub ascii_word<String>,
    map_res!(map_res!(alpha1, str::from_utf8), String::from_str)
);

named!(
    pub alphanum_word<String>,
    map_res!(
        map_res!(alphanumeric1, str::from_utf8),
        str::FromStr::from_str
    )
);

named!(
    pub alphanum_word_with_spaces_inside<String>,
    map_res!(
        map_res!(take_while(|s| {is_alphanumeric(s) || is_space(s)}), str::from_utf8),
        |s : &str| {str::FromStr::from_str(s.trim())}
    )
);

named!(
    pub molecule_name_parser<String>,
    map_res!(
        map_res!(take_while(|s| {is_alphanumeric(s) || 
            is_space(s) || char::from(s) == '(' || 
            char::from(s) == ')' || 
            char::from(s) == ',' || 
            char::from(s) == '/'
        }), str::from_utf8),
        |s : &str| {str::FromStr::from_str(s.trim())}
    )
);

named!(
    pub month_parser<u32>,
    map_res!(ascii_word, |s: String| -> Result<u32, ()> {
        let mut parsed = Parsed::new();
        chrono::format::parse(&mut parsed, s.as_str(), StrftimeItems::new("%b"))
            .expect("Can not parse month");
        Result::Ok(parsed.month.unwrap())
    })
);

named!(
    pub date_parser<NaiveDate>,
    do_parse!(
        dayp: integer
            >> tag!("-")
            >> monthp: month_parser
            >> tag!("-")
            >> yearp: integer
            >> (NaiveDate::from_ymd(yearp as i32, monthp, dayp))
    )
);

named!(
    pub alphanum_word_space<String>,
    do_parse!(w: alphanum_word >> space1 >> (w))
);

named!(
    pub idcode_list<Vec<String>>,
    fold_many0!(alphanum_word_space, Vec::new(), |mut acc: Vec<String>,
                                                  item: String|
     -> Vec<String> {
        acc.push(item);
        acc
    })
);

pub fn chain_value_parser(s: &[u8]) -> IResult<&[u8], Vec<String>> {
    separated_list(tag(","), alphanum_word_with_spaces_inside)(s)
}

pub fn structural_annotation(s: &[u8]) -> IResult<&[u8], String> {
    map_res(
        map_res(
            take_while(|s: u8| s == b',' || is_alphanumeric(s) || is_space(s)),
            str::from_utf8,
        ),
        str::FromStr::from_str,
    )(s)
}

pub fn structural_annotation_list_parser(s: &[u8]) -> IResult<&[u8], Vec<String>> {
    separated_list(tag(";"), structural_annotation)(s)
}

pub fn ec_value_parser(s: &[u8]) -> IResult<&[u8], Vec<String>> {
    separated_list(
        tag(","),
        map_res(
            map_res(
                take_while(|c: u8| c == b'.' || is_digit(c) || is_space(c)),
                str::from_utf8,
            ),
            str::FromStr::from_str,
        ),
    )(s)
}

pub fn yes(s: &[u8]) -> IResult<&[u8], bool> {
    map_res(tag("YES"), |_| -> Result<bool, ()> { Ok(true) })(s)
}

pub fn no(s: &[u8]) -> IResult<&[u8], bool> {
    map_res(tag("NO"), |_| -> Result<bool, ()> { Ok(false) })(s)
}

pub fn yes_no_parser(s: &[u8]) -> IResult<&[u8], bool> {
    alt((yes, no))(s)
}

use super::entity::ModificationType;

named!(
    pub modification_type_parser<ModificationType>,
    alt!(
        do_parse!(tag!("0") >> (ModificationType::InitialRelease)) |
        do_parse!(tag!("1") >> (ModificationType::OtherModification))
    )
);

make_token_tagger!(mol_id);
make_token_tagger!(molecule);
make_token_tagger!(chain);
make_token_tagger!(fragment);
make_token_tagger!(synonym);
make_token_tagger!(ec);
make_token_tagger!(engineered);
make_token_tagger!(mutation);
make_token_tagger!(other_details);
make_token_tagger!(synthetic);
make_token_tagger!(organism_scientific);
make_token_tagger!(organism_common);
make_token_tagger!(organism_taxid);
make_token_tagger!(strain);
make_token_tagger!(variant);
make_token_tagger!(cell_line);
make_token_tagger!(atcc);
make_token_tagger!(organ);
make_token_tagger!(tissue);
make_token_tagger!(cell);
make_token_tagger!(organelle);
make_token_tagger!(secretion);
make_token_tagger!(cellular_location);
make_token_tagger!(plasmid);
make_token_tagger!(gene);
make_token_tagger!(expression_system);
make_token_tagger!(expression_system_common);
make_token_tagger!(expression_system_tax_id);
make_token_tagger!(expression_system_strain);
make_token_tagger!(expression_system_variant);
make_token_tagger!(expression_system_cell_line);
make_token_tagger!(expression_system_atcc_number);
make_token_tagger!(expression_system_organ);
make_token_tagger!(expression_system_tissue);
make_token_tagger!(expression_system_cell);
make_token_tagger!(expression_system_organelle);
make_token_tagger!(expression_system_cellular_location);
make_token_tagger!(expression_system_vector_type);
make_token_tagger!(expression_system_vector);
make_token_tagger!(expression_system_plasmid);
make_token_tagger!(expression_system_gene);

pub fn till_line_ending(s: &[u8]) -> IResult<&[u8], &[u8]> {
    take_till(|c| char::from(c) == '\r' || char::from(c) == '\n')(s)
}

named!(pub residue_parser<String>, map_res!(alt!(take_str!(3) | take_str!(2) | take_str!(1)), str::FromStr::from_str));

pub fn residue_list_parser(s: &[u8]) -> IResult<&[u8], Vec<String>> {
    separated_list(multispace1, residue_parser)(s)
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_date_parser() {
        let temp: NaiveDate = date_parser("12-SEP-09".as_bytes()).unwrap().1;
        assert_eq!(temp.day(), 12);
        assert_eq!(temp.year(), 9);
    }

    #[test]
    fn test_yes_parser() {
        if let Ok((_, res)) = yes("YES".as_bytes()) {
            assert_eq!(res, true);
        }
    }

    #[test]
    fn test_no_parser() {
        if let Ok((_, res)) = no("NO".as_bytes()) {
            assert_eq!(res, false);
        }
    }

    #[test]
    fn test_token_mol_id_parser() {
        if let Ok((_, _res)) = mol_id("MOL_ID:".as_bytes()) {
            assert!(true);
        }
    }

    #[test]
    fn test_residue_list_parser() {
        let res = residue_list_parser("GLY ILE VAL".as_bytes());
        match res {
            Ok((_, r)) => {
                assert_eq!(r[0], "GLY");
                assert!(true);
            }
            Err(_err) => assert!(false),
        }
    }

    #[test]
    fn test_integer_list_parser() {
        let res = integer_list("1,2,3".as_bytes());
        match res {
            Ok((_, r)) => {
                assert_eq!(r[0], 1);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_structural_annotation_list_parser() {
        let res = structural_annotation_list_parser(
            "CA ATOMS ONLY, CHAIN A, B, C, D, E, F, G, H, I, J, K ; P ATOMS ONLY, CHAIN X, Y, Z-"
                .as_bytes(),
        );
        match res {
            Ok((_, ann)) => {
                assert_eq!(
                    ann[0],
                    "CA ATOMS ONLY, CHAIN A, B, C, D, E, F, G, H, I, J, K "
                );
                assert_eq!(ann[1], " P ATOMS ONLY, CHAIN X, Y, Z");
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
}
