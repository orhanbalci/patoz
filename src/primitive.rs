use chrono::format::strftime::StrftimeItems;
use chrono::format::Parsed;
use chrono::NaiveDate;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace1, space0, space1};
use nom::character::{is_alphanumeric, is_digit, is_space};
use nom::{
    alt, do_parse, fold_many0, map_res, named, separated_list, tag, take, take_str, take_till,
    take_while, IResult,
};
use std::result::Result;
use std::str;
use std::str::FromStr;

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
                        acc.into_iter().chain(item.remaining.into_bytes()).collect()
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
make_tagger!(end);

named!(
    pub twodigit_integer<u32>,
    map_res!(map_res!(take!(2), str::from_utf8), str::FromStr::from_str)
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
        map_res!(take_while!(|s| {is_alphanumeric(s) || is_space(s)}), str::from_utf8),
        str::FromStr::from_str
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

named!(
    pub chain_value_parser<&[u8],Vec<String>>,
    separated_list!(tag!(","), alphanum_word_with_spaces_inside)
);

named!(pub ec_value_parser<&[u8],Vec<String>>,
        separated_list!(
                        tag!(","),
                        map_res!(
                            map_res!(
                                take_while!(|c : u8| {c == b'.' || is_digit(c) || is_space(c)}), str::from_utf8)
                            , str::FromStr::from_str)
                    )

);

named!(
    pub yes<bool>,
    map_res!(tag!("YES"), |_| -> Result<bool, ()> { Ok(true) })
);

named!(
    pub no<bool>,
    map_res!(tag!("NO"), |_| -> Result<bool, ()> { Ok(false) })
);

named!(pub yes_no_parser<bool>, alt!(yes | no));

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
make_token_tagger!(organism_tax_id);
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

named!(pub till_line_ending<&[u8]>, take_till!(|c| char::from(c) == '\r' || char::from(c) == '\n'));

named!(pub residue_parser<&str>, take_str!(3));
named!(pub residue_list_parser<&[u8], Vec<&str>>, separated_list!(multispace1, residue_parser));

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
}
