/*!
Parses [REMARK](http://www.wwpdb.org/documentation/file-format-content/format33/remarks.html) records. Every remark is kept as text;
remarks 2, 350 and 465 can additionally be interpreted.
*/
use crate::{ast::types::*, primitive::*};

/// Parses consecutive REMARK lines with the same remark number.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    Some(Record::Remark(Remark {
        number: lines[0].number(8, 10)?,
        lines: lines
            .iter()
            .map(|l| l.cols(12, 80).trim_end().to_owned())
            .collect(),
    }))
}

// Remark text starts at column 12, so a column c of the specification is
// column c - 11 of a remark line.

/// Resolution in angstroms from REMARK 2. `Some(None)` when the remark says
/// resolution is not applicable, e.g. for NMR entries.
pub(crate) fn resolution(remark: &Remark) -> Option<Option<f64>> {
    let text = remark
        .lines
        .iter()
        .find_map(|l| l.strip_prefix("RESOLUTION."))?
        .trim();
    if text == "NOT APPLICABLE." {
        return Some(None);
    }
    let value = text.strip_suffix("ANGSTROMS.")?.trim();
    value.parse().ok().map(Some)
}

/// Residues listed after the `M RES C SSSEQI` header of REMARK 465.
pub(crate) fn missing_residues(remark: &Remark) -> Option<Vec<MissingResidue>> {
    let header = remark
        .lines
        .iter()
        .position(|l| l.split_whitespace().eq(["M", "RES", "C", "SSSEQI"]))?;
    remark.lines[header + 1..]
        .iter()
        .map(|l| {
            let line = Line(l);
            Some(MissingResidue {
                model: line.number(1, 3),
                residue: ResidueRef {
                    residue_name: line.text(5, 7).to_owned(),
                    chain_id: line.char_at(9).unwrap_or(' '),
                    residue_seq: line.number(11, 15)?,
                    insertion_code: line.char_at(16),
                },
            })
        })
        .collect()
}

/// Biological assemblies of REMARK 350.
pub(crate) fn biological_assemblies(remark: &Remark) -> Option<Vec<BiologicalAssembly>> {
    let mut assemblies: Vec<BiologicalAssembly> = Vec::new();
    let mut rows = Vec::new();
    for text in &remark.lines {
        let line = Line(text);
        let field = |key: &str| text.strip_prefix(key).map(str::trim);
        if let Some(id) = field("BIOMOLECULE:") {
            assemblies.push(BiologicalAssembly {
                id: id.parse().ok()?,
                ..Default::default()
            });
            continue;
        }
        if line.text(1, 2).is_empty() && line.text(3, 7) == "BIOMT" {
            rows.push(line);
            if let [row1, row2, row3] = rows[..] {
                rows.clear();
                let mut transformation = Transformation::default();
                for (n, row) in [row1, row2, row3].into_iter().enumerate() {
                    if row.number::<usize>(8, 8)? != n + 1 {
                        return None;
                    }
                    transformation.matrix[n] = [
                        row.number(13, 22)?,
                        row.number(23, 32)?,
                        row.number(33, 42)?,
                    ];
                    transformation.vector[n] = row.number(48, 57)?;
                }
                let part = assemblies.last_mut()?.parts.last_mut()?;
                part.operations.push(transformation);
            }
            continue;
        }
        let assembly = match assemblies.last_mut() {
            Some(assembly) => assembly,
            None => continue,
        };
        if let Some(unit) = field("AUTHOR DETERMINED BIOLOGICAL UNIT:") {
            assembly.author_determined_unit = Some(unit.to_owned());
        } else if let Some(unit) = field("SOFTWARE DETERMINED QUATERNARY STRUCTURE:") {
            assembly.software_determined_unit = Some(unit.to_owned());
        } else if let Some(chains) = field("APPLY THE FOLLOWING TO CHAINS:") {
            assembly.parts.push(AssemblyPart {
                chains: parse_all(list(','), chains)?,
                operations: Vec::new(),
            });
        } else if let Some(chains) = text.trim_start().strip_prefix("AND CHAINS:") {
            let part = assembly.parts.last_mut()?;
            part.chains.extend(parse_all(list(','), chains)?);
        }
    }
    rows.is_empty().then_some(assemblies)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{parse as parse_pdb, test_util::single_record};

    fn remark(content: &str) -> Remark {
        match single_record(content) {
            Record::Remark(r) => r,
            r => panic!("{:?}", r),
        }
    }

    #[test]
    fn remark_text_is_kept() {
        let r = remark("REMARK   2\nREMARK   2 RESOLUTION.    1.74 ANGSTROMS.\n");
        assert_eq!(r.number, 2);
        assert_eq!(r.lines, ["", "RESOLUTION.    1.74 ANGSTROMS."]);
    }

    #[test]
    fn remarks_are_grouped_by_number() {
        let pdb = parse_pdb("REMARK   1\nREMARK   2\nREMARK   2 RESOLUTION.    1.74 ANGSTROMS.\n");
        assert!(matches!(
            pdb.records(),
            [Record::Remark(_), Record::Remark(_)]
        ));
    }

    #[test]
    fn resolution_value() {
        let r = remark("REMARK   2\nREMARK   2 RESOLUTION.    1.74 ANGSTROMS.\n");
        assert_eq!(resolution(&r), Some(Some(1.74)));
        let r = remark("REMARK   2\nREMARK   2 RESOLUTION. NOT APPLICABLE.\n");
        assert_eq!(resolution(&r), Some(None));
    }

    #[test]
    fn missing() {
        let r = remark(
            "REMARK 465
REMARK 465 MISSING RESIDUES
REMARK 465 EXPERIMENT. (M=MODEL NUMBER; RES=RESIDUE NAME; C=CHAIN
REMARK 465
REMARK 465   M RES C SSSEQI
REMARK 465     GLY A   264
REMARK 465   2 MET B    -1A
",
        );
        let missing = missing_residues(&r).unwrap();
        assert_eq!(missing.len(), 2);
        assert_eq!(missing[0].model, None);
        assert_eq!(missing[0].residue.residue_name, "GLY");
        assert_eq!(missing[0].residue.residue_seq, 264);
        assert_eq!(missing[1].model, Some(2));
        assert_eq!(missing[1].residue.chain_id, 'B');
        assert_eq!(missing[1].residue.residue_seq, -1);
        assert_eq!(missing[1].residue.insertion_code, Some('A'));
    }

    #[test]
    fn assemblies() {
        let r = remark(
            "REMARK 350 BIOMOLECULE: 1
REMARK 350 AUTHOR DETERMINED BIOLOGICAL UNIT: TETRADECAMERIC
REMARK 350 SOFTWARE USED: PISA
REMARK 350 APPLY THE FOLLOWING TO CHAINS: A, B, C, D, E, F, G, H, I, J,
REMARK 350                    AND CHAINS: K, L, M, N
REMARK 350   BIOMT1   1  1.000000  0.000000  0.000000        0.00000
REMARK 350   BIOMT2   1  0.000000  1.000000  0.000000        0.00000
REMARK 350   BIOMT3   1  0.000000  0.000000  1.000000        0.00000
REMARK 350   BIOMT1   2 -1.000000  0.000000  0.000000      132.50000
REMARK 350   BIOMT2   2  0.000000  1.000000  0.000000        0.00000
REMARK 350   BIOMT3   2  0.000000  0.000000 -1.000000        0.00000
REMARK 350 BIOMOLECULE: 2
REMARK 350 APPLY THE FOLLOWING TO CHAINS: O
REMARK 350   BIOMT1   1  1.000000  0.000000  0.000000        0.00000
REMARK 350   BIOMT2   1  0.000000  1.000000  0.000000        0.00000
REMARK 350   BIOMT3   1  0.000000  0.000000  1.000000        0.00000
",
        );
        let assemblies = biological_assemblies(&r).unwrap();
        assert_eq!(assemblies.len(), 2);
        let first = &assemblies[0];
        assert_eq!(
            first.author_determined_unit.as_deref(),
            Some("TETRADECAMERIC")
        );
        assert_eq!(first.parts[0].chains.len(), 14);
        assert_eq!(first.parts[0].operations.len(), 2);
        assert_eq!(first.parts[0].operations[1].matrix[0], [-1.0, 0.0, 0.0]);
        assert_eq!(first.parts[0].operations[1].vector, [132.5, 0.0, 0.0]);
        assert_eq!(assemblies[1].parts[0].chains, ["O"]);
    }

    #[test]
    fn incomplete_biomt_fails() {
        let r = remark(
            "REMARK 350 BIOMOLECULE: 1
REMARK 350 APPLY THE FOLLOWING TO CHAINS: A
REMARK 350   BIOMT1   1  1.000000  0.000000  0.000000        0.00000
",
        );
        assert_eq!(biological_assemblies(&r), None);
    }
}
