use crate::{ast::types::*, primitive::*};

/// Parses continued [EXPDTA](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#EXPDTA) lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    let text = join_continued(lines.iter().map(|l| l.cols(11, 79)));
    let techniques = parse_all(list(';'), &text)?
        .iter()
        .map(|t| t.parse().ok())
        .collect::<Option<_>>()?;
    Some(Record::Experimental(Experimental { techniques }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, ExperimentalTechnique, Record};

    #[test]
    fn expdta() {
        let r = single_record("EXPDTA    NEUTRON DIFFRACTION; X-RAY DIFFRACTION\n");
        let Record::Experimental(e) = r else { panic!() };
        assert_eq!(
            e.techniques,
            [
                ExperimentalTechnique::NeutronDiffraction,
                ExperimentalTechnique::XRayDiffraction
            ]
        );
    }
}
