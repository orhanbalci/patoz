use crate::{ast::types::*, primitive::*};

/// Parses continued [SPRSDE](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SPRSDE) lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    Some(Record::Sprsde(Sprsde {
        sprsde_date: parse_all(date, lines[0].text(12, 20))?,
        id_code: lines[0].text(22, 25).to_owned(),
        superseeded: lines.iter().flat_map(|l| l.id_codes(32, 9)).collect(),
    }))
}

/// Writes SPRSDE lines, nine superseded ids per line.
pub(crate) fn write(sprsde: &Sprsde, out: &mut Vec<String>) {
    let ids: Vec<_> = sprsde.superseeded.chunks(9).collect();
    for (i, ids) in ids
        .iter()
        .enumerate()
        .chain(ids.is_empty().then_some((0, &&[][..])))
    {
        let mut line = continued("SPRSDE", 9, 10, i + 1)
            .left(12, &format_date(sprsde.sprsde_date))
            .left(22, &sprsde.id_code);
        for (j, id) in ids.iter().enumerate() {
            line = line.left(32 + 5 * j, id);
        }
        out.push(line.build());
    }
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn sprsde() {
        let r = single_record("SPRSDE     17-JUL-84 4HHB      1HHB\n");
        let Record::Sprsde(s) = r else { panic!() };
        assert_eq!(s.sprsde_date.to_string(), "1984-07-17");
        assert_eq!(s.id_code, "4HHB");
        assert_eq!(s.superseeded, ["1HHB"]);
    }
}
