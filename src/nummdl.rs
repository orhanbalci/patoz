use crate::{ast::types::*, primitive::*};

/// Parses a [NUMMDL](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#NUMMDL) record.
pub(crate) fn parse(line: Line) -> Option<Record> {
    Some(Record::Nummdl(Nummdl {
        num: line.int(11, 14)?,
    }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn nummdl() {
        let Record::Nummdl(n) = single_record("NUMMDL    20\n") else {
            panic!()
        };
        assert_eq!(n.num, 20);
    }
}
