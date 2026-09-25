use crate::{ast::types::*, primitive::*};

/// Parses consecutive [SEQRES](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQRES) lines of one chain.
/// Residue names are in columns 20-70, 4 columns apart.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    Some(Record::Seqres(Seqres {
        chain_id: lines[0].char_at(12),
        num_res: lines[0].int(14, 17)?,
        residues: lines
            .iter()
            .flat_map(|l| l.cols(20, 70).split_whitespace())
            .map(str::to_owned)
            .collect(),
    }))
}

#[cfg(test)]
mod test {
    use crate::{parse, Record};

    #[test]
    fn groups_lines_per_chain() {
        let pdb = parse(
            "SEQRES   1 A   15  ALA ASP THR ILE VAL ALA VAL GLU LEU ASP THR TYR PRO
SEQRES   2 A   15  SER GLY
SEQRES   1 B    2  DA  DT
",
        );
        let [Record::Seqres(a), Record::Seqres(b)] = pdb.records() else {
            panic!()
        };
        assert_eq!(a.chain_id, Some('A'));
        assert_eq!(a.num_res, 15);
        assert_eq!(a.residues.len(), 15);
        assert_eq!(a.residues[13..], ["SER", "GLY"]);
        assert_eq!(b.chain_id, Some('B'));
        assert_eq!(b.residues, ["DA", "DT"]);
    }

    #[test]
    fn blank_chain_id() {
        let pdb = parse("SEQRES   1      2  GLY ALA");
        let [Record::Seqres(s)] = pdb.records() else {
            panic!()
        };
        assert_eq!(s.chain_id, None);
        assert_eq!(s.residues, ["GLY", "ALA"]);
    }
}
