/*!
Parses the heterogen section: [HET](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HET),
[HETNAM](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HETNAM), [HETSYN](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HETSYN) and
[FORMUL](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#FORMUL) records.
*/
use crate::{ast::types::*, primitive::*};

/// Parses a HET record.
pub(crate) fn het(line: Line) -> Option<Record> {
    Some(Record::Het(Het {
        residue: line.residue(8, 13, 14)?,
        num_het_atoms: line.number(21, 25)?,
        text: line.text(31, 70).to_owned(),
    }))
}

/// Parses continued HETNAM lines of one heterogen.
pub(crate) fn hetnam(lines: &[Line]) -> Option<Record> {
    Some(Record::Hetnam(Hetnam {
        het_id: lines[0].text(12, 14).to_owned(),
        name: join_chemical_name(lines.iter().map(|l| l.cols(16, 70))),
    }))
}

/// Parses continued HETSYN lines of one heterogen. Synonyms are separated
/// by `;`.
pub(crate) fn hetsyn(lines: &[Line]) -> Option<Record> {
    let text = join_chemical_name(lines.iter().map(|l| l.cols(16, 70)));
    Some(Record::Hetsyn(Hetsyn {
        het_id: lines[0].text(12, 14).to_owned(),
        synonyms: parse_all(list(';'), &text)?,
    }))
}

/// Parses continued FORMUL lines of one heterogen. Formulas are joined
/// with spaces since a line may end with a charge such as `2-`.
pub(crate) fn formul(lines: &[Line]) -> Option<Record> {
    let formula = lines
        .iter()
        .map(|l| l.text(20, 70))
        .filter(|t| !t.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    Some(Record::Formul(Formul {
        component: lines[0].number(9, 10)?,
        het_id: lines[0].text(13, 15).to_owned(),
        water: lines[0].char_at(19) == Some('*'),
        formula,
    }))
}

#[cfg(test)]
mod test {
    use crate::{parse, test_util::single_record, Record};

    #[test]
    fn het() {
        let Record::Het(h) = single_record("HET    PP7  A 324      74\n") else {
            panic!()
        };
        assert_eq!(h.residue.residue_name, "PP7");
        assert_eq!((h.residue.chain_id, h.residue.residue_seq), ('A', 324));
        assert_eq!(h.num_het_atoms, 74);
    }

    #[test]
    fn hetnam() {
        let pdb = parse(
            "HETNAM     MAN ALPHA-D-MANNOPYRANOSE
HETNAM     PP7 METHYL CYCLO[(2S)-2-[[(1R)-1-(N-(L-N-(3-
HETNAM   2 PP7  METHYLBUTANOYL)VALYL-L-ASPARTYL)AMINO)-3-
HETNAM   3 PP7  METHYLBUTYL]HYDROXYPHOSPHINYLOXY]-3-(3-AMINOMETHYL)
HETNAM   4 PP7  PHENYLPROPANOATE
",
        );
        let [Record::Hetnam(man), Record::Hetnam(pp7)] = pdb.records() else {
            panic!()
        };
        assert_eq!(man.name, "ALPHA-D-MANNOPYRANOSE");
        assert_eq!(pp7.het_id, "PP7");
        assert_eq!(
            pp7.name,
            "METHYL CYCLO[(2S)-2-[[(1R)-1-(N-(L-N-(3-METHYLBUTANOYL)VALYL-L-ASPARTYL)AMINO)-3-METHYLBUTYL]HYDROXYPHOSPHINYLOXY]-3-(3-AMINOMETHYL)PHENYLPROPANOATE"
        );
    }

    #[test]
    fn hetnam_break_at_space() {
        let r = single_record(
            "HETNAM     NDP NADPH DIHYDRO-NICOTINAMIDE-ADENINE-DINUCLEOTIDE
HETNAM   2 NDP  PHOSPHATE
",
        );
        let Record::Hetnam(n) = r else { panic!() };
        assert_eq!(
            n.name,
            "NADPH DIHYDRO-NICOTINAMIDE-ADENINE-DINUCLEOTIDE PHOSPHATE"
        );
    }

    #[test]
    fn hetsyn() {
        let r = single_record("HETSYN     MAN ALPHA-D-MANNOSE; D-MANNOSE; MANNOSE\n");
        let Record::Hetsyn(s) = r else { panic!() };
        assert_eq!(s.synonyms, ["ALPHA-D-MANNOSE", "D-MANNOSE", "MANNOSE"]);
    }

    #[test]
    fn formul() {
        let pdb = parse(
            "FORMUL   4  SO4    O4 S 2-
FORMUL   8  HOH   *528(H2 O)
",
        );
        let [Record::Formul(so4), Record::Formul(hoh)] = pdb.records() else {
            panic!()
        };
        assert_eq!((so4.component, so4.het_id.as_str()), (4, "SO4"));
        assert_eq!((so4.formula.as_str(), so4.water), ("O4 S 2-", false));
        assert_eq!((hoh.formula.as_str(), hoh.water), ("528(H2 O)", true));
    }
}
