use crate::{ast::types::*, primitive::*};

/// Parses continued [CAVEAT](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#CAVEAT) lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    Some(Record::Caveat(Caveat {
        id_code: lines[0].text(12, 15).to_owned(),
        comment: join_continued(lines.iter().map(|l| l.cols(20, 79))),
    }))
}

/// Writes CAVEAT lines.
pub(crate) fn write(caveat: &Caveat, out: &mut Vec<String>) {
    write_wrapped(out, &caveat.comment, 20, 79, false, Wrap::TEXT, |n| {
        continued("CAVEAT", 9, 10, n).left(12, &caveat.id_code)
    });
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn caveat() {
        let r = single_record(
            "CAVEAT     1ABC    INCORRECT CHIRALITY AT
CAVEAT   2 1ABC    RESIDUE 12
",
        );
        let Record::Caveat(c) = r else { panic!() };
        assert_eq!(c.id_code, "1ABC");
        assert_eq!(c.comment, "INCORRECT CHIRALITY AT RESIDUE 12");
    }
}
