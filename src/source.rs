use crate::{ast::types::*, compnd::tokens, primitive::*};

/// Parses continued [SOURCE](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SOURCE) lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    Some(Record::Source(Source {
        tokens: tokens(lines, 79)?,
    }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record, Token};

    #[test]
    fn source() {
        let r = single_record(
            "SOURCE    MOL_ID: 1;
SOURCE   2 ORGANISM_SCIENTIFIC: CRAMBE HISPANICA SUBSP ABYSSINICA;
SOURCE   3 ORGANISM_TAXID: 3721;
SOURCE   4 ATCC: CRL-1573;
SOURCE   5 STRAIN: SUBSP ABYSSINICA
",
        );
        let Record::Source(s) = r else { panic!() };
        assert_eq!(
            s.tokens,
            [
                Token::MoleculeId(1),
                Token::OrganismScientific("CRAMBE HISPANICA SUBSP ABYSSINICA".to_owned()),
                Token::OrganismTaxId { id: vec![3721] },
                Token::Other {
                    key: "ATCC".to_owned(),
                    value: "CRL-1573".to_owned()
                },
                Token::Strain("SUBSP ABYSSINICA".to_owned()),
            ]
        );
    }
}
