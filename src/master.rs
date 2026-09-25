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

/// Writes a MASTER record.
pub(crate) fn write(master: &Master, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new("MASTER")
            .right(11, 15, master.num_remark)
            .right(16, 20, 0)
            .right(21, 25, master.num_het)
            .right(26, 30, master.num_helix)
            .right(31, 35, master.num_sheet)
            .right(36, 40, 0)
            .right(41, 45, master.num_site)
            .right(46, 50, master.num_xform)
            .right(51, 55, master.num_coord)
            .right(56, 60, master.num_ter)
            .right(61, 65, master.num_conect)
            .right(66, 70, master.num_seq)
            .build(),
    );
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
