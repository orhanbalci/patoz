use crate::{ast::types::*, primitive::*};

/// Parses all [REVDAT](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#REVDAT) lines. Lines with the same
/// modification number (columns 8-10) are continuations of one revision.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    let revdat = lines
        .chunk_by(|a, b| a.cols(8, 10) == b.cols(8, 10))
        .map(|revision| {
            let first = revision[0];
            Some(Revdat {
                modification_number: first.number(8, 10)?,
                modification_date: parse_all(date, first.text(14, 22))?,
                idcode: first.text(24, 27).to_owned(),
                modification_type: match first.number(32, 32)? {
                    0 => ModificationType::InitialRelease,
                    1 => ModificationType::OtherModification,
                    other => ModificationType::UnknownModification(other),
                },
                modification_detail: revision
                    .iter()
                    .flat_map(|l| {
                        [(40, 45), (47, 52), (54, 59), (61, 66)].map(|(f, t)| l.text(f, t))
                    })
                    .filter(|d| !d.is_empty())
                    .map(str::to_owned)
                    .collect(),
            })
        })
        .collect::<Option<_>>()?;
    Some(Record::Revdats(Revdats { revdat }))
}

/// Writes REVDAT lines, four modified record names per line.
pub(crate) fn write(revdats: &Revdats, out: &mut Vec<String>) {
    for revdat in &revdats.revdat {
        let modification_type = match revdat.modification_type {
            ModificationType::InitialRelease => 0,
            ModificationType::OtherModification => 1,
            ModificationType::UnknownModification(other) => other,
        };
        let details: Vec<_> = revdat.modification_detail.chunks(4).collect();
        for (i, details) in details
            .iter()
            .enumerate()
            .chain(details.is_empty().then_some((0, &&[][..])))
        {
            let mut line = LineBuilder::new("REVDAT").right(8, 10, revdat.modification_number);
            line = if i == 0 {
                line.left(14, &format_date(revdat.modification_date))
                    .left(24, &revdat.idcode)
            } else {
                line.right(11, 12, i + 1)
            };
            line = line.right(32, 32, modification_type);
            for (j, detail) in details.iter().enumerate() {
                line = line.left(40 + 7 * j, detail);
            }
            out.push(line.build());
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn revdat() {
        let r = single_record(
            "REVDAT   3   24-JAN-01 1BXO    3       ATOM
REVDAT   2   22-DEC-99 1BXO    4       HEADER COMPND REMARK JRNL
REVDAT   2 2                           ATOM   SOURCE SEQRES
REVDAT   1   14-OCT-98 1BXO    0
",
        );
        let Record::Revdats(r) = r else { panic!() };
        assert_eq!(r.revdat.len(), 3);
        assert_eq!(r.revdat[1].modification_number, 2);
        assert_eq!(
            r.revdat[1].modification_detail,
            ["HEADER", "COMPND", "REMARK", "JRNL", "ATOM", "SOURCE", "SEQRES"]
        );
        assert_eq!(r.revdat[2].modification_date.to_string(), "1998-10-14");
    }
}
