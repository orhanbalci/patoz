use crate::{ast::types::*, primitive::*};

/// Parses a comma separated author list.
pub(crate) fn authors(text: &str) -> Option<Vec<Author>> {
    parse_all(list(','), text).map(|names| names.into_iter().map(Author).collect())
}

/// Parses continued [AUTHOR](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#AUTHOR) lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    let text = join_continued(lines.iter().map(|l| l.cols(11, 79)));
    Some(Record::Authors(Authors {
        authors: authors(&text)?,
    }))
}

/// Writes an author list in columns `text_col..=last_col` of continued
/// lines started by `line`.
pub(crate) fn write_authors(
    out: &mut Vec<String>,
    authors: &[Author],
    text_col: usize,
    last_col: usize,
    indent: bool,
    whole_units: bool,
    line: impl Fn(usize) -> LineBuilder,
) {
    let text = authors
        .iter()
        .map(|a| a.0.as_str())
        .collect::<Vec<_>>()
        .join(",");
    let mut wrapping = Wrap::list(Join::Text, ',');
    wrapping.whole_units = whole_units;
    write_wrapped(out, &text, text_col, last_col, indent, wrapping, line);
}

/// Writes AUTHOR lines.
pub(crate) fn write(authors: &Authors, out: &mut Vec<String>) {
    write_authors(out, &authors.authors, 11, 79, true, false, |n| {
        continued("AUTHOR", 9, 10, n)
    });
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Author, Record};

    #[test]
    fn author() {
        let r = single_record(
            "AUTHOR    C.JELSCH,M.M.TEETER,V.LAMZIN,V.PICHON-LESME,B.BLESSING,
AUTHOR   2 C.LECOMTE, D.VAN DER HELM
",
        );
        let Record::Authors(a) = r else { panic!() };
        assert_eq!(a.authors.len(), 7);
        assert_eq!(a.authors[0], Author("C.JELSCH".to_owned()));
        assert_eq!(a.authors[6], Author("D.VAN DER HELM".to_owned()));
    }
}
