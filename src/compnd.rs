use super::{ast::types::*, primitive::*};
use nom::{
    alt,
    bytes::complete::tag,
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map,
    multi::separated_list,
    named, opt, IResult,
};

use crate::{make_line_folder, make_token_parser};

use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct CmpndLine;

make_token_parser!(mol_id_parser, mol_id, integer, a, Token::MoleculeId(a));

make_token_parser!(
    molecule_parser,
    molecule,
    molecule_name_parser,
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
    fragment_parser,
    fragment,
    alphanum_word_with_spaces_inside,
    a,
    Token::Fragment(a)
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
    organism_taxid,
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
            | fragment_parser
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

// named!(
//     pub (crate) tokens_parser<Vec<Token>>,
//     separated_list!(tag!(";"), token_parser)
// );

pub(crate) fn tokens_parser(s: &[u8]) -> IResult<&[u8], Vec<Token>> {
    separated_list(tag(";"), token_parser)(s)
}

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
    pub (crate) cmpnd_token_parser<Record>,
    map!(
        cmpnd_line_folder,
        |v: Vec<u8>|  tokens_parser(v.as_slice())
                        .map(|res| Record::Cmpnd(Cmpnd{ tokens : res.1}))
                        .expect("Could not parse tokens")
    )
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mol_id_parser() {
        if let Ok((_, Token::MoleculeId(res))) = super::mol_id_parser("MOL_ID:  1".as_bytes()) {
            assert_eq!(res, 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn molecule_parser() {
        if let Ok((_, Token::Molecule(name))) =
            super::molecule_parser("MOLECULE:  HEMOGLOBIN ALPHA CHAIN\n".as_bytes())
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
    fn test_cmpnd_parser() {
        if let Ok((_, res)) = cmpnd_line_folder(
            r#"COMPND    MOL_ID:  1;
COMPND   2 MOLECULE:  HEMOGLOBIN ALPHA CHAIN;
"#
            .as_bytes(),
        ) {
            assert_eq!(
                str::from_utf8(res.as_slice()).unwrap(),
                "MOL_ID:  1; MOLECULE:  HEMOGLOBIN ALPHA CHAIN;"
            );
        }
    }

    #[test]
    fn test_cmpnd_token_parser() {
        if let Ok((_, Record::Cmpnd(Cmpnd { tokens: res }))) = cmpnd_token_parser(
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
}
