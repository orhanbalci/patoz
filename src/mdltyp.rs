use crate::{ast::types::*, primitive::*};

/// Parses continued [MDLTYP](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#MDLTYP) lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    let text = join_continued(lines.iter().map(|l| l.cols(11, 80)));
    Some(Record::Mdltyp(Mdltyp {
        structural_annotation: parse_all(list(';'), &text)?,
    }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn mdltyp() {
        let r = single_record(
            "MDLTYP    CA ATOMS ONLY, CHAIN A, B, C, D, E, F, G, H, I, J, K ; P ATOMS ONLY,
MDLTYP   2 CHAIN X, Y, Z
",
        );
        let Record::Mdltyp(m) = r else { panic!() };
        assert_eq!(
            m.structural_annotation,
            [
                "CA ATOMS ONLY, CHAIN A, B, C, D, E, F, G, H, I, J, K",
                "P ATOMS ONLY, CHAIN X, Y, Z"
            ]
        );
    }
}
