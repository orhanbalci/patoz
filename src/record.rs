use super::entity::*;
use super::primitive::*;
use nom::character::complete::{line_ending, multispace1, space0, space1};
use nom::character::{is_alphanumeric, is_space};
use nom::Err;
use nom::{
    alt, do_parse, fold_many0, map, map_res, named, opt, separated_list, tag, take, take_str,
    take_while,
};

use std::marker::PhantomData;
use std::str;
use std::str::FromStr;

use crate::make_line_folder;
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
            >> line_ending
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
            >> line_ending
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
            >> line_ending
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
            >> line_ending
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
            >> line_ending
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
    cmpnd_line_parser<Continuation<CmpndLine>>,
    do_parse!(
        compnd
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<CmpndLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(cmpnd_line_folder, cmpnd_line_parser, CmpndLine);

named!(
    cmpnd_token_parser<Vec<Token>>,
    map!(
        cmpnd_line_folder,
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
    source_line_parser<Continuation<SourceLine>>,
    do_parse!(
        source
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<SourceLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(source_line_folder, source_line_parser, SourceLine);

named!(
    source_token_parser<Vec<Token>>,
    map!(
        source_line_folder,
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
    keywds_line_parser<Continuation<KeywdsLine>>,
    do_parse!(
        keywds
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<KeywdsLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(keywds_line_folder, keywds_line_parser, KeywdsLine);

named!(
    keywds_parser<Vec<String>>,
    map!(
        keywds_line_folder,
        |v: Vec<u8>| match chain_value_parser(v.as_slice()) {
            Ok((_, res)) => res,
            Err(_err) => Vec::new(),
        }
    )
);

named!(
    experimental_technique_parser<ExperimentalTechnique>,
    alt!(
        do_parse!(
            space0
                >> tag!("X-RAY DIFFRACTION")
                >> space0
                >> (ExperimentalTechnique::XRayDiffraction)
        ) | do_parse!(
            space0
                >> tag!("FIBER DIFFRACTION")
                >> space0
                >> (ExperimentalTechnique::FiberDiffraction)
        ) | do_parse!(
            space0
                >> tag!("NEUTRON DIFFRACTION")
                >> space0
                >> (ExperimentalTechnique::NeutronDiffraction)
        ) | do_parse!(
            space0
                >> tag!("ELECTRON CRYSTALLOGRAPHY")
                >> space0
                >> (ExperimentalTechnique::ElectronCrystallography)
        ) | do_parse!(
            space0
                >> tag!("ELECTRON MICROSCOPY")
                >> space0
                >> (ExperimentalTechnique::ElectronMicroscopy)
        ) | do_parse!(
            space0 >> tag!("SOLID-STATE NMR") >> space0 >> (ExperimentalTechnique::SolidStateNmr)
        ) | do_parse!(
            space0 >> tag!("SOLUTION NMR") >> space0 >> (ExperimentalTechnique::SolutionNmr)
        ) | do_parse!(
            space0
                >> tag!("SOLUTION SCATTERING")
                >> space0
                >> (ExperimentalTechnique::SolutionScattering)
        )
    )
);

named!(
    experimental_technique_list_parser<Vec<ExperimentalTechnique>>,
    separated_list!(tag!(";"), experimental_technique_parser)
);

named!(
    expdata_line_parser<Continuation<ExpdataLine>>,
    do_parse!(
        expdta
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<ExpdataLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(expdata_line_folder, expdata_line_parser, ExpdataLine);

named!(
    expdata_parser<Vec<ExperimentalTechnique>>,
    map!(
        expdata_line_folder,
        |v: Vec<u8>| match experimental_technique_list_parser(v.as_slice()) {
            Ok((_, res)) => res,
            Err(_err) => Vec::new(),
        }
    )
);

named!(
    nummdl_parser<u32>,
    do_parse!(nummdl >> space0 >> model_number: integer >> (model_number))
);

named!(
    author_line_parser<Continuation<AuthorLine>>,
    do_parse!(
        author
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<AuthorLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(author_line_folder, author_line_parser, AuthorLine);

named!(
    author_value_parser<Author>,
    map_res!(
        map_res!(
            map_res!(
                take_while!(|s| {
                    is_alphanumeric(s)
                        || is_space(s)
                        || char::from(s) == '.'
                        || char::from(s) == '\''
                }),
                str::from_utf8
            ),
            str::FromStr::from_str
        ),
        |s: String| {
            println!("{}", s);
            Result::Ok::<Author, Err<String>>(Author(String::from_str(s.trim()).unwrap()))
        }
    )
);

named!(
    author_list_parser<Vec<Author>>,
    separated_list!(tag!(","), author_value_parser)
);

named!(
    author_parser<Vec<Author>>,
    map!(author_line_folder, |v: Vec<u8>| {
        println!("{}", str::from_utf8(&v).unwrap());
        match author_list_parser(v.as_slice()) {
            Ok((_, res)) => res,
            Err(err) => {
                println!("{:?}", err);
                Vec::new()
            }
        }
    })
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
COMPND  12 MUTATION:  NO;"#
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
        if let Ok((_, res)) = cmpnd_line_folder(
            r#"COMPND    MOL_ID:  1;
COMPND   2 MOLECULE:  HEMOGLOBIN ALPHA CHAIN;"#
                .as_bytes(),
        ) {
            assert_eq!(
                str::from_utf8(res.as_slice()).unwrap(),
                "MOL_ID:  1;MOLECULE:  HEMOGLOBIN ALPHA CHAIN;"
            );
        }
    }
}
