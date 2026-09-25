use crate::{ast::types::*, primitive::*};

/// Parses a [SEQADV](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQADV) record.
pub(crate) fn parse(line: Line) -> Option<Record> {
    Some(Record::Seqadv(Seqadv {
        idcode: line.text(8, 11).to_owned(),
        conflicting_residue: line.text(13, 15).to_owned(),
        chain_id: line.char_at(17).unwrap_or(' '),
        sequence_number: line.number(19, 22),
        insertion_code: line.char_at(23),
        database: line.text(25, 28).to_owned(),
        db_accession: line.text(30, 38).to_owned(),
        sequence_db_residue: Some(line.text(40, 42))
            .filter(|r| !r.is_empty())
            .map(str::to_owned),
        sequence_db_sequence_number: line.number(44, 48),
        conflict: line.text(50, 70).to_owned(),
    }))
}

/// Writes a SEQADV record.
pub(crate) fn write(seqadv: &Seqadv, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new("SEQADV")
            .left(8, &seqadv.idcode)
            .right(13, 15, &seqadv.conflicting_residue)
            .char_at(17, Some(seqadv.chain_id))
            .right_opt(19, 22, seqadv.sequence_number)
            .char_at(23, seqadv.insertion_code)
            .left(25, &seqadv.database)
            .left(30, &seqadv.db_accession)
            .right(40, 42, seqadv.sequence_db_residue.as_deref().unwrap_or(""))
            .right_opt(44, 48, seqadv.sequence_db_sequence_number)
            .left(50, &seqadv.conflict)
            .build(),
    );
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn seqadv() {
        let r = single_record(
            "SEQADV 1DY5 IAS A   67  UNP  P61823    ASN    93 CONFLICT                       \n",
        );
        let Record::Seqadv(s) = r else { panic!() };
        assert_eq!(s.conflicting_residue, "IAS");
        assert_eq!(s.sequence_number, Some(67));
        assert_eq!(s.sequence_db_residue.as_deref(), Some("ASN"));
        assert_eq!(s.sequence_db_sequence_number, Some(93));
        assert_eq!(s.conflict, "CONFLICT");
    }

    #[test]
    fn expression_tag_without_db_residue() {
        let r = single_record("SEQADV 1ABC MET A   -1  UNP  P12345              EXPRESSION TAG\n");
        let Record::Seqadv(s) = r else { panic!() };
        assert_eq!(s.sequence_number, Some(-1));
        assert_eq!(s.sequence_db_residue, None);
        assert_eq!(s.conflict, "EXPRESSION TAG");
    }

    #[test]
    fn deletion_without_structure_residue() {
        let r = single_record(
            "SEQADV 1GKM     B       UNP  P21310    ALA   143 DELETION                       \n",
        );
        let Record::Seqadv(s) = r else { panic!() };
        assert_eq!(s.conflicting_residue, "");
        assert_eq!(s.sequence_number, None);
        assert_eq!(s.sequence_db_sequence_number, Some(143));
        assert_eq!(s.conflict, "DELETION");
    }
}
