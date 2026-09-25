/*!
Parses the coordinate section: [ATOM](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ATOM),
[HETATM](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#HETATM), [ANISOU](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU),
[TER](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#TER) and [MODEL](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#MODEL) records.
*/
use crate::{ast::types::*, primitive::*};

/// Charge in columns 79-80 written as `2+` or `1-`.
fn charge(line: Line) -> Option<i8> {
    let text = line.text(79, 80);
    let (magnitude, sign) = text.split_at(text.len().checked_sub(1)?);
    let magnitude: i8 = magnitude.parse().ok()?;
    match sign {
        "+" => Some(magnitude),
        "-" => Some(-magnitude),
        _ => None,
    }
}

fn element(line: Line) -> Option<String> {
    Some(line.text(77, 78))
        .filter(|e| !e.is_empty())
        .map(str::to_owned)
}

/// Parses the fields shared by ATOM and HETATM records.
pub(crate) fn atom(line: Line) -> Option<Atom> {
    Some(Atom {
        serial: line.number(7, 11)?,
        name: line.text(13, 16).to_owned(),
        alt_loc: line.char_at(17),
        residue_name: line.text(18, 20).to_owned(),
        chain_id: line.char_at(22).unwrap_or(' '),
        residue_seq: line.number(23, 26)?,
        insertion_code: line.char_at(27),
        x: line.number(31, 38)?,
        y: line.number(39, 46)?,
        z: line.number(47, 54)?,
        occupancy: line.number(55, 60)?,
        temp_factor: line.number(61, 66)?,
        element: element(line),
        charge: charge(line),
    })
}

/// Parses the fields of ANISOU and SIGUIJ records.
pub(crate) fn anisou(line: Line) -> Option<Anisou> {
    Some(Anisou {
        serial: line.number(7, 11)?,
        name: line.text(13, 16).to_owned(),
        alt_loc: line.char_at(17),
        residue_name: line.text(18, 20).to_owned(),
        chain_id: line.char_at(22).unwrap_or(' '),
        residue_seq: line.number(23, 26)?,
        insertion_code: line.char_at(27),
        u11: line.number(29, 35)?,
        u22: line.number(36, 42)?,
        u33: line.number(43, 49)?,
        u12: line.number(50, 56)?,
        u13: line.number(57, 63)?,
        u23: line.number(64, 70)?,
        element: element(line),
        charge: charge(line),
    })
}

/// Parses a TER record. Fields are optional since old entries often
/// contain a bare `TER`.
pub(crate) fn ter(line: Line) -> Option<Record> {
    Some(Record::Ter(Ter {
        serial: line.number(7, 11),
        residue_name: line.text(18, 20).to_owned(),
        chain_id: line.char_at(22).unwrap_or(' '),
        residue_seq: line.number(23, 26),
        insertion_code: line.char_at(27),
    }))
}

/// Parses a MODEL record.
pub(crate) fn model(line: Line) -> Option<Record> {
    Some(Record::Model(Model {
        serial: line.number(11, 14)?,
    }))
}

fn format_charge(charge: Option<i8>) -> String {
    match charge {
        Some(c) if c < 0 => format!("{}-", -c),
        Some(c) => format!("{}+", c),
        None => String::new(),
    }
}

/// Writes an ATOM, HETATM or SIGATM record.
pub(crate) fn write_atom(name: &str, atom: &Atom, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new(name)
            .right(7, 11, atom.serial)
            .atom_name(13, &atom.name, atom.element.as_deref())
            .char_at(17, atom.alt_loc)
            .right(18, 20, &atom.residue_name)
            .char_at(22, Some(atom.chain_id))
            .right(23, 26, atom.residue_seq)
            .char_at(27, atom.insertion_code)
            .right(31, 38, format!("{:.3}", atom.x))
            .right(39, 46, format!("{:.3}", atom.y))
            .right(47, 54, format!("{:.3}", atom.z))
            .right(55, 60, format!("{:.2}", atom.occupancy))
            .right(61, 66, format!("{:.2}", atom.temp_factor))
            .right(77, 78, atom.element.as_deref().unwrap_or(""))
            .left(79, &format_charge(atom.charge))
            .build(),
    );
}

/// Writes an ANISOU or SIGUIJ record.
pub(crate) fn write_anisou(name: &str, anisou: &Anisou, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new(name)
            .right(7, 11, anisou.serial)
            .atom_name(13, &anisou.name, anisou.element.as_deref())
            .char_at(17, anisou.alt_loc)
            .right(18, 20, &anisou.residue_name)
            .char_at(22, Some(anisou.chain_id))
            .right(23, 26, anisou.residue_seq)
            .char_at(27, anisou.insertion_code)
            .right(29, 35, anisou.u11)
            .right(36, 42, anisou.u22)
            .right(43, 49, anisou.u33)
            .right(50, 56, anisou.u12)
            .right(57, 63, anisou.u13)
            .right(64, 70, anisou.u23)
            .right(77, 78, anisou.element.as_deref().unwrap_or(""))
            .left(79, &format_charge(anisou.charge))
            .build(),
    );
}

/// Writes a TER record.
pub(crate) fn write_ter(ter: &Ter, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new("TER")
            .right_opt(7, 11, ter.serial)
            .right(18, 20, &ter.residue_name)
            .char_at(22, Some(ter.chain_id))
            .right_opt(23, 26, ter.residue_seq)
            .char_at(27, ter.insertion_code)
            .build(),
    );
}

/// Writes a MODEL record.
pub(crate) fn write_model(model: &Model, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new("MODEL")
            .right(11, 14, model.serial)
            .build(),
    );
}

#[cfg(test)]
mod test {
    use crate::{parse, test_util::single_record, Record};

    #[test]
    fn atom() {
        let r = single_record(
            "ATOM      2  N  BTHR A   1      17.553  14.234   4.214  0.18  5.51           N  \n",
        );
        let Record::Atom(a) = r else { panic!() };
        assert_eq!(a.serial, 2);
        assert_eq!(a.name, "N");
        assert_eq!(a.alt_loc, Some('B'));
        assert_eq!(a.residue_name, "THR");
        assert_eq!(a.chain_id, 'A');
        assert_eq!(a.residue_seq, 1);
        assert_eq!(a.insertion_code, None);
        assert_eq!((a.x, a.y, a.z), (17.553, 14.234, 4.214));
        assert_eq!((a.occupancy, a.temp_factor), (0.18, 5.51));
        assert_eq!(a.element.as_deref(), Some("N"));
        assert_eq!(a.charge, None);
    }

    #[test]
    fn hetatm_with_charge() {
        let r = single_record(
            "HETATM 2322  O   ACT A1279       9.375  29.086   9.029  1.00 18.52           O1-\n",
        );
        let Record::Hetatm(a) = r else { panic!() };
        assert_eq!(a.residue_seq, 1279);
        assert_eq!(a.charge, Some(-1));
        let r = single_record(
            "HETATM 4087 MG    MG A 623       0.232  55.563   5.347  0.33  4.75          MG2+\n",
        );
        let Record::Hetatm(a) = r else { panic!() };
        assert_eq!(a.element.as_deref(), Some("MG"));
        assert_eq!(a.charge, Some(2));
    }

    #[test]
    fn atom_without_element_columns() {
        let r = single_record("ATOM      1  CA  GLY A  -3      -1.000   2.500 -30.125  1.00  0.00");
        let Record::Atom(a) = r else { panic!() };
        assert_eq!(a.residue_seq, -3);
        assert_eq!(a.z, -30.125);
        assert_eq!(a.element, None);
    }

    #[test]
    fn anisou() {
        let r = single_record(
            "ANISOU    1  N  ATHR A   1      434    531    735    201    133    -28       N  \n",
        );
        let Record::Anisou(u) = r else { panic!() };
        assert_eq!(
            (u.u11, u.u22, u.u33, u.u12, u.u13, u.u23),
            (434, 531, 735, 201, 133, -28)
        );
    }

    #[test]
    fn sigatm_and_siguij() {
        let r = single_record(
            "SIGATM    1  N   SER A   2       0.048   0.063   0.062  0.00  1.58           N  \n",
        );
        let Record::Sigatm(s) = r else { panic!() };
        assert_eq!((s.x, s.temp_factor), (0.048, 1.58));
        let r = single_record(
            "SIGUIJ    6  N   HIS A   3      537    655    469    453    397    449       N  \n",
        );
        let Record::Siguij(s) = r else { panic!() };
        assert_eq!(s.u23, 449);
    }

    #[test]
    fn ter() {
        let Record::Ter(t) = single_record("TER     844      ASN A  46\n") else {
            panic!()
        };
        assert_eq!(t.serial, Some(844));
        assert_eq!((t.residue_name.as_str(), t.residue_seq), ("ASN", Some(46)));
        let Record::Ter(t) = single_record("TER\n") else {
            panic!()
        };
        assert_eq!(t.serial, None);
    }

    #[test]
    fn models() {
        let pdb = parse(
            "MODEL        1
ATOM      1  N   MET A   1      -8.901   4.127  -0.555  1.00  0.00           N
ENDMDL
MODEL        2
ATOM      1  N   MET A   1      -8.521   3.920  -0.325  1.00  0.00           N
ENDMDL
END
",
        );
        assert!(matches!(
            pdb.records(),
            [
                Record::Model(m1),
                Record::Atom(_),
                Record::Endmdl,
                Record::Model(m2),
                Record::Atom(_),
                Record::Endmdl,
                Record::End
            ] if m1.serial == 1 && m2.serial == 2
        ));
    }
}
