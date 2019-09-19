use chrono::format::strftime::StrftimeItems;
use chrono::format::Parsed;
use chrono::Datelike;
use chrono::NaiveDate;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, multispace0, multispace1};
use nom::{do_parse, map, map_res, named, tag, take_str, IResult};
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

#[derive(Debug)]
struct Header {
    classification: String,
    deposition_date: NaiveDate,
    id_code: String,
}

named!(
    integer<u32>,
    map_res!(map_res!(digit1, str::from_utf8), str::FromStr::from_str)
);

named!(
    ascii_word<String>,
    map_res!(map_res!(alpha1, str::from_utf8), String::from_str)
);

named!(spacer<()>, map!(multispace0, |_| ()));

named!(
    month_parser<u32>,
    map_res!(ascii_word, |s: String| -> Result<u32, ()> {
        let mut parsed = Parsed::new();
        chrono::format::parse(&mut parsed, s.as_str(), StrftimeItems::new("%b"))
            .expect("Can not parse month");
        Result::Ok(parsed.month.unwrap())
    })
);

named!(
    date_parser<NaiveDate>,
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
    header_parser<Header>,
    do_parse!(
        tag!("HEADER")
            >> multispace1
            >> classification_p: map!(take_str!(40), |s| s.trim())
            >> deposition_date_p: date_parser
            >> multispace1
            >> id_code_p: take_str!(4)
            >> (Header {
                classification: classification_p.to_string(),
                deposition_date: deposition_date_p,
                id_code: id_code_p.to_string()
            })
    )
);

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_header_parser() {
        let head = header_parser(
            "HEADER    PHOTOSYNTHESIS                          28-MAR-07   2UXK".as_bytes(),
        )
        .unwrap()
        .1;
        assert_eq!(head.classification, "PHOTOSYNTHESIS")
    }

    #[test]
    fn test_date_parser() {
        let temp: NaiveDate = date_parser("12-SEP-09".as_bytes()).unwrap().1;
        assert_eq!(temp.day(), 12);
        assert_eq!(temp.year(), 9);
    }

}
