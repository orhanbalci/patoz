use super::{ast::types::*, primitive::*};
use nom::IResult;

struct SeqresLine {
    chain_id: Option<char>,
    num_res: u32,
    residues: Vec<String>,
}

/// Parses a single line of
/// [SEQRES](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQRES)
/// record by its fixed columns.
///
/// |COLUMNS    |   DATA TYPE     | FIELD      |  DEFINITION                                   |
/// |-----------|-----------------|------------|-----------------------------------------------|
/// |1 -  6     |    Record name  |  SEQRES    |                                               |
/// |8 - 10     |    Integer      |  serNum    |   Serial number of the SEQRES record for the  |
/// |           |                 |            |   current chain. Reset to 1 for each chain.   |
/// |12         |    Character    |  chainID   |   Chain identifier. Blank if single chain.    |
/// |14 - 17    |    Integer      |  numRes    |   Number of residues in the chain.            |
/// |20 - 70    |    Residue name |  resName   |   Up to 13 residue names, 4 columns apart.    |
fn seqres_line_parser(s: &[u8]) -> IResult<&[u8], SeqresLine> {
    let (rest, l) = line(s)?;
    let fail = || nom::Err::Error((s, nom::error::ErrorKind::Tag));
    if !l.starts_with(b"SEQRES") {
        return Err(fail());
    }
    let num_res = columns(l, 14, 17).trim().parse().map_err(|_| fail())?;
    let chain_id = columns(l, 12, 12).chars().next().filter(|c| *c != ' ');
    let residues = columns(l, 20, 70)
        .split_whitespace()
        .map(str::to_owned)
        .collect();
    Ok((
        rest,
        SeqresLine {
            chain_id,
            num_res,
            residues,
        },
    ))
}

/// Parses consecutive SEQRES lines of one chain into a single record.
pub fn seqres_record_parser(s: &[u8]) -> IResult<&[u8], Record> {
    let (mut rest, first) = seqres_line_parser(s)?;
    let mut seqres = Seqres {
        chain_id: first.chain_id,
        num_res: first.num_res,
        residues: first.residues,
    };
    while let Ok((next_rest, next)) = seqres_line_parser(rest) {
        if next.chain_id != seqres.chain_id {
            break;
        }
        seqres.residues.extend(next.residues);
        rest = next_rest;
    }
    Ok((rest, Record::Seqres(seqres)))
}

#[cfg(test)]
mod test {
    use super::*;

    const TWO_CHAINS: &[u8] =
        b"SEQRES   1 A   15  ALA ASP THR ILE VAL ALA VAL GLU LEU ASP THR TYR PRO
SEQRES   2 A   15  SER GLY
SEQRES   1 B    2  DA  DT
END
";

    #[test]
    fn groups_lines_per_chain() {
        let (rest, a) = seqres_record_parser(TWO_CHAINS).unwrap();
        let (rest, b) = seqres_record_parser(rest).unwrap();
        assert_eq!(rest, b"END\n");
        match (a, b) {
            (Record::Seqres(a), Record::Seqres(b)) => {
                assert_eq!(a.chain_id, Some('A'));
                assert_eq!(a.num_res, 15);
                assert_eq!(a.residues.len(), 15);
                assert_eq!(a.residues[13..], ["SER", "GLY"]);
                assert_eq!(b.chain_id, Some('B'));
                assert_eq!(b.residues, ["DA", "DT"]);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn blank_chain_id() {
        let (_, r) = seqres_record_parser(b"SEQRES   1      2  GLY ALA").unwrap();
        match r {
            Record::Seqres(s) => {
                assert_eq!(s.chain_id, None);
                assert_eq!(s.residues, ["GLY", "ALA"]);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn rejects_other_records() {
        assert!(seqres_record_parser(b"SEQADV 1BXO\n").is_err());
    }
}
