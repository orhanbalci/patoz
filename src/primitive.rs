use chrono::format::strftime::StrftimeItems;
use chrono::format::Parsed;
use chrono::Datelike;
use chrono::NaiveDate;
use nom::bytes::complete::tag;
use nom::character::complete::{
    alpha1, alphanumeric1, digit1, multispace0, multispace1, space0, space1,
};
use nom::character::{is_alphanumeric, is_space};

use super::entity::{Header, Obslte};
use nom::{
    alt, do_parse, fold_many0, map, map_res, named, opt, tag, take, take_str, take_while, IResult,
};
use std::result::Result;
use std::str;
use std::str::FromStr;

macro_rules! make_tagger(

    ($fnname:ident) =>(
            pub fn $fnname(s : &str) -> IResult<&str, &str>{
                tag(stringify!($fnname).to_ascii_uppercase().as_str())(s)
            }
        );
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

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_date_parser() {
        let temp: NaiveDate = date_parser("12-SEP-09".as_bytes()).unwrap().1;
        assert_eq!(temp.day(), 12);
        assert_eq!(temp.year(), 9);
    }

}
