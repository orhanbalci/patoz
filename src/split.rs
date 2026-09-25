use crate::{ast::types::*, primitive::*};

/// Parses continued [SPLIT](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SPLIT) lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    Some(Record::Split(Split {
        id_codes: lines.iter().flat_map(|l| l.id_codes(12, 14)).collect(),
    }))
}

/// Writes SPLIT lines, fourteen id codes per line.
pub(crate) fn write(split: &Split, out: &mut Vec<String>) {
    for (i, ids) in split.id_codes.chunks(14).enumerate() {
        let mut line = continued("SPLIT", 9, 10, i + 1);
        for (j, id) in ids.iter().enumerate() {
            line = line.left(12 + 5 * j, id);
        }
        out.push(line.build());
    }
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn split() {
        let r = single_record(
            "SPLIT      1VOQ 1VOR 1VOS 1VOU 1VOV 1VOW 1VOX 1VOY 1VP0 1VOZ 1VP1 1VP2 1VP3 1VP4
SPLIT    2 1VP5
",
        );
        let Record::Split(s) = r else { panic!() };
        assert_eq!(s.id_codes.len(), 15);
        assert_eq!(s.id_codes[14], "1VP5");
    }
}
