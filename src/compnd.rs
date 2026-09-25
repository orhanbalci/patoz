/*!
Parses [COMPND](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#COMPND) records and the specification list
tokens shared with SOURCE records.
*/
use crate::{ast::types::*, primitive::*};

fn yes_no(value: &str) -> Option<bool> {
    match value {
        "YES" => Some(true),
        "NO" => Some(false),
        _ => None,
    }
}

fn int_list(value: &str) -> Option<Vec<u32>> {
    parse_all(list(','), value)?
        .iter()
        .map(|i| i.parse().ok())
        .collect()
}

/// Converts a specification list key value pair to a [Token]. Unknown keys
/// and values that do not match the key's type become [Token::Other].
pub(crate) fn token(key: &str, value: &str) -> Token {
    let text = || Some(value.to_owned());
    let items = || parse_all(list(','), value);
    let token = match key {
        "MOL_ID" => value.parse().ok().map(Token::MoleculeId),
        "MOLECULE" => text().map(Token::Molecule),
        "CHAIN" => items().map(|identifiers| Token::Chain { identifiers }),
        "FRAGMENT" => text().map(Token::Fragment),
        "SYNONYM" => items().map(|synonyms| Token::Synonym { synonyms }),
        "EC" => items().map(|commission_numbers| Token::Ec { commission_numbers }),
        "ENGINEERED" => yes_no(value).map(Token::Engineered),
        "MUTATION" => yes_no(value).map(Token::Mutation),
        "OTHER_DETAILS" => text().map(Token::OtherDetails),
        "SYNTHETIC" => text().map(Token::Synthetic),
        "ORGANISM_SCIENTIFIC" => text().map(Token::OrganismScientific),
        "ORGANISM_COMMON" => items().map(|organisms| Token::OrganismCommon { organisms }),
        "ORGANISM_TAXID" => int_list(value).map(|id| Token::OrganismTaxId { id }),
        "STRAIN" => text().map(Token::Strain),
        "VARIANT" => text().map(Token::Variant),
        "CELL_LINE" => text().map(Token::CellLine),
        "ATCC" => value.parse().ok().map(Token::Atcc),
        "ORGAN" => text().map(Token::Organ),
        "TISSUE" => text().map(Token::Tissue),
        "CELL" => text().map(Token::Cell),
        "ORGANELLE" => text().map(Token::Organelle),
        "SECRETION" => text().map(Token::Secretion),
        "CELLULAR_LOCATION" => text().map(Token::CellularLocation),
        "PLASMID" => text().map(Token::Plasmid),
        "GENE" => items().map(|gene| Token::Gene { gene }),
        "EXPRESSION_SYSTEM" => text().map(Token::ExpressionSystem),
        "EXPRESSION_SYSTEM_COMMON" => {
            items().map(|systems| Token::ExpressionSystemCommon { systems })
        }
        "EXPRESSION_SYSTEM_TAXID" => int_list(value).map(|id| Token::ExpressionSystemTaxId { id }),
        "EXPRESSION_SYSTEM_STRAIN" => text().map(Token::ExpressionSystemStrain),
        "EXPRESSION_SYSTEM_VARIANT" => text().map(Token::ExpressionSystemVariant),
        "EXPRESSION_SYSTEM_CELL_LINE" => text().map(Token::ExpressionSystemCellLine),
        "EXPRESSION_SYSTEM_ATCC_NUMBER" => value.parse().ok().map(Token::ExpressionSystemAtcc),
        "EXPRESSION_SYSTEM_ORGAN" => text().map(Token::ExpressionSystemOrgan),
        "EXPRESSION_SYSTEM_TISSUE" => text().map(Token::ExpressionSystemTissue),
        "EXPRESSION_SYSTEM_CELL" => text().map(Token::ExpressionSystemCell),
        "EXPRESSION_SYSTEM_ORGANELLE" => text().map(Token::ExpressionSystemOrganelle),
        "EXPRESSION_SYSTEM_CELLULAR_LOCATION" => {
            text().map(Token::ExpressionSystemCellularLocation)
        }
        "EXPRESSION_SYSTEM_VECTOR_TYPE" => text().map(Token::ExpressionSystemVectorType),
        "EXPRESSION_SYSTEM_VECTOR" => text().map(Token::ExpressionSystemVector),
        "EXPRESSION_SYSTEM_PLASMID" => text().map(Token::ExpressionSystemPlasmid),
        "EXPRESSION_SYSTEM_GENE" => text().map(Token::ExpressionSystemGene),
        _ => None,
    };
    token.unwrap_or_else(|| Token::Other {
        key: key.to_owned(),
        value: value.to_owned(),
    })
}

/// Parses the specification list of continued COMPND or SOURCE lines.
pub(crate) fn tokens(lines: &[Line], last_col: usize) -> Option<Vec<Token>> {
    let text = join_continued(lines.iter().map(|l| l.cols(11, last_col)));
    let pairs = parse_all(specification_list, &text)?;
    Some(pairs.into_iter().map(|(k, v)| token(k, v)).collect())
}

/// Parses continued COMPND lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    Some(Record::Cmpnd(Cmpnd {
        tokens: tokens(lines, 80)?,
    }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record, Token};

    #[test]
    fn compnd() {
        let r = single_record(
            "COMPND    MOL_ID:  1;
COMPND   2 MOLECULE:  HEMOGLOBIN ALPHA CHAIN;
COMPND   3 CHAIN: A,  C;
COMPND   4 SYNONYM:  DEOXYHEMOGLOBIN BETA CHAIN;
COMPND   5 EC:  3.2.1.14, 3.2.1.17;
COMPND   6 ENGINEERED: YES;
COMPND   7 MUTATION:  NO;
COMPND   8 OTHER_DETAILS: RATIO 1:1
",
        );
        let Record::Cmpnd(c) = r else { panic!() };
        assert_eq!(
            c.tokens,
            [
                Token::MoleculeId(1),
                Token::Molecule("HEMOGLOBIN ALPHA CHAIN".to_owned()),
                Token::Chain {
                    identifiers: vec!["A".to_owned(), "C".to_owned()]
                },
                Token::Synonym {
                    synonyms: vec!["DEOXYHEMOGLOBIN BETA CHAIN".to_owned()]
                },
                Token::Ec {
                    commission_numbers: vec!["3.2.1.14".to_owned(), "3.2.1.17".to_owned()]
                },
                Token::Engineered(true),
                Token::Mutation(false),
                Token::OtherDetails("RATIO 1:1".to_owned()),
            ]
        );
    }

    #[test]
    fn value_split_across_lines() {
        let r = single_record(
            "COMPND    MOL_ID: 1;
COMPND   2 SYNONYM: PROTEIN-BETA-ASPARTATE METHYLTRANSFERASE; PIMT; PROTEIN L-
COMPND   3 ISOASPARTATE;
",
        );
        let Record::Cmpnd(c) = r else { panic!() };
        assert_eq!(c.tokens.len(), 2);
    }
}
