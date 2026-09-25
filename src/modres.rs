use crate::{ast::types::*, primitive::*};

/// Parses a [MODRES](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#MODRES) record.
pub(crate) fn parse(line: Line) -> Option<Record> {
    Some(Record::Modres(Modres {
        idcode: line.text(8, 11).to_owned(),
        residue_name: line.text(13, 15).to_owned(),
        chain_id: line.char_at(17).unwrap_or(' '),
        sequence_number: line.number(19, 22)?,
        insertion_code: line.char_at(23),
        standart_residue_name: line.text(25, 27).to_owned(),
        comment: line.text(30, 80).to_owned(),
    }))
}

/// Writes a MODRES record.
pub(crate) fn write(modres: &Modres, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new("MODRES")
            .left(8, &modres.idcode)
            .right(13, 15, &modres.residue_name)
            .char_at(17, Some(modres.chain_id))
            .right(19, 22, modres.sequence_number)
            .char_at(23, modres.insertion_code)
            .right(25, 27, &modres.standart_residue_name)
            .left(30, &modres.comment)
            .build(),
    );
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn modres() {
        let r = single_record("MODRES 2R0L ASN A   74  ASN  GLYCOSYLATION SITE  \n");
        let Record::Modres(m) = r else { panic!() };
        assert_eq!(m.idcode, "2R0L");
        assert_eq!(m.residue_name, "ASN");
        assert_eq!(m.chain_id, 'A');
        assert_eq!(m.sequence_number, 74);
        assert_eq!(m.insertion_code, None);
        assert_eq!(m.standart_residue_name, "ASN");
        assert_eq!(m.comment, "GLYCOSYLATION SITE");
    }
}
