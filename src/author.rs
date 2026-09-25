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
