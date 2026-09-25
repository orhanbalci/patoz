/*!
Parses the crystallographic and coordinate transformation section:
[CRYST1](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#CRYST1), [ORIGXn](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#ORIGXn),
[SCALEn](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#SCALEn) and [MTRIXn](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#MTRIXn).
*/
use crate::{ast::types::*, primitive::*};

/// Parses a CRYST1 record.
pub(crate) fn cryst1(line: Line) -> Option<Record> {
    Some(Record::Cryst1(Cryst1 {
        a: line.number(7, 15)?,
        b: line.number(16, 24)?,
        c: line.number(25, 33)?,
        alpha: line.number(34, 40)?,
        beta: line.number(41, 47)?,
        gamma: line.number(48, 54)?,
        space_group: line.text(56, 66).to_owned(),
        z: line.number(67, 70),
    }))
}

/// Parses the three rows of an ORIGXn, SCALEn or MTRIXn record. Row `n`
/// must be on line `n`.
fn transformation(lines: &[Line], name: &str) -> Option<Transformation> {
    let [row1, row2, row3] = lines else {
        return None;
    };
    let mut transformation = Transformation::default();
    for (n, row) in [row1, row2, row3].into_iter().enumerate() {
        if row.record_name() != format!("{}{}", name, n + 1) {
            return None;
        }
        transformation.matrix[n] = [
            row.number(11, 20)?,
            row.number(21, 30)?,
            row.number(31, 40)?,
        ];
        transformation.vector[n] = row.number(46, 55)?;
    }
    Some(transformation)
}

/// Parses ORIGX1-3 lines.
pub(crate) fn origx(lines: &[Line]) -> Option<Record> {
    transformation(lines, "ORIGX").map(Record::Origx)
}

/// Parses SCALE1-3 lines.
pub(crate) fn scale(lines: &[Line]) -> Option<Record> {
    transformation(lines, "SCALE").map(Record::Scale)
}

/// Parses MTRIX1-3 lines of one operation.
pub(crate) fn mtrix(lines: &[Line]) -> Option<Record> {
    Some(Record::Mtrix(Mtrix {
        serial: lines[0].number(8, 10)?,
        transformation: transformation(lines, "MTRIX")?,
        given: lines[0].char_at(60) == Some('1'),
    }))
}

/// Writes a CRYST1 record.
pub(crate) fn write_cryst1(cryst1: &Cryst1, out: &mut Vec<String>) {
    out.push(
        LineBuilder::new("CRYST1")
            .right(7, 15, format!("{:.3}", cryst1.a))
            .right(16, 24, format!("{:.3}", cryst1.b))
            .right(25, 33, format!("{:.3}", cryst1.c))
            .right(34, 40, format!("{:.2}", cryst1.alpha))
            .right(41, 47, format!("{:.2}", cryst1.beta))
            .right(48, 54, format!("{:.2}", cryst1.gamma))
            .left(56, &cryst1.space_group)
            .right_opt(67, 70, cryst1.z)
            .build(),
    );
}

/// Writes the three lines of an ORIGXn, SCALEn or MTRIXn record.
pub(crate) fn write_transformation(
    name: &str,
    transformation: &Transformation,
    line: impl Fn(LineBuilder) -> LineBuilder,
    out: &mut Vec<String>,
) {
    for n in 0..3 {
        let [a, b, c] = transformation.matrix[n];
        out.push(
            line(LineBuilder::new(&format!("{}{}", name, n + 1)))
                .right(11, 20, format!("{:.6}", a))
                .right(21, 30, format!("{:.6}", b))
                .right(31, 40, format!("{:.6}", c))
                .right(46, 55, format!("{:.5}", transformation.vector[n]))
                .build(),
        );
    }
}

/// Writes MTRIX1-3 lines.
pub(crate) fn write_mtrix(mtrix: &Mtrix, out: &mut Vec<String>) {
    let given = mtrix.given.then_some('1');
    write_transformation(
        "MTRIX",
        &mtrix.transformation,
        |l| l.right(8, 10, mtrix.serial).char_at(60, given),
        out,
    );
}

#[cfg(test)]
mod test {
    use crate::{parse, test_util::single_record, Record};

    #[test]
    fn cryst1() {
        let r = single_record(
            "CRYST1   89.550   86.460   62.110  90.00  90.00  90.00 I 2 2 2       8          \n",
        );
        let Record::Cryst1(c) = r else { panic!() };
        assert_eq!((c.a, c.b, c.c), (89.55, 86.46, 62.11));
        assert_eq!((c.alpha, c.beta, c.gamma), (90.0, 90.0, 90.0));
        assert_eq!(c.space_group, "I 2 2 2");
        assert_eq!(c.z, Some(8));
    }

    #[test]
    fn scale() {
        let r = single_record(
            "SCALE1      0.011167  0.000000  0.000000        0.00000
SCALE2      0.000000  0.011566  0.000000        0.00000
SCALE3      0.000000  0.000000  0.016100        0.50000
",
        );
        let Record::Scale(s) = r else { panic!() };
        assert_eq!(s.matrix[1], [0.0, 0.011566, 0.0]);
        assert_eq!(s.vector, [0.0, 0.0, 0.5]);
    }

    #[test]
    fn mtrix() {
        let r = single_record(
            "MTRIX1   1 -0.254172  0.010299  0.967104       29.27348    1
MTRIX2   1 -0.911654  0.331324 -0.243127       35.04161    1
MTRIX3   1 -0.322929 -0.943461 -0.074824       26.58102    1
",
        );
        let Record::Mtrix(m) = r else { panic!() };
        assert_eq!(m.serial, 1);
        assert!(m.given);
        assert_eq!(
            m.transformation.matrix[2],
            [-0.322929, -0.943461, -0.074824]
        );
        assert_eq!(m.transformation.vector[0], 29.27348);
    }

    #[test]
    fn incomplete_transformation_is_unknown() {
        let pdb = parse(
            "ORIGX1      1.000000  0.000000  0.000000        0.00000
ORIGX2      0.000000  1.000000  0.000000        0.00000
",
        );
        assert!(matches!(
            pdb.records(),
            [Record::Unknown(_), Record::Unknown(_)]
        ));
    }
}
