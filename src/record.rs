use super::entity::*;
use super::primitive::{
    alphanum_word_with_spaces_inside, atcc, caveat, cell, cell_line, cellular_location, chain,
    chain_value_parser, date_parser, ec, ec_value_parser, engineered, expression_system,
    expression_system_atcc_number, expression_system_cell, expression_system_cell_line,
    expression_system_cellular_location, expression_system_common, expression_system_gene,
    expression_system_organ, expression_system_organelle, expression_system_plasmid,
    expression_system_strain, expression_system_tax_id, expression_system_tissue,
    expression_system_variant, expression_system_vector, expression_system_vector_type, gene,
    header, idcode_list, integer, integer_list, integer_with_spaces, mol_id, molecule, mutation,
    obslte, organ, organelle, organism_common, organism_scientific, organism_tax_id, other_details,
    plasmid, secretion, split, strain, synonym, synthetic, tissue, title, twodigit_integer,
    variant, yes_no_parser,
};
use nom::character::complete::{multispace1, newline, space0, space1};
use nom::{
    alt, do_parse, fold_many0, map, named, opt, separated_list, tag, take, take_str, take_until,
};

use std::str;
use std::str::FromStr;

use crate::make_token_parser;

named!(
    header_parser<Header>,
    do_parse!(
        header
            >> multispace1
            >> classification_p: map!(take_str!(40), str::trim)
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

make_token_parser!(mol_id_parser, mol_id, integer, a, Token::MoleculeId(a));
make_token_parser!(
    molecule_parser,
    molecule,
    alphanum_word_with_spaces_inside,
    a,
    Token::Molecule(a)
);
make_token_parser!(
    chain_parser,
    chain,
    chain_value_parser,
    a,
    Token::Chain { identifiers: a }
);
make_token_parser!(
    synonym_parser,
    synonym,
    chain_value_parser,
    a,
    Token::Synonym { synonyms: a }
);
make_token_parser!(
    ec_parser,
    ec,
    ec_value_parser,
    a,
    Token::Ec {
        commission_numbers: a
    }
);

make_token_parser!(
    engineered_parser,
    engineered,
    yes_no_parser,
    a,
    Token::Engineered(a)
);

make_token_parser!(
    mutation_parser,
    mutation,
    yes_no_parser,
    a,
    Token::Mutation(a)
);

make_token_parser!(
    other_details_parser,
    other_details,
    alphanum_word_with_spaces_inside,
    a,
    Token::OtherDetails(a)
);

make_token_parser!(
    synthetic_parser,
    synthetic,
    alphanum_word_with_spaces_inside,
    a,
    Token::Synthetic(a)
);

make_token_parser!(
    organism_scientific_parser,
    organism_scientific,
    alphanum_word_with_spaces_inside,
    a,
    Token::OrganismScientific(a)
);

make_token_parser!(
    organism_common_parser,
    organism_common,
    chain_value_parser,
    a,
    Token::OrganismCommon { organisms: a }
);

make_token_parser!(
    organism_tax_id_parser,
    organism_tax_id,
    integer_list,
    a,
    Token::OrganismTaxId { id: a }
);

make_token_parser!(
    strain_parser,
    strain,
    alphanum_word_with_spaces_inside,
    a,
    Token::Strain(a)
);

make_token_parser!(
    variant_parser,
    variant,
    alphanum_word_with_spaces_inside,
    a,
    Token::Variant(a)
);

make_token_parser!(
    cell_line_parser,
    cell_line,
    alphanum_word_with_spaces_inside,
    a,
    Token::CellLine(a)
);

make_token_parser!(atcc_parser, atcc, integer_with_spaces, a, Token::Atcc(a));

make_token_parser!(
    organ_parser,
    organ,
    alphanum_word_with_spaces_inside,
    a,
    Token::Organ(a)
);

make_token_parser!(
    tissue_parser,
    tissue,
    alphanum_word_with_spaces_inside,
    a,
    Token::Tissue(a)
);

make_token_parser!(
    cell_parser,
    cell,
    alphanum_word_with_spaces_inside,
    a,
    Token::Cell(a)
);

make_token_parser!(
    organelle_parser,
    organelle,
    alphanum_word_with_spaces_inside,
    a,
    Token::Organelle(a)
);

make_token_parser!(
    secretion_parser,
    secretion,
    alphanum_word_with_spaces_inside,
    a,
    Token::Secretion(a)
);

make_token_parser!(
    cellular_location_parser,
    cellular_location,
    alphanum_word_with_spaces_inside,
    a,
    Token::CellularLocation(a)
);

make_token_parser!(
    plasmid_parser,
    plasmid,
    alphanum_word_with_spaces_inside,
    a,
    Token::Plasmid(a)
);

make_token_parser!(
    gene_parser,
    gene,
    chain_value_parser,
    a,
    Token::Gene { gene: a }
);

make_token_parser!(
    expression_system_parser,
    expression_system,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystem(a)
);

make_token_parser!(
    expression_system_common_parser,
    expression_system_common,
    chain_value_parser,
    a,
    Token::ExpressionSystemCommon { systems: a }
);

make_token_parser!(
    expression_system_tax_id_parser,
    expression_system_tax_id,
    integer_list,
    a,
    Token::ExpressionSystemTaxId { id: a }
);

make_token_parser!(
    expression_system_strain_parser,
    expression_system_strain,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemStrain(a)
);

make_token_parser!(
    expression_system_variant_parser,
    expression_system_variant,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemVariant(a)
);

make_token_parser!(
    expression_system_cell_line_parser,
    expression_system_cell_line,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemCellLine(a)
);

make_token_parser!(
    expression_system_atcc_number_parser,
    expression_system_atcc_number,
    integer_with_spaces,
    a,
    Token::ExpressionSystemAtcc(a)
);

make_token_parser!(
    expression_system_organ_parser,
    expression_system_organ,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemOrgan(a)
);

make_token_parser!(
    expression_system_tissue_parser,
    expression_system_tissue,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemTissue(a)
);

make_token_parser!(
    expression_system_cell_parser,
    expression_system_cell,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemCell(a)
);

make_token_parser!(
    expression_system_organelle_parser,
    expression_system_organelle,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemOrganelle(a)
);

make_token_parser!(
    expression_system_cellular_location_parser,
    expression_system_cellular_location,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemCellularLocation(a)
);

make_token_parser!(
    expression_system_vector_type_parser,
    expression_system_vector_type,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemVectorType(a)
);

make_token_parser!(
    expression_system_vector_parser,
    expression_system_vector,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemVector(a)
);

make_token_parser!(
    expression_system_plasmid_parser,
    expression_system_plasmid,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemPlasmid(a)
);

make_token_parser!(
    expression_system_gene_parser,
    expression_system_gene,
    alphanum_word_with_spaces_inside,
    a,
    Token::ExpressionSystemGene(a)
);

named!(
    token_parser<Token>,
    alt!(
        molecule_parser
            | mol_id_parser
            | chain_parser
            | synonym_parser
            | ec_parser
            | engineered_parser
            | mutation_parser
            | other_details_parser
            | synthetic_parser
            | organism_scientific_parser
            | organism_common_parser
            | organism_tax_id_parser
            | strain_parser
            | variant_parser
            | cell_line_parser
            | atcc_parser
            | organ_parser
            | tissue_parser
            | cell_parser
            | organelle_parser
            | secretion_parser
            | cellular_location_parser
            | plasmid_parser
            | gene_parser
            | expression_system_parser
            | expression_system_common_parser
            | expression_system_tax_id_parser
            | expression_system_strain_parser
            | expression_system_variant_parser
            | expression_system_cell_line_parser
            | expression_system_atcc_number_parser
            | expression_system_organ_parser
            | expression_system_tissue_parser
            | expression_system_cell_parser
            | expression_system_organelle_parser
            | expression_system_cellular_location_parser
            | expression_system_vector_type_parser
            | expression_system_vector_parser
            | expression_system_plasmid_parser
            | expression_system_gene_parser
    )
);

named!(
    tokens_parser<Vec<Token>>,
    separated_list!(tag!(";"), token_parser)
);

named!(
    cmpnd_line_parser<CmpndLine>,
    do_parse!(
        tag!("COMPND")
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: take_until!("\n")
            >> newline
            >> (CmpndLine {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
            })
    )
);

named!(
    cmpnd_parser<Vec<u8>>,
    fold_many0!(
        cmpnd_line_parser,
        Vec::new(),
        |acc: Vec<u8>, item: CmpndLine| {
            //println!("{}", item.remaining);
            acc.into_iter().chain(item.remaining.into_bytes()).collect()
        }
    )
);

named!(
    cmpnd_token_parser<Vec<Token>>,
    map!(
        cmpnd_parser,
        |v: Vec<u8>| match tokens_parser(v.as_slice()) {
            Ok((_, res)) => {
                println!("Okkk {:?}", res);
                res
            }
            Err(err) => {
                println!("Errrr {:?}", err);
                Vec::new()
            }
        }
    )
);

named!(
    source_line_parser<SourceLine>,
    do_parse!(
        tag!("SOURCE")
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: take_until!("\n")
            >> newline
            >> (SourceLine {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
            })
    )
);

named!(
    source_parser<Vec<u8>>,
    fold_many0!(
        source_line_parser,
        Vec::new(),
        |acc: Vec<u8>, item: SourceLine| {
            //println!("{}", item.remaining);
            acc.into_iter().chain(item.remaining.into_bytes()).collect()
        }
    )
);

named!(
    source_token_parser<Vec<Token>>,
    map!(
        source_parser,
        |v: Vec<u8>| match tokens_parser(v.as_slice()) {
            Ok((_, res)) => {
                //println!("Okkk {:?}", res);
                res
            }
            Err(_err) => {
                //println!("Errrr {:?}", err);
                Vec::new()
            }
        }
    )
);

named!(
    keywds_line_parser<KeywdsLine>,
    do_parse!(
        tag!("KEYWDS")
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: take_until!("\n")
            >> newline
            >> (KeywdsLine {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
            })
    )
);

named!(
    keywds_line_folder<Vec<u8>>,
    fold_many0!(
        keywds_line_parser,
        Vec::new(),
        |acc: Vec<u8>, item: KeywdsLine| {
            //println!("{}", item.remaining);
            acc.into_iter().chain(item.remaining.into_bytes()).collect()
        }
    )
);

named!(
    keywds_parser<Vec<String>>,
    map!(
        keywds_line_folder,
        |v: Vec<u8>| match chain_value_parser(v.as_slice()) {
            Ok((_, res)) => {
                println!("Okkk {:?}", res);
                res
            }
            Err(err) => {
                println!("Errrr {:?}", err);
                Vec::new()
            }
        }
    )
);

named!(
    experimental_technique_parser<ExperimentalTechnique>,
    alt!(
        do_parse!(tag!("X-RAY  DIFFRACTION") >> (ExperimentalTechnique::XRayDiffraction))
            | do_parse!(tag!("FIBER  DIFFRACTION") >> (ExperimentalTechnique::FiberDiffraction))
            | do_parse!(
                tag!("NEUTRON  DIFFRACTION") >> (ExperimentalTechnique::NeutronDiffraction)
            )
            | do_parse!(
                tag!("ELECTRON CRYSTALLOGRAPHY")
                    >> (ExperimentalTechnique::ElectronCrystallography)
            )
            | do_parse!(
                tag!("ELECTRON  MICROSCOPY") >> (ExperimentalTechnique::ElectronMicroscopy)
            )
            | do_parse!(tag!("SOLID-STATE  NMR") >> (ExperimentalTechnique::SolidStateNmr))
            | do_parse!(tag!("SOLUTION  NMR") >> (ExperimentalTechnique::SolutionNmr))
            | do_parse!(
                tag!("SOLUTION  SCATTERING") >> (ExperimentalTechnique::SolutionScattering)
            )
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

    #[test]
    fn test_cmpnd_token_parser() {
        if let Ok((_, res)) = cmpnd_token_parser(
            r#"COMPND    MOL_ID:  1;
COMPND   2 MOLECULE:  HEMOGLOBIN ALPHA CHAIN;
COMPND   3 CHAIN: A,  C;
COMPND  10 SYNONYM:  DEOXYHEMOGLOBIN BETA CHAIN;
COMPND   4 EC:  3.2.1.14, 3.2.1.17;
COMPND  11 ENGINEERED: YES;
COMPND  12 MUTATION:  NO
"#
            .as_bytes(),
        ) {
            assert_eq!(res[0], Token::MoleculeId(1));
            assert_eq!(
                res[1],
                Token::Molecule("HEMOGLOBIN ALPHA CHAIN".to_string())
            );
            assert_eq!(
                res[2],
                Token::Chain {
                    identifiers: vec!["A".to_string(), "C".to_string()]
                }
            );
            assert_eq!(res[5], Token::Engineered(true));
        }
    }

    #[test]
    fn test_cmpnd_parser() {
        if let Ok((_, res)) = cmpnd_parser(
            "COMPND    MOL_ID:  1;\nCOMPND   2 MOLECULE:  HEMOGLOBIN ALPHA CHAIN;".as_bytes(),
        ) {
            assert_eq!(
                str::from_utf8(res.as_slice()).unwrap(),
                "MOL_ID:  1;MOLECULE:  HEMOGLOBIN ALPHA CHAIN;"
            );
        }
    }
}
