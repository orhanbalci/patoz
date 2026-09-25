use crate::{ast::types::*, primitive::*};

/// Parses continued [TITLE](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#TITLE) lines.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    Some(Record::Title(Title {
        title: join_continued(lines.iter().map(|l| l.cols(11, 80))),
    }))
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record};

    #[test]
    fn continued_title() {
        let r = single_record(
            "TITLE     HUMAN DIHYDROFOLATE REDUCTASE COMPLEXED WITH NADPH AND (Z)-6-(2-[2,5-
TITLE    2 DIMETHOXYPHENYL]ETHEN-1-YL)
",
        );
        let Record::Title(t) = r else { panic!() };
        assert_eq!(
            t.title,
            "HUMAN DIHYDROFOLATE REDUCTASE COMPLEXED WITH NADPH AND (Z)-6-(2-[2,5-DIMETHOXYPHENYL]ETHEN-1-YL)"
        );
    }
}
