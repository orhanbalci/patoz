use crate::{ast::types::*, primitive::*};

/// Parses a [DBREF1](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF1) line and its following DBREF2
/// line, used when database ids do not fit DBREF, into a single [Dbref].
pub(crate) fn parse(dbref1: Line, dbref2: Line) -> Option<Record> {
    Some(Record::Dbref(Dbref {
        idcode: dbref1.text(8, 11).to_owned(),
        chain_id: dbref1.char_at(13).unwrap_or(' '),
        seq_begin: dbref1.int(15, 18)?,
        initial_sequence: dbref1.char_at(19),
        seq_end: dbref1.int(21, 24)?,
        ending_sequence: dbref1.char_at(25),
        database: dbref1.text(27, 32).to_owned(),
        db_idcode: dbref1.text(48, 67).to_owned(),
        db_accession: dbref2.text(19, 40).to_owned(),
        db_seq_begin: dbref2.int(46, 55)?,
        idbns_begin: None,
        db_seq_end: dbref2.int(58, 67)?,
        dbins_end: None,
    }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn dbref1() {
        let r = single_record(
            "DBREF1 1ABC A   61   322  UNIMES               UPI000148A153
DBREF2 1ABC A     MES00005880000                     61         322
",
        );
        let Record::Dbref(d) = r else { panic!() };
        assert_eq!(d.idcode, "1ABC");
        assert_eq!(d.database, "UNIMES");
        assert_eq!(d.db_idcode, "UPI000148A153");
        assert_eq!(d.db_accession, "MES00005880000");
        assert_eq!((d.db_seq_begin, d.db_seq_end), (61, 322));
    }
}
