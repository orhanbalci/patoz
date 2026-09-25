/*!
Parses secondary structure records [HELIX](http://www.wwpdb.org/documentation/file-format-content/format33/sect5.html#HELIX) and
[SHEET](http://www.wwpdb.org/documentation/file-format-content/format33/sect5.html#SHEET), and [SITE](http://www.wwpdb.org/documentation/file-format-content/format33/sect7.html#SITE) records.
*/
use crate::{ast::types::*, primitive::*};

/// Parses a HELIX record.
pub(crate) fn helix(line: Line) -> Option<Record> {
    Some(Record::Helix(Helix {
        serial: line.number(8, 10)?,
        helix_id: line.text(12, 14).to_owned(),
        start: line.residue(16, 20, 22)?,
        end: line.residue(28, 32, 34)?,
        class: line.number(39, 40),
        comment: line.text(41, 70).to_owned(),
        length: line.number(72, 76),
    }))
}

/// Parses a SHEET record, one strand of a sheet.
pub(crate) fn sheet(line: Line) -> Option<Record> {
    let registration = if line.text(42, 70).is_empty() {
        None
    } else {
        Some(SheetRegistration {
            current_atom: line.text(42, 45).to_owned(),
            current: line.residue(46, 50, 51)?,
            previous_atom: line.text(57, 60).to_owned(),
            previous: line.residue(61, 65, 66)?,
        })
    };
    Some(Record::Sheet(Sheet {
        strand: line.number(8, 10)?,
        sheet_id: line.text(12, 14).to_owned(),
        num_strands: line.number(15, 16)?,
        start: line.residue(18, 22, 23)?,
        end: line.residue(29, 33, 34)?,
        sense: line.number(39, 40)?,
        registration,
    }))
}

/// Parses consecutive SITE lines of one site. Each line lists up to four
/// residues.
pub(crate) fn site(lines: &[Line]) -> Option<Record> {
    let residues = lines
        .iter()
        .flat_map(|l| [(19, 23, 24), (30, 34, 35), (41, 45, 46), (52, 56, 57)].map(|c| (l, c)))
        .filter(|(l, (name, _, _))| !l.text(*name, name + 2).is_empty())
        .map(|(l, (name, chain, seq))| l.residue(name, chain, seq))
        .collect::<Option<_>>()?;
    Some(Record::Site(Site {
        site_id: lines[0].text(12, 14).to_owned(),
        num_res: lines[0].number(16, 17)?,
        residues,
    }))
}

/// Writes a HELIX record.
pub(crate) fn write_helix(helix: &Helix, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new("HELIX")
            .right(8, 10, helix.serial)
            .right(12, 14, &helix.helix_id)
            .residue(16, 20, 22, &helix.start)
            .residue(28, 32, 34, &helix.end)
            .right_opt(39, 40, helix.class)
            .left(41, &helix.comment)
            .right_opt(72, 76, helix.length)
            .build(),
    );
}

/// Writes a SHEET record.
pub(crate) fn write_sheet(sheet: &Sheet, out: &mut Vec<String>) {
    let mut line = LineBuilder::new("SHEET")
        .right(8, 10, sheet.strand)
        .right(12, 14, &sheet.sheet_id)
        .right(15, 16, sheet.num_strands)
        .residue(18, 22, 23, &sheet.start)
        .residue(29, 33, 34, &sheet.end)
        .right(39, 40, sheet.sense);
    if let Some(r) = &sheet.registration {
        line = line
            .atom_name(42, &r.current_atom, None)
            .residue(46, 50, 51, &r.current)
            .atom_name(57, &r.previous_atom, None)
            .residue(61, 65, 66, &r.previous);
    }
    out.push(line.build());
}

/// Writes SITE lines, four residues per line.
pub(crate) fn write_site(site: &Site, out: &mut Vec<String>) {
    let columns = [(19, 23, 24), (30, 34, 35), (41, 45, 46), (52, 56, 57)];
    for (i, residues) in site.residues.chunks(4).enumerate() {
        let mut line = LineBuilder::new("SITE")
            .right(8, 10, i + 1)
            .right(12, 14, &site.site_id)
            .right(16, 17, site.num_res);
        for (residue, (name, chain, seq)) in residues.iter().zip(columns) {
            line = line.residue(name, chain, seq, residue);
        }
        out.push(line.build());
    }
}

#[cfg(test)]
mod test {
    use crate::{parse, test_util::single_record, Record, ResidueRef};

    fn residue(name: &str, chain_id: char, residue_seq: i32) -> ResidueRef {
        ResidueRef {
            residue_name: name.to_owned(),
            chain_id,
            residue_seq,
            insertion_code: None,
        }
    }

    #[test]
    fn helix() {
        let r = single_record(
            "HELIX    1   1 CYS A   12  ASP A   16  5                                   5    \n",
        );
        let Record::Helix(h) = r else { panic!() };
        assert_eq!(h.serial, 1);
        assert_eq!(h.helix_id, "1");
        assert_eq!(h.start, residue("CYS", 'A', 12));
        assert_eq!(h.end, residue("ASP", 'A', 16));
        assert_eq!((h.class, h.length), (Some(5), Some(5)));
    }

    #[test]
    fn sheet() {
        let pdb = parse(
            "SHEET    1   A 7 LYS A  36  LYS A  39  0
SHEET    2   A 7 HIS A  24  ILE A  29 -1  N  ILE A  27   O  LYS A  36
",
        );
        let [Record::Sheet(first), Record::Sheet(second)] = pdb.records() else {
            panic!()
        };
        assert_eq!((first.sheet_id.as_str(), first.num_strands), ("A", 7));
        assert_eq!(first.start, residue("LYS", 'A', 36));
        assert_eq!((first.sense, &first.registration), (0, &None));
        assert_eq!(second.sense, -1);
        let registration = second.registration.as_ref().unwrap();
        assert_eq!(registration.current_atom, "N");
        assert_eq!(registration.current, residue("ILE", 'A', 27));
        assert_eq!(registration.previous_atom, "O");
        assert_eq!(registration.previous, residue("LYS", 'A', 36));
    }

    #[test]
    fn site() {
        let r = single_record(
            "SITE     1 AC1  6 GLU A   8  ASP A  10  ASP A  19  HIS A  24
SITE     2 AC1  6 HOH A 412  HOH A 413
",
        );
        let Record::Site(s) = r else { panic!() };
        assert_eq!((s.site_id.as_str(), s.num_res), ("AC1", 6));
        assert_eq!(s.residues.len(), 6);
        assert_eq!(s.residues[5], residue("HOH", 'A', 413));
    }
}
