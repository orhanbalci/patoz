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

/// How the text of continued lines is joined back together. The writer
/// wraps text only where the same rule restores it, see [wrap].
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Join {
    /// A space between lines, except after a line ending with a hyphen,
    /// which continues its word on the next line.
    Text,
    /// Chemical names (HETNAM, HETSYN). wwPDB wraps these either at a space
    /// or right after punctuation such as `-` or `)`, so a space is
    /// restored only between two letters or digits.
    Chemical,
    /// Always a space, for text such as formulas where a line may end with
    /// a charge like `2-`.
    Word,
}

impl Join {
    fn separator(self, left: &str, right: &str) -> &'static str {
        let alphanumeric = |c: Option<char>| c.is_some_and(|c| c.is_ascii_alphanumeric());
        let space = match self {
            Join::Text => !left.ends_with('-'),
            Join::Chemical => {
                alphanumeric(left.chars().last()) && alphanumeric(right.chars().next())
            }
            Join::Word => true,
        };
        if space {
            " "
        } else {
            ""
        }
    }

    /// Whether a line may end right after `c` without a space being lost.
    /// wwPDB breaks chemical names only after these characters.
    fn breaks_after(self, c: char) -> bool {
        match self {
            Join::Chemical => matches!(c, '-' | ')' | ',' | ';'),
            Join::Text | Join::Word => true,
        }
    }

    /// Joins trimmed parts, skipping empty ones.
    pub fn join<'a>(self, parts: impl IntoIterator<Item = &'a str>) -> String {
        let mut joined = String::new();
        for part in parts.into_iter().map(str::trim).filter(|p| !p.is_empty()) {
            if !joined.is_empty() {
                joined.push_str(self.separator(&joined, part));
            }
            joined.push_str(part);
        }
        joined
    }
}

/// Joins the text of continued lines, see [Join::Text].
pub(crate) fn join_continued<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    Join::Text.join(parts)
}

/// Joins continued chemical names, see [Join::Chemical].
pub(crate) fn join_chemical_name<'a>(parts: impl IntoIterator<Item = &'a str>) -> String {
    Join::Chemical.join(parts)
}

/// How text is wrapped over continued lines.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Wrap {
    /// how the parser joins the lines back
    pub join: Join,
    /// separator of list items; whitespace around items is not significant
    /// so lines may also break after a separator
    pub list_separator: Option<char>,
    /// Keep words, or list items, whole: split one after a hyphen or
    /// similar only when it does not fit on a line of its own. Otherwise
    /// each line is filled as far as possible. wwPDB fills JRNL lines this
    /// way, and other records greedily.
    pub whole_units: bool,
}

impl Wrap {
    pub const TEXT: Wrap = Wrap {
        join: Join::Text,
        list_separator: None,
        whole_units: false,
    };

    pub const fn list(join: Join, separator: char) -> Wrap {
        Wrap {
            join,
            list_separator: Some(separator),
            whole_units: false,
        }
    }

    pub const fn whole_units(self) -> Wrap {
        Wrap {
            whole_units: true,
            ..self
        }
    }
}

/// Splits `text` into pieces, the first at most `first_width` and the rest
/// at most `width` characters long, such that the join rule of `wrapping`
/// gives back `text`. Text without any break point is cut at the width.
pub(crate) fn wrap(text: &str, first_width: usize, width: usize, wrapping: Wrap) -> Vec<String> {
    let Wrap {
        join,
        list_separator,
        whole_units,
    } = wrapping;
    let chars: Vec<char> = text.trim().chars().collect();
    // a break ending the line after `end` characters: the line length, the
    // start of the next line and whether it is a preferred break
    let break_at = |rest: &[char], end: usize| -> Option<(usize, usize, bool)> {
        let left_len = rest[..end]
            .iter()
            .rposition(|c| *c != ' ')
            .map_or(0, |i| i + 1);
        let right_start = end + rest[end..].iter().take_while(|c| **c == ' ').count();
        if left_len == 0 || right_start == rest.len() {
            return None;
        }
        let left: String = rest[..left_len].iter().collect();
        let right: String = rest[right_start..].iter().take(1).collect();
        let removed = right_start - left_len;
        let after_separator = list_separator.is_some_and(|s| left.ends_with(s));
        let restores = removed == join.separator(&left, &right).len()
            && (removed > 0 || join.breaks_after(left.chars().last().unwrap()));
        if after_separator && removed <= 1 {
            Some((left_len, right_start, true))
        } else if restores {
            Some((
                left_len,
                right_start,
                list_separator.is_none() && removed > 0,
            ))
        } else {
            None
        }
    };
    let mut pieces = Vec::new();
    let mut rest = &chars[..];
    let mut max = first_width;
    while rest.len() > max {
        let breaks: Vec<_> = (1..=max).filter_map(|end| break_at(rest, end)).collect();
        let last_preferred = breaks.iter().rev().find(|b| b.2);
        let last_preferred = last_preferred.filter(|_| whole_units);
        let cut = match last_preferred {
            Some(&(len, start, _)) => {
                // the unit after a preferred break goes to the next line whole
                // if it fits there
                let next_unit = (start + 1..rest.len())
                    .find(|&end| break_at(rest, end).is_some_and(|b| b.2 && b.0 > start))
                    .unwrap_or(rest.len())
                    - start;
                if next_unit <= width {
                    (len, start)
                } else {
                    breaks.last().map_or((len, start), |b| (b.0, b.1))
                }
            }
            None => breaks.last().map_or((max, max), |b| (b.0, b.1)),
        };
        pieces.push(rest[..cut.0].iter().collect());
        rest = &rest[cut.1..];
        max = width;
    }
    if !rest.is_empty() || pieces.is_empty() {
        pieces.push(rest.iter().collect());
    }
    pieces
}

/// Builds a line by placing values at the 1-based columns of the
/// specification. Built lines are padded to 80 columns like wwPDB files.
pub(crate) struct LineBuilder(Vec<char>);

impl LineBuilder {
    pub fn new(record_name: &str) -> Self {
        LineBuilder(Vec::with_capacity(80)).left(1, record_name)
    }

    /// writes `text` starting at column `from`
    pub fn left(mut self, from: usize, text: &str) -> Self {
        let start = from - 1;
        if self.0.len() < start {
            self.0.resize(start, ' ');
        }
        for (i, c) in text.chars().enumerate() {
            match self.0.get_mut(start + i) {
                Some(slot) => *slot = c,
                None => self.0.push(c),
            }
        }
        self
    }

    /// writes `value` right aligned in columns `from..=to`
    pub fn right(self, from: usize, to: usize, value: impl std::fmt::Display) -> Self {
        let text = value.to_string();
        let start = (to + 1).saturating_sub(text.chars().count()).max(from);
        self.left(start, &text)
    }

    /// writes `value` right aligned in columns `from..=to` if present
    pub fn right_opt(self, from: usize, to: usize, value: Option<impl std::fmt::Display>) -> Self {
        match value {
            Some(v) => self.right(from, to, v),
            None => self,
        }
    }

    /// writes a character at `col`, blank if `None`
    pub fn char_at(self, col: usize, c: Option<char>) -> Self {
        self.left(col, &c.unwrap_or(' ').to_string())
    }

    /// writes a residue in the layout read by [Line::residue]
    pub fn residue(self, name: usize, chain: usize, seq: usize, residue: &ResidueRef) -> Self {
        self.right(name, name + 2, &residue.residue_name)
            .char_at(chain, Some(residue.chain_id))
            .right(seq, seq + 3, residue.residue_seq)
            .char_at(seq + 4, residue.insertion_code)
    }

    /// writes an atom name in the four columns starting at `col`. Names
    /// start one column later unless they fill all four columns or start
    /// with a two letter element such as `FE`.
    pub fn atom_name(self, col: usize, name: &str, element: Option<&str>) -> Self {
        let two_letter_element = match element {
            Some(e) => e.len() == 2,
            None => TWO_LETTER_ELEMENTS.iter().any(|e| name.starts_with(e)) && name.len() <= 2,
        };
        if name.len() >= 4 || two_letter_element {
            self.left(col, name)
        } else {
            self.left(col + 1, name)
        }
    }

    pub fn build(mut self) -> String {
        if self.0.len() < 80 {
            self.0.resize(80, ' ');
        }
        self.0.into_iter().collect()
    }
}

/// Elements whose symbol is written from the first column of an atom name
/// when the element column is not available, as in LINK records.
const TWO_LETTER_ELEMENTS: [&str; 12] = [
    "FE", "ZN", "MG", "MN", "CU", "CO", "NI", "CL", "BR", "NA", "CD", "HG",
];

/// Writes `text` over as many lines as needed. `line(n)` builds the start
/// of line `n` (1-based) and the text is placed at `text_col` up to
/// `last_col`; continuation lines leave column `text_col` blank if
/// `indent` is set, as TITLE and similar records do.
pub(crate) fn write_wrapped(
    out: &mut Vec<String>,
    text: &str,
    text_col: usize,
    last_col: usize,
    indent: bool,
    wrapping: Wrap,
    line: impl Fn(usize) -> LineBuilder,
) {
    let width = last_col + 1 - text_col;
    let rest_width = if indent { width - 1 } else { width };
    for (i, piece) in wrap(text, width, rest_width, wrapping).iter().enumerate() {
        let col = if i > 0 && indent {
            text_col + 1
        } else {
            text_col
        };
        out.push(line(i + 1).left(col, piece).build());
    }
}

/// Starts line `n` of a continued record, writing the continuation number
/// right aligned in columns `from..=to` from the second line on.
pub(crate) fn continued(name: &str, from: usize, to: usize, n: usize) -> LineBuilder {
    let line = LineBuilder::new(name);
    if n > 1 {
        line.right(from, to, n)
    } else {
        line
    }
}

/// Formats a date as `DD-MMM-YY`.
pub(crate) fn format_date(date: NaiveDate) -> String {
    date.format("%d-%b-%y").to_string().to_uppercase()
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
    fn wrap_restores_text() {
        let title = "CRAMBIN AT ULTRA-HIGH RESOLUTION: VALENCE ELECTRON DENSITY";
        // 11 is the longest word, "RESOLUTION:"
        for width in 11..40 {
            let pieces = wrap(title, width, width, Wrap::TEXT);
            assert!(pieces.iter().all(|p| p.chars().count() <= width));
            assert_eq!(Join::Text.join(pieces.iter().map(String::as_str)), title);
        }
        let name = "METHYL CYCLO[(2S)-2-[[(1R)-1-(N-(L-N-(3-METHYLBUTANOYL)VALYL)PHENYLPROPANOATE";
        // 16 is the longest unbreakable piece, "PHENYLPROPANOATE"
        for width in 16..40 {
            let pieces = wrap(
                name,
                width,
                width,
                Wrap {
                    join: Join::Chemical,
                    ..Wrap::TEXT
                },
            );
            assert_eq!(Join::Chemical.join(pieces.iter().map(String::as_str)), name);
        }
    }

    #[test]
    fn wrap_keeps_short_hyphenated_words_whole() {
        let title = "CRYSTAL STRUCTURES OF MYOGLOBIN-LIGAND COMPLEXES AT NEAR-ATOMIC RESOLUTION.";
        assert_eq!(
            wrap(title, 60, 60, Wrap::TEXT.whole_units()),
            [
                "CRYSTAL STRUCTURES OF MYOGLOBIN-LIGAND COMPLEXES AT",
                "NEAR-ATOMIC RESOLUTION."
            ]
        );
    }

    #[test]
    fn wrap_list_keeps_items_whole() {
        let authors = "K.S.WILSON,J.J.VAN BEEUMEN,S.CIURLI";
        assert_eq!(
            wrap(authors, 20, 20, Wrap::list(Join::Text, ',').whole_units()),
            ["K.S.WILSON,", "J.J.VAN BEEUMEN,", "S.CIURLI"]
        );
    }

    #[test]
    fn wrap_list_after_separator() {
        let pieces = wrap(
            "C.JELSCH,M.M.TEETER,V.LAMZIN",
            20,
            20,
            Wrap::list(Join::Text, ','),
        );
        assert_eq!(pieces, ["C.JELSCH,M.M.TEETER,", "V.LAMZIN"]);
    }

    #[test]
    fn line_builder() {
        let line = LineBuilder::new("SEQRES")
            .right(8, 10, 1)
            .char_at(12, Some('A'))
            .right(14, 17, 224)
            .build();
        assert_eq!(line.len(), 80);
        assert_eq!(line.trim_end(), "SEQRES   1 A  224");
    }

    #[test]
    fn atom_name_alignment() {
        let name = |n: &str, e: Option<&str>| {
            LineBuilder::new("").atom_name(13, n, e).build()[12..16].to_owned()
        };
        assert_eq!(name("CA", Some("C")), " CA ");
        assert_eq!(name("FE", Some("FE")), "FE  ");
        assert_eq!(name("HG21", Some("H")), "HG21");
        assert_eq!(name("FE", None), "FE  ");
        assert_eq!(name("NE2", None), " NE2");
    }

    #[test]
    fn test_date_parser() {
        let temp = parse_all(date, "12-SEP-09").unwrap();
        assert_eq!(temp.day(), 12);
        assert_eq!(temp.year(), 2009);
    }

    #[test]
    fn date_format() {
        let date = parse_all(date, "15-OCT-98").unwrap();
        assert_eq!(format_date(date), "15-OCT-98");
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
