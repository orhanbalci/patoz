/*!
Building blocks shared by record parsers: fixed column access to lines and
nom parsers for values found inside records.
*/
use crate::ResidueRef;
use chrono::NaiveDate;
use nom::{
    bytes::complete::{take_till, take_while1},
    character::complete::{alpha1, char, space0, u32 as uint},
    combinator::{all_consuming, map_opt, opt},
    error::Error,
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use std::str::FromStr;

/// A line of a pdb file addressed by the 1-based inclusive column numbers
/// used in the format specification. Columns past the end of a short line
/// read as blank.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Line<'a>(pub &'a str);

impl<'a> Line<'a> {
    /// raw text of columns `from..=to`
    pub fn cols(&self, from: usize, to: usize) -> &'a str {
        let start = (from - 1).min(self.0.len());
        let end = to.min(self.0.len()).max(start);
        self.0.get(start..end).unwrap_or("")
    }

    /// text of columns `from..=to` without surrounding whitespace
    pub fn text(&self, from: usize, to: usize) -> &'a str {
        self.cols(from, to).trim()
    }

    /// record name in columns 1-6
    pub fn record_name(&self) -> &'a str {
        self.text(1, 6)
    }

    /// character at `col`, `None` if blank
    pub fn char_at(&self, col: usize) -> Option<char> {
        self.cols(col, col).chars().next().filter(|c| *c != ' ')
    }

    /// number in columns `from..=to`, `None` if blank or not a number
    pub fn number<T: FromStr>(&self, from: usize, to: usize) -> Option<T> {
        self.text(from, to).parse().ok()
    }

    /// residue whose name starts at column `name`, followed by chain id at
    /// `chain`, sequence number in `seq..=seq + 3` and insertion code
    pub fn residue(&self, name: usize, chain: usize, seq: usize) -> Option<ResidueRef> {
        Some(ResidueRef {
            residue_name: self.text(name, name + 2).to_owned(),
            chain_id: self.char_at(chain).unwrap_or(' '),
            residue_seq: self.number(seq, seq + 3)?,
            insertion_code: self.char_at(seq + 4),
        })
    }

    /// four character id codes starting at column `from`, 5 columns apart
    pub fn id_codes(&self, from: usize, count: usize) -> impl Iterator<Item = String> + 'a {
        let line = *self;
        (0..count)
            .map(move |i| line.text(from + 5 * i, from + 5 * i + 3))
            .filter(|id| !id.is_empty())
            .map(str::to_owned)
    }
}

/// Joins the text of continued lines with a space. A line ending with a
/// hyphen continues its word on the next line so no space is added.
pub(crate) fn join_continued<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    let mut joined = String::new();
    for part in parts.into_iter().map(str::trim).filter(|p| !p.is_empty()) {
        if !joined.is_empty() && !joined.ends_with('-') {
            joined.push(' ');
        }
        joined.push_str(part);
    }
    joined
}

/// Joins continued chemical names (HETNAM, HETSYN). wwPDB wraps these
/// either at a space or right after punctuation such as `-` or `)`, so a
/// space is restored only between two letters or digits.
pub(crate) fn join_chemical_name<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    let mut joined = String::new();
    for part in parts.into_iter().map(str::trim).filter(|p| !p.is_empty()) {
        let boundary_is_space = joined.ends_with(|c: char| c.is_ascii_alphanumeric())
            && part.starts_with(|c: char| c.is_ascii_alphanumeric());
        if boundary_is_space {
            joined.push(' ');
        }
        joined.push_str(part);
    }
    joined
}

/// Runs `parser` on the whole of `input`, `None` if it fails or leaves
/// anything unparsed.
pub(crate) fn parse_all<'a, O>(
    parser: impl Parser<&'a str, Output = O, Error = Error<&'a str>>,
    input: &'a str,
) -> Option<O> {
    all_consuming(parser).parse(input).ok().map(|(_, o)| o)
}

/// PDB dates carry two digit years. PDB archive started in 1971 so years
/// from 71 onwards belong to 20th century, earlier ones to 21st century.
fn four_digit_year(year: u32) -> i32 {
    match year {
        0..=70 => 2000 + year as i32,
        71..=99 => 1900 + year as i32,
        _ => year as i32,
    }
}

fn month(s: &str) -> IResult<&str, u32> {
    const MONTHS: [&str; 12] = [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    map_opt(alpha1, |m: &str| {
        MONTHS.iter().position(|x| *x == m).map(|i| i as u32 + 1)
    })
    .parse(s)
}

/// Parses dates of the form `DD-MMM-YY`.
pub(crate) fn date(s: &str) -> IResult<&str, NaiveDate> {
    map_opt(
        (uint, char('-'), month, char('-'), uint),
        |(day, _, month, _, year)| NaiveDate::from_ymd_opt(four_digit_year(year), month, day),
    )
    .parse(s)
}

/// Parses a list of items separated by `separator`, trimming whitespace
/// around items and dropping empty ones.
pub(crate) fn list<'a>(
    separator: char,
) -> impl Parser<&'a str, Output = Vec<String>, Error = Error<&'a str>> {
    separated_list1(char(separator), take_till(move |c| c == separator)).map(|items: Vec<&str>| {
        items
            .into_iter()
            .map(str::trim)
            .filter(|i| !i.is_empty())
            .map(str::to_owned)
            .collect()
    })
}

fn specification_key(s: &str) -> IResult<&str, &str> {
    delimited(
        space0,
        take_while1(|c: char| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'),
        space0,
    )
    .parse(s)
}

/// Value of a specification list token. Values may contain `;` themselves
/// (e.g. `SYNONYM: PIMT; PROTEIN L-ISOASPARTATE`), so a `;` only ends the
/// value when a `KEY:` or the end of input follows it.
fn specification_value(s: &str) -> IResult<&str, &str> {
    let mut end = 0;
    while let Some(i) = s[end..].find(';') {
        let after = &s[end + i + 1..];
        if after.trim().is_empty() || (specification_key, char(':')).parse(after).is_ok() {
            return Ok((&s[end + i..], s[..end + i].trim()));
        }
        end += i + 1;
    }
    Ok(("", s.trim()))
}

/// Parses `KEY: value; KEY: value` specification lists used by COMPND and
/// SOURCE records into key value pairs.
pub(crate) fn specification_list(s: &str) -> IResult<&str, Vec<(&str, &str)>> {
    terminated(
        separated_list1(
            char(';'),
            separated_pair(specification_key, char(':'), specification_value),
        ),
        (opt(char(';')), space0),
    )
    .parse(s)
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn columns_of_short_line() {
        let line = Line("HEADER    PLANT");
        assert_eq!(line.record_name(), "HEADER");
        assert_eq!(line.text(11, 50), "PLANT");
        assert_eq!(line.cols(60, 66), "");
        assert_eq!(line.char_at(7), None);
        assert_eq!(line.number::<u32>(11, 15), None);
    }

    #[test]
    fn id_codes() {
        let line = Line("SPLIT      1VOQ 1VOR 1VOS");
        assert_eq!(
            line.id_codes(12, 14).collect::<Vec<_>>(),
            ["1VOQ", "1VOR", "1VOS"]
        );
    }

    #[test]
    fn join_continued_lines() {
        assert_eq!(join_continued(["A B ", " C", ""]), "A B C");
        assert_eq!(join_continued(["N-(3-", "DIMETHOXY"]), "N-(3-DIMETHOXY");
    }

    #[test]
    fn join_chemical_name_parts() {
        assert_eq!(
            join_chemical_name(["ADENINE-DINUCLEOTIDE", "PHOSPHATE"]),
            "ADENINE-DINUCLEOTIDE PHOSPHATE"
        );
        assert_eq!(
            join_chemical_name(["(3-AMINOMETHYL)", "PHENYL"]),
            "(3-AMINOMETHYL)PHENYL"
        );
        assert_eq!(join_chemical_name(["5,5-", "DIMETHYL"]), "5,5-DIMETHYL");
    }

    #[test]
    fn test_date_parser() {
        let temp = parse_all(date, "12-SEP-09").unwrap();
        assert_eq!(temp.day(), 12);
        assert_eq!(temp.year(), 2009);
    }

    #[test]
    fn test_date_parser_twentieth_century() {
        let temp = parse_all(date, "15-OCT-98").unwrap();
        assert_eq!(temp, NaiveDate::from_ymd_opt(1998, 10, 15).unwrap());
    }

    #[test]
    fn test_date_parser_invalid_date() {
        assert!(parse_all(date, "31-FEB-99").is_none());
    }

    #[test]
    fn test_date_parser_invalid_month() {
        assert!(parse_all(date, "12-XYZ-09").is_none());
    }

    #[test]
    fn test_list() {
        assert_eq!(parse_all(list(','), "A,  C").unwrap(), ["A", "C"]);
        assert_eq!(
            parse_all(list(','), "D.VAN DER HELM, J.DOE,").unwrap(),
            ["D.VAN DER HELM", "J.DOE"]
        );
    }

    #[test]
    fn test_specification_list() {
        let tokens =
            parse_all(specification_list, "MOL_ID:  1; OTHER_DETAILS: RATIO 1:1;").unwrap();
        assert_eq!(tokens, [("MOL_ID", "1"), ("OTHER_DETAILS", "RATIO 1:1")]);
    }

    #[test]
    fn specification_value_with_semicolon() {
        let tokens =
            parse_all(specification_list, "SYNONYM: PIMT; PROTEIN L-ISO; EC: 2.1").unwrap();
        assert_eq!(tokens, [("SYNONYM", "PIMT; PROTEIN L-ISO"), ("EC", "2.1")]);
    }

    #[test]
    fn specification_list_without_key_fails() {
        assert!(parse_all(specification_list, "NOT A TOKEN").is_none());
    }
}
