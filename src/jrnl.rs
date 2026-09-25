/*!
Parses [JRNL](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#JRNL) records. Each sub record (AUTH, TITL,
EDIT, REF, PUBL, REFN, PMID, DOI) becomes its own [Record].
*/
use crate::{ast::types::*, author::authors, primitive::*};

/// Parses continued lines of one JRNL sub record.
pub(crate) fn parse(lines: &[Line]) -> Option<Record> {
    let first = lines[0];
    let text = || join_continued(lines.iter().map(|l| l.cols(20, 79)));
    Some(match first.text(13, 16) {
        "AUTH" => Record::JournalAuthors(JournalAuthors {
            authors: authors(&text())?,
        }),
        "TITL" => Record::JournalTitle(JournalTitle { title: text() }),
        "EDIT" => Record::JournalEditors(JournalEditors {
            name: authors(&text())?,
        }),
        "REF" => Record::JournalReference(JournalReference {
            publication_name: join_continued(lines.iter().map(|l| l.cols(20, 47))),
            volume: first.number(52, 55),
            page: first.number(57, 61),
            year: first.number(63, 66),
        }),
        "PUBL" => Record::JournalPublication(JournalPublication {
            publication: text(),
        }),
        "REFN" => Record::JournalCitation(JournalCitation {
            serial_type: match first.text(36, 39) {
                "ISSN" => Some(SerialNumber::Issn),
                "ESSN" => Some(SerialNumber::Essn),
                _ => None,
            },
            serial: Some(first.text(41, 65))
                .filter(|s| !s.is_empty())
                .map(str::to_owned),
        }),
        "PMID" => Record::JournalPubMedId(JournalPubMedId {
            id: first.number(20, 79)?,
        }),
        "DOI" => Record::JournalDoi(JournalDoi {
            id: first.text(20, 79).to_owned(),
        }),
        _ => return None,
    })
}

#[cfg(test)]
mod test {
    use crate::{test_util::single_record, Record, SerialNumber};

    #[test]
    fn refn() {
        let r = single_record("JRNL        REFN                   ISSN 0027-8424\n");
        let Record::JournalCitation(c) = r else {
            panic!()
        };
        assert_eq!(c.serial_type, Some(SerialNumber::Issn));
        assert_eq!(c.serial.as_deref(), Some("0027-8424"));
    }

    #[test]
    fn title() {
        let r = single_record(
            "JRNL        TITL   THE CRYSTAL STRUCTURE OF  HUMAN DEOXYHAEMOGLOBIN AT
JRNL        TITL 2 1.74 A RESOLUTION
",
        );
        let Record::JournalTitle(t) = r else { panic!() };
        assert_eq!(
            t.title,
            "THE CRYSTAL STRUCTURE OF  HUMAN DEOXYHAEMOGLOBIN AT 1.74 A RESOLUTION"
        );
    }

    #[test]
    fn reference() {
        let r =
            single_record("JRNL        REF    ACTA CRYSTALLOGR.,SECT.D      V.  54  1245 1998\n");
        let Record::JournalReference(r) = r else {
            panic!()
        };
        assert_eq!(r.publication_name, "ACTA CRYSTALLOGR.,SECT.D");
        assert_eq!(
            (r.volume, r.page, r.year),
            (Some(54), Some(1245), Some(1998))
        );
    }

    #[test]
    fn reference_to_be_published() {
        let r = single_record("JRNL        REF    TO BE PUBLISHED\n");
        let Record::JournalReference(r) = r else {
            panic!()
        };
        assert_eq!(r.publication_name, "TO BE PUBLISHED");
        assert_eq!(r.volume, None);
    }

    #[test]
    fn pmid_and_doi() {
        let Record::JournalPubMedId(p) = single_record("JRNL        PMID   10737790\n") else {
            panic!()
        };
        assert_eq!(p.id, 10737790);
        let Record::JournalDoi(d) = single_record("JRNL        DOI    10.1073/PNAS.97.7.3171\n")
        else {
            panic!()
        };
        assert_eq!(d.id, "10.1073/PNAS.97.7.3171");
    }
}
