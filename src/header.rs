use crate::{ast::types::*, primitive::*};

/// Parses a [HEADER](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#HEADER) record.
pub(crate) fn parse(line: Line) -> Option<Record> {
    Some(Record::Header(Header {
        classification: line.text(11, 50).to_owned(),
        deposition_date: parse_all(date, line.text(51, 59))?,
        id_code: line.text(63, 66).to_owned(),
    }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn header() {
        let r =
            single_record("HEADER    TRANSFERASE/TRANSFERASE                 28-MAR-07   2UXK \n");
        let Record::Header(h) = r else { panic!() };
        assert_eq!(h.classification, "TRANSFERASE/TRANSFERASE");
        assert_eq!(h.deposition_date.to_string(), "2007-03-28");
        assert_eq!(h.id_code, "2UXK");
    }
}
