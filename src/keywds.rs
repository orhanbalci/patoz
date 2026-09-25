use crate::{ast::types::*, primitive::*};

/// Parses continued [KEYWDS](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#KEYWDS) lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    let text = join_continued(lines.iter().map(|l| l.cols(11, 79)));
    Some(Record::Keywds(Keywds {
        keywords: parse_all(list(','), &text)?,
    }))
}

/// Writes KEYWDS lines.
pub(crate) fn write(keywds: &Keywds, out: &mut Vec<String>) {
    let text = keywds.keywords.join(", ");
    write_wrapped(out, &text, 11, 79, true, Wrap::list(Join::Text, ','), |n| {
        continued("KEYWDS", 9, 10, n)
    });
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn keywds() {
        let r = single_record(
            "KEYWDS    VALENCE ELECTRON DENSITY, MULTI-SUBSTATE, MULTIPOLE REFINEMENT, PLANT
KEYWDS   2 PROTEIN
",
        );
        let Record::Keywds(k) = r else { panic!() };
        assert_eq!(k.keywords.len(), 4);
        assert_eq!(k.keywords[3], "PLANT PROTEIN");
    }
}
