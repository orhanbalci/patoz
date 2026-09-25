/*!
Parses [JRNL](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#JRNL) records. Each sub record (AUTH, TITL,
EDIT, REF, PUBL, REFN, PMID, DOI) becomes its own [Record].
*/
use crate::{
    ast::types::*,
    author::{authors, write_authors},
    primitive::*,
};

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

fn jrnl(sub_record: &str, n: usize) -> LineBuilder {
    continued("JRNL", 17, 18, n).left(13, sub_record)
}

/// Writes a JRNL sub record.
pub(crate) fn write(record: &Record, out: &mut Vec<String>) {
    match record {
        Record::JournalAuthors(a) => {
            write_authors(out, &a.authors, 20, 79, false, true, |n| jrnl("AUTH", n))
        }
        Record::JournalEditors(e) => {
            write_authors(out, &e.name, 20, 79, false, true, |n| jrnl("EDIT", n))
        }
        Record::JournalTitle(t) => write_wrapped(
            out,
            &t.title,
            20,
            79,
            false,
            Wrap::TEXT.whole_units(),
            |n| jrnl("TITL", n),
        ),
        Record::JournalPublication(p) => write_wrapped(
            out,
            &p.publication,
            20,
            79,
            false,
            Wrap::TEXT.whole_units(),
            |n| jrnl("PUBL", n),
        ),
        Record::JournalReference(r) => {
            let start = out.len();
            write_wrapped(
                out,
                &r.publication_name,
                20,
                47,
                false,
                Wrap::TEXT.whole_units(),
                |n| jrnl("REF", n),
            );
            let mut first = LineBuilder::new("").left(1, &out[start]);
            if let Some(volume) = r.volume {
                first = first.left(50, "V.").right(52, 55, volume);
            }
            out[start] = first
                .right_opt(57, 61, r.page)
                .right_opt(63, 66, r.year)
                .build();
        }
        Record::JournalCitation(c) => {
            let serial_type = match c.serial_type {
                Some(SerialNumber::Issn) => "ISSN",
                Some(SerialNumber::Essn) => "ESSN",
                None => "",
            };
            out.push(
                jrnl("REFN", 1)
                    .left(36, serial_type)
                    .left(41, c.serial.as_deref().unwrap_or(""))
                    .build(),
            );
        }
        Record::JournalPubMedId(p) => out.push(jrnl("PMID", 1).left(20, &p.id.to_string()).build()),
        Record::JournalDoi(d) => out.push(jrnl("DOI", 1).left(20, &d.id).build()),
        _ => {}
    }
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
