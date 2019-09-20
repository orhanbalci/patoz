use super::entity::*;
use super::primitive::{
    alphanum_word_with_spaces_inside, date_parser, idcode_list, twodigit_integer,
};
use nom::character::complete::{multispace0, multispace1, newline, space0, space1};
use nom::{do_parse, map, named, opt, tag, take, take_str};

named!(
    header_parser<Header>,
    do_parse!(
        tag!("HEADER")
            >> multispace1
            >> classification_p: map!(take_str!(40), |s| s.trim())
            >> deposition_date_p: date_parser
            >> multispace1
            >> id_code_p: take_str!(4)
            >> space0
            >> newline
            >> (Header {
                classification: classification_p.to_string(),
                deposition_date: deposition_date_p,
                id_code: id_code_p.to_string()
            })
    )
);

named!(
    obslte_parser<Obslte>,
    do_parse!(
        tag!("OBSLTE")
            >> take!(2)
            >> cont: opt!(twodigit_integer)
            >> space0
            >> cont_date: date_parser
            >> space0
            >> ids: idcode_list
            >> newline
            >> (Obslte {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                replacement_date: cont_date,
                replacement_ids: ids
            })
    )
);

named!(
    title_parser<Title>,
    do_parse!(
        tag!("TITLE")
            >> take!(2)
            >> cont: opt!(twodigit_integer)
            >> space1
            >> tit: alphanum_word_with_spaces_inside
            >> space0
            >> newline
            >> (Title {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                title: tit
            })
    )
);

named!(
    split_parser<Split>,
    do_parse!(
        tag!("SPLIT")
            >> take!(2)
            >> cont: opt!(twodigit_integer)
            >> space1
            >> ids: idcode_list
            >> (Split {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                id_codes: ids
            })
    )
);

named!(
    caveat_parser<Caveat>,
    do_parse!(
        tag!("CAVEAT")
            >> take!(2)
            >> cont: opt!(twodigit_integer)
            >> space1
            >> com: take_str!(59)
            >> newline
            >> (Caveat {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                comment: String::from(com),
            })
    )
);

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_header_parser() {
        let head = header_parser(
            "HEADER    PHOTOSYNTHESIS                          28-MAR-07   2UXK \n".as_bytes(),
        )
        .unwrap()
        .1;
        assert_eq!(head.classification, "PHOTOSYNTHESIS")
    }

    #[test]
    fn test_obslte_parser() {
        let obs = obslte_parser("OBSLTE  02 31-JAN-94 1MBP      2MBP    \n".as_bytes())
            .unwrap()
            .1;
        assert_eq!(obs.replacement_ids[0], "1MBP");
    }

    #[test]
    fn test_title_parser() {
        let tit = title_parser(
            "TITLE     RHIZOPUSPEPSIN COMPLEXED WITH REDUCED PEPTIDE INHIBITOR\n".as_bytes(),
        )
        .unwrap()
        .1;

        assert_eq!(
            tit.title,
            "RHIZOPUSPEPSIN COMPLEXED WITH REDUCED PEPTIDE INHIBITOR"
        );
    }

    #[test]
    fn test_split_parser() {
        let splt = split_parser(
            "SPLIT      1VOQ 1VOR 1VOS 1VOU 1VOV 1VOW 1VOX 1VOY 1VP0 1VOZ ;".as_bytes(),
        )
        .unwrap()
        .1;

        assert_eq!(splt.id_codes[0], "1VOQ")
    }

}
