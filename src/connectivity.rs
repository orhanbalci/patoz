/*!
Parses connectivity records: [SSBOND](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#SSBOND),
[LINK](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#LINK), [CISPEP](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#CISPEP) and
[CONECT](http://www.wwpdb.org/documentation/file-format-content/format33/sect10.html#CONECT).
*/
use crate::{ast::types::*, primitive::*};

/// Parses an SSBOND record.
pub(crate) fn ssbond(line: Line) -> Option<Record> {
    Some(Record::Ssbond(Ssbond {
        serial: line.number(8, 10)?,
        residue1: line.residue(12, 16, 18)?,
        residue2: line.residue(26, 30, 32)?,
        symmetry1: line.text(60, 65).to_owned(),
        symmetry2: line.text(67, 72).to_owned(),
        length: line.number(74, 78),
    }))
}

/// Parses a LINK record.
pub(crate) fn link(line: Line) -> Option<Record> {
    Some(Record::Link(Link {
        name1: line.text(13, 16).to_owned(),
        alt_loc1: line.char_at(17),
        residue1: line.residue(18, 22, 23)?,
        name2: line.text(43, 46).to_owned(),
        alt_loc2: line.char_at(47),
        residue2: line.residue(48, 52, 53)?,
        symmetry1: line.text(60, 65).to_owned(),
        symmetry2: line.text(67, 72).to_owned(),
        length: line.number(74, 78),
    }))
}

/// Parses a CISPEP record.
pub(crate) fn cispep(line: Line) -> Option<Record> {
    Some(Record::Cispep(Cispep {
        serial: line.number(8, 10)?,
        residue1: line.residue(12, 16, 18)?,
        residue2: line.residue(26, 30, 32)?,
        model: line.number(44, 46)?,
        angle: line.number(54, 59),
    }))
}

/// Parses a CONECT record. Bonded atom serials are in columns 12-31,
/// 5 columns each.
pub(crate) fn conect(line: Line) -> Option<Record> {
    let fields = [(12, 16), (17, 21), (22, 26), (27, 31)];
    let bonded = fields
        .iter()
        .map(|&(from, to)| line.text(from, to))
        .filter(|f| !f.is_empty())
        .map(|f| f.parse().ok())
        .collect::<Option<_>>()?;
    Some(Record::Conect(Conect {
        serial: line.number(7, 11)?,
        bonded,
    }))
}

fn symmetry_and_length(
    line: LineBuilder,
    symmetry1: &str,
    symmetry2: &str,
    length: Option<f64>,
) -> LineBuilder {
    line.right(60, 65, symmetry1)
        .right(67, 72, symmetry2)
        .right_opt(74, 78, length.map(|l| format!("{:.2}", l)))
}

/// Writes an SSBOND record.
pub(crate) fn write_ssbond(ssbond: &Ssbond, out: &mut Vec<String>) {
    let line = LineBuilder::new("SSBOND")
        .right(8, 10, ssbond.serial)
        .residue(12, 16, 18, &ssbond.residue1)
        .residue(26, 30, 32, &ssbond.residue2);
    out.push(
        symmetry_and_length(line, &ssbond.symmetry1, &ssbond.symmetry2, ssbond.length).build(),
    );
}

/// Writes a LINK record. `element` looks up the element of an atom by
/// residue and atom name to align atom names as ATOM records do.
pub(crate) fn write_link<'a>(
    link: &Link,
    element: impl Fn(&str, &str) -> Option<&'a str>,
    out: &mut Vec<String>,
) {
    let line = LineBuilder::new("LINK")
        .atom_name(
            13,
            &link.name1,
            element(&link.residue1.residue_name, &link.name1),
        )
        .char_at(17, link.alt_loc1)
        .residue(18, 22, 23, &link.residue1)
        .atom_name(
            43,
            &link.name2,
            element(&link.residue2.residue_name, &link.name2),
        )
        .char_at(47, link.alt_loc2)
        .residue(48, 52, 53, &link.residue2);
    out.push(symmetry_and_length(line, &link.symmetry1, &link.symmetry2, link.length).build());
}

/// Writes a CISPEP record.
pub(crate) fn write_cispep(cispep: &Cispep, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new("CISPEP")
            .right(8, 10, cispep.serial)
            .residue(12, 16, 18, &cispep.residue1)
            .residue(26, 30, 32, &cispep.residue2)
            .right(44, 46, cispep.model)
            .right_opt(54, 59, cispep.angle.map(|a| format!("{:.2}", a)))
            .build(),
    );
}

/// Writes a CONECT record.
pub(crate) fn write_conect(conect: &Conect, out: &mut Vec<String>) {
    let mut line = LineBuilder::new("CONECT").right(7, 11, conect.serial);
    for (i, bonded) in conect.bonded.iter().enumerate() {
        line = line.right(12 + 5 * i, 16 + 5 * i, bonded);
    }
    out.push(line.build());
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record, ResidueRef};

    fn residue(name: &str, chain_id: char, residue_seq: i32) -> ResidueRef {
        ResidueRef {
            residue_name: name.to_owned(),
            chain_id,
            residue_seq,
            insertion_code: None,
        }
    }

    #[test]
    fn ssbond() {
        let r = single_record(
            "SSBOND   1 CYS A    6    CYS A  216                          1555   1555  2.03  \n",
        );
        let Record::Ssbond(s) = r else { panic!() };
        assert_eq!(s.serial, 1);
        assert_eq!(s.residue1, residue("CYS", 'A', 6));
        assert_eq!(s.residue2, residue("CYS", 'A', 216));
        assert_eq!(
            (s.symmetry1.as_str(), s.symmetry2.as_str()),
            ("1555", "1555")
        );
        assert_eq!(s.length, Some(2.03));
    }

    #[test]
    fn link() {
        let r = single_record(
            "LINK         NE2 HIS A  93                FE   HEM A 154     1555   1555  2.06  \n",
        );
        let Record::Link(l) = r else { panic!() };
        assert_eq!(l.name1, "NE2");
        assert_eq!(l.residue1, residue("HIS", 'A', 93));
        assert_eq!(l.name2, "FE");
        assert_eq!(l.residue2, residue("HEM", 'A', 154));
        assert_eq!(l.length, Some(2.06));
    }

    #[test]
    fn cispep() {
        let r = single_record(
            "CISPEP   1 ALA A   72    PRO A   73          0         6.22                     \n",
        );
        let Record::Cispep(c) = r else { panic!() };
        assert_eq!(c.residue1, residue("ALA", 'A', 72));
        assert_eq!(c.residue2, residue("PRO", 'A', 73));
        assert_eq!((c.model, c.angle), (0, Some(6.22)));
    }

    #[test]
    fn conect() {
        let Record::Conect(c) = single_record("CONECT 1179  746 1184 1195 1203\n") else {
            panic!()
        };
        assert_eq!(c.serial, 1179);
        assert_eq!(c.bonded, [746, 1184, 1195, 1203]);
        let Record::Conect(c) = single_record("CONECT   33 1582\n") else {
            panic!()
        };
        assert_eq!(c.bonded, [1582]);
    }
}
