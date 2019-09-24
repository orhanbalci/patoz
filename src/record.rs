use super::entity::*;
use super::primitive::{
    alphanum_word_with_spaces_inside, caveat, chain, chain_value_parser, date_parser, ec,
    ec_value_parser, header, idcode_list, integer, mol_id, molecule, obslte, split, synonym, title,
    twodigit_integer,
};
use nom::character::complete::{multispace1, newline, space0, space1};
use nom::{do_parse, map, named, opt, take, take_str};

named!(
    header_parser<Header>,
    do_parse!(
        header
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
        obslte
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
        title
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
        split
            >> take!(2)
            >> cont: opt!(twodigit_integer)
            >> space1
            >> ids: idcode_list
            >> newline
            >> (Split {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                id_codes: ids
            })
    )
);

named!(
    caveat_parser<Caveat>,
    do_parse!(
        caveat
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

named!(
    mol_id_parser<Token>,
    do_parse!(mol_id >> space0 >> id: integer >> space0 >> (Token::MoleculeId(id)))
);

named!(
    molecule_parser<Token>,
    do_parse!(
        molecule
            >> space1
            >> name: alphanum_word_with_spaces_inside
            >> space0
            >> (Token::Molecule(name))
    )
);

named!(
    chain_parser<Token>,
    do_parse!(
        chain
            >> space1
            >> chain: chain_value_parser
            >> space0
            >> (Token::Chain { identifiers: chain })
    )
);

named!(
    synonym_parser<Token>,
    do_parse!(synonym >> space1 >> syns: chain_value_parser >> (Token::Synonym { synonyms: syns }))
);

named!(
    ec_parser<Token>,
    do_parse!(
        ec >> space1
            >> syns: ec_value_parser
            >> (Token::Ec {
                commission_numbers: syns,
            })
    )
);

#[cfg(test)]
mod test {
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
    fn test_header_parser_2() {
        let head = header_parser(
            "HEADER    TRANSFERASE/TRANSFERASE                 28-MAR-07   2UXK \n".as_bytes(),
        )
        .unwrap()
        .1;
        assert_eq!(head.classification, "TRANSFERASE/TRANSFERASE")
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
            "SPLIT      1VOQ 1VOR 1VOS 1VOU 1VOV 1VOW 1VOX 1VOY 1VP0 1VOZ \n".as_bytes(),
        )
        .unwrap()
        .1;

        assert_eq!(splt.id_codes[0], "1VOQ")
    }

    #[test]
    fn test_mol_id_parser() {
        if let Ok((_, Token::MoleculeId(res))) = mol_id_parser("MOL_ID:  1".as_bytes()) {
            assert_eq!(res, 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_molecule_parser() {
        if let Ok((_, Token::Molecule(name))) =
            molecule_parser("MOLECULE:  HEMOGLOBIN ALPHA CHAIN\n".as_bytes())
        {
            assert_eq!(name, "HEMOGLOBIN ALPHA CHAIN");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_chain_parser() {
        if let Ok((_, Token::Chain { identifiers: res })) = chain_parser("CHAIN: A,  C".as_bytes())
        {
            assert_eq!(res[1], "C")
        }
    }

    #[test]
    fn test_synonym_parser() {
        if let Ok((_, Token::Synonym { synonyms: res })) =
            synonym_parser("SYNONYM: PRECURSOR OF PLEUROTOLYSIN B".as_bytes())
        {
            assert_eq!(res[0], "PRECURSOR OF PLEUROTOLYSIN B");
        }
    }

    #[test]
    fn test_ec_parser() {
        if let Ok((
            _,
            Token::Ec {
                commission_numbers: res,
            },
        )) = ec_parser("EC:  3.2.1.14, 3.2.1.17".as_bytes())
        {
            assert_eq!(res[0], "3.2.1.14")
        }
    }
}
