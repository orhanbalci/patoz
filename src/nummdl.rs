use crate::{ast::types::*, primitive::*};

/// Parses a [NUMMDL](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#NUMMDL) record.
pub(crate) fn parse(line: Line) -> Option<Record> {
    Some(Record::Nummdl(Nummdl {
        num: line.number(11, 14)?,
    }))
}

/// Writes a NUMMDL record.
pub(crate) fn write(nummdl: &Nummdl, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new("NUMMDL")
            .left(11, &nummdl.num.to_string())
            .build(),
    );
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
