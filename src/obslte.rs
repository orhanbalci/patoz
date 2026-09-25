use crate::{ast::types::*, primitive::*};

/// Parses continued [OBSLTE](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#OBSLTE) lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    Some(Record::Obslte(Obslte {
        replacement_date: parse_all(date, lines[0].text(12, 20))?,
        id_code: lines[0].text(22, 25).to_owned(),
        replacement_ids: lines.iter().flat_map(|l| l.id_codes(32, 9)).collect(),
    }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn obslte() {
        let r = single_record("OBSLTE     31-JAN-94 1MBP      2MBP    \n");
        let Record::Obslte(o) = r else { panic!() };
        assert_eq!(o.id_code, "1MBP");
        assert_eq!(o.replacement_ids, ["2MBP"]);
    }
}
