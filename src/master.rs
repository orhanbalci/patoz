use crate::{ast::types::*, primitive::*};

/// Parses a [MASTER](http://www.wwpdb.org/documentation/file-format-content/format33/sect11.html#MASTER) record.
pub(crate) fn parse(line: Line) -> Option<Record> {
    Some(Record::Master(Master {
        num_remark: line.number(11, 15)?,
        num_het: line.number(21, 25)?,
        num_helix: line.number(26, 30)?,
        num_sheet: line.number(31, 35)?,
        num_site: line.number(41, 45)?,
        num_xform: line.number(46, 50)?,
        num_coord: line.number(51, 55)?,
        num_ter: line.number(56, 60)?,
        num_conect: line.number(61, 65)?,
        num_seq: line.number(66, 70)?,
    }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn master() {
        let r = single_record(
            "MASTER      266    0    0    2    2    0    0    6  340    1    6    4          \n",
        );
        let Record::Master(m) = r else { panic!() };
        assert_eq!(m.num_remark, 266);
        assert_eq!((m.num_helix, m.num_sheet), (2, 2));
        assert_eq!(m.num_xform, 6);
        assert_eq!(
            (m.num_coord, m.num_ter, m.num_conect, m.num_seq),
            (340, 1, 6, 4)
        );
    }
}
