use crate::{ast::types::*, primitive::*};

/// Parses a [DBREF](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF) record.
pub(crate) fn parse(line: Line) -> Option<Record> {
    Some(Record::Dbref(Dbref {
        idcode: line.text(8, 11).to_owned(),
        chain_id: line.char_at(13).unwrap_or(' '),
        seq_begin: line.int(15, 18)?,
        initial_sequence: line.char_at(19),
        seq_end: line.int(21, 24)?,
        ending_sequence: line.char_at(25),
        database: line.text(27, 32).to_owned(),
        db_accession: line.text(34, 41).to_owned(),
        db_idcode: line.text(43, 54).to_owned(),
        db_seq_begin: line.int(56, 60)?,
        idbns_begin: line.char_at(61),
        db_seq_end: line.int(63, 67)?,
        dbins_end: line.char_at(68),
    }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn dbref() {
        let r = single_record(
            "DBREF  2JHQ A    1   226  UNP    Q9KPK8   UNG_VIBCH        1    226  \n",
        );
        let Record::Dbref(d) = r else { panic!() };
        assert_eq!(d.idcode, "2JHQ");
        assert_eq!(d.chain_id, 'A');
        assert_eq!((d.seq_begin, d.seq_end), (1, 226));
        assert_eq!(d.database, "UNP");
        assert_eq!(d.db_accession, "Q9KPK8");
        assert_eq!(d.db_idcode, "UNG_VIBCH");
        assert_eq!((d.db_seq_begin, d.db_seq_end), (1, 226));
    }

    #[test]
    fn negative_residue_number() {
        let r = single_record(
            "DBREF  1ABC A   -5   100  UNP    P12345   ABC_HUMAN        1    106  \n",
        );
        let Record::Dbref(d) = r else { panic!() };
        assert_eq!(d.seq_begin, -5);
    }

    #[test]
    fn self_reference_with_negative_db_positions() {
        let r = single_record(
            "DBREF  1OD3 A  -16    18  PDB    1OD3     1OD3           -16     18             \n",
        );
        let Record::Dbref(d) = r else { panic!() };
        assert_eq!((d.db_seq_begin, d.db_seq_end), (-16, 18));
    }
}
