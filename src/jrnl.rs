/*!
Contains parsers related to [Jrnl](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#JRNL) records.

The JRNL record contains the primary literature citation that describes the experiment which resulted in the deposited coordinate set..
*/
use super::{ast::types::*, primitive::*};
use nom::{
    alt,
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map, map_res, named, opt, tag, take_str,
};

use crate::author::author_list_parser;

use std::{marker::PhantomData, str, str::FromStr};

use crate::make_line_folder;

#[allow(dead_code)]
struct JrnlAuthorLine;

#[allow(dead_code)]
struct JrnlTitleLine;

#[allow(dead_code)]
struct JrnlEditLine;

#[allow(dead_code)]
struct JrnlPublLine;

#[allow(dead_code)]
#[derive(Default, Clone)]
struct JrnlRefLine {
    continuation: u32,
    publication_name: String,
    volume: Option<u32>,
    page: Option<u32>,
    year: Option<u32>,
}

named!(
    pub (crate) issn<SerialNumber>,
    map_res!(tag!("ISSN"), |_| -> Result<SerialNumber, ()> { Ok(SerialNumber::Issn) })
);

named!(
    pub (crate) essn<SerialNumber>,
    map_res!(tag!("ESSN"), |_| -> Result<SerialNumber, ()> { Ok(SerialNumber::Essn) })
);

named!(
#[doc="parses serial number type as [SerialNumber](../ast/types/enum.SerialNumber.html)"],
    pub serial_number_type_parser<SerialNumber>, alt!(issn | essn));

named!(
    jrnl_author_line_parser<Continuation<JrnlAuthorLine>>,
    do_parse!(
        jrnl >> space1
            >> auth
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<JrnlAuthorLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(
    jrnl_author_line_folder,
    jrnl_author_line_parser,
    JrnlAuthorLine
);

named!(
    #[doc=r#"Parses AUTH sub-record of JRNL record. AUTH contains the list of authors associated
    with the cited article or contribution to a larger work. Formatted in a similar way with main
    AUTHOR record. If successfull return [Record](../ast/types/enum.Record.html) variant which
    contains [JournalAuthors](../ast/types/struct.JournalAuthors.html)

## Record Structure

| COLUMNS  | DATA  TYPE   | FIELD        | DEFINITION                           |
|----------|--------------|--------------|--------------------------------------|
| 1 -  6   | Record name  | JRNL         |                                      |
| 13 - 16  | LString(4)   | AUTH         | Appears on all continuation records. |
| 17 - 18  | Continuation | continuation | Allows  a long list of authors.      |
| 20 - 79  | List         | authorList   | List of the authors.                 |

"#],
    pub jrnl_author_record_parser<Record>,
    map!(jrnl_author_line_folder, |jrnl_author: Vec<u8>| {
        author_list_parser(jrnl_author.as_slice())
            .map(|res| Record::JournalAuthors(JournalAuthors{ authors: res.1 }))
            .expect("Can not parse journal author record")
    })
);

named!(
    jrnl_title_line_parser<Continuation<JrnlTitleLine>>,
    do_parse!(
        jrnl >> space1
            >> titl
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<JrnlTitleLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(
    jrnl_title_line_folder,
    jrnl_title_line_parser,
    JrnlTitleLine
);

named!(
    #[doc=r#"Parses TITLE sub-record of JRNL record. TITLE contains  title of a journal article,
    chapter, or part of a book. Is a continuation record which may span multi-lines. If successfull
    return [Record](../ast/types/enum.Record.html) variant which contains
    [JournalTitle](../ast/types/struct.JournalTitle.html)

## Record Structure

| COLUMNS  | DATA  TYPE    | FIELD           |    DEFINITION                        |
|----------|---------------|-----------------|--------------------------------------|
| 1 -  6   | Record name   | JRNL            |                                      |
| 13 - 16  | LString(4)    | TITL            | Appears on all continuation records. |
| 17 - 18  | Continuation  | continuation    | Permits long titles.                 |
| 20 - 79  | LString       | title           | Title of the article.                |

"#],
    pub jrnl_title_record_parser<Record>,
    map!(jrnl_title_line_folder, |jrnl_title: Vec<u8>| {
        Record::JournalTitle(JournalTitle{ title : String::from(str::from_utf8(jrnl_title.as_slice()).unwrap())})
    })
);

named!(
    jrnl_edit_line_parser<Continuation<JrnlEditLine>>,
    do_parse!(
        jrnl >> space1
            >> edit
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<JrnlEditLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(jrnl_edit_line_folder, jrnl_edit_line_parser, JrnlEditLine);

named!(
    #[doc=r#"Parses EDIT sub-record of JRNL record. EDIT appears if editors are associated
    with a non-journal reference. Formatted in a similar  way with AUTHOR record.
    If successfull return [Record](../ast/types/enum.Record.html) variant which
    contains [JournalEditors](../ast/types/struct.JournalEditors.html)

## Record Structure

| COLUMNS  | DATA  TYPE   | FIELD        | DEFINITION                           |
|----------|--------------|--------------|--------------------------------------|
| 1 -  6   | Record name  | JRNL         |                                      |
| 13 - 16  | LString(4)   | EDIT         | Appears on all continuation records. |
| 17 - 18  | Continuation | continuation | Allows  a long list of authors.      |
| 20 - 79  | List         | authorList   | List of the authors.                 |

"#],
    pub jrnl_edit_record_parser<Record>,
    map!(jrnl_edit_line_folder, |jrnl_edit: Vec<u8>| {
        if let Ok((_, res)) = author_list_parser(jrnl_edit.as_slice()) {
            Record::JournalEditors (JournalEditors{ name: res })
        } else {
            Record::JournalEditors(JournalEditors::default())
        }
    })
);

named!(
    jrnl_ref_line_parser<JrnlRefLine>,
    do_parse!(
        jrnl >> space1
            >> tag!("REF")
            >> space1
            >> cont: opt!(integer)
            >> publication_name: take_str!(28)
            >> space1
            >> opt!(take_str!(2))
            >> space1
            >> volume: opt!(integer)
            >> space1
            >> page: opt!(integer)
            >> space1
            >> year: opt!(integer)
            >> space0
            >> line_ending
            >> (JrnlRefLine {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                publication_name: publication_name.trim().to_owned(),
                volume,
                page,
                year,
            })
    )
);

named!(
    jrnl_ref_line_folder<JrnlRefLine>,
    fold_many1!(
        jrnl_ref_line_parser,
        JrnlRefLine::default(),
        |acc: JrnlRefLine, item: JrnlRefLine| {
            JrnlRefLine {
                continuation: acc.continuation,
                publication_name: acc.publication_name + &item.publication_name,
                page: acc.page.or(item.page),
                volume: acc.volume.or(item.volume),
                year: acc.year.or(item.year),
            }
        }
    )
);

named!(
    #[doc=r#"
Parses REF sub-record of JRNL record. REF is a group of fields that contain either the
publication status or the name of the publication (and any supplement and/or report information),
volume, page, and year. If successfull return [Record](../ast/types/enum.Record.html)variant which
contains [JournalReference](../ast/types/struct.JournalReference.html)

## Record Structure

| COLUMNS | DATA  TYPE     | FIELD          | DEFINITION                                  |
|---------|----------------|----------------|---------------------------------------------|
| 1 -  6  | Record name    | JRNL           |                                             |
| 13 - 16 | LString(3)     | REF            |                                             |
| 17 - 18 | Continuation   | continuation   | Allows long publication names.              |
| 20 - 47 | LString        | pubName        | Name  of the publication including section  |
|         |                |                | or series designation. This is the only     |
|         |                |                | field of this sub-record which may be       |
|         |                |                | continued on  successive sub-records.       |
| 50 - 51 | LString(2)     | V.             | Appears in the first sub-record only.       |
|         |                |                | and  only if column 55 is non-blank.        |
| 52 - 55 | String         | volume         | Right-justified blank-filled volume         |
|         |                |                | sub-record only.                            |
| 57 - 61 | String         | page           | First page of the article; appears in       |
|         |                |                | the first sub-record only.                  |
| 63 - 66 | Integer        | year           | Year of publication; first sub-record only. |
    "#],
    pub jrnl_ref_record_parser<Record>,
    map!(jrnl_ref_line_folder, |jrnl_ref : JrnlRefLine| { Record::JournalReference(JournalReference{
        publication_name : jrnl_ref.publication_name,
        volume : jrnl_ref.volume,
        page : jrnl_ref.page,
        year : jrnl_ref.year,
    })})
);

named!(
    jrnl_publ_line_parser<Continuation<JrnlPublLine>>,
    do_parse!(
        jrnl >> space1
            >> tag!("PUBL")
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<JrnlPublLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap().trim()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(jrnl_publ_line_folder, jrnl_publ_line_parser, JrnlPublLine);

named!(
    #[doc=r#"
Parses PUBL sub-record of JRNL record. PUBL contains the name of the publisher and
place of publication if the reference is to a book or other non-journal publication.
If successfull returns [Record](../ast/types/enum.Record.html) variant that contains
[JournalPublication](../ast/types/struct.JournalPublication.html)

## Record Structure



| COLUMNS  | DATA  TYPE     | FIELD          | DEFINITION                             |
|----------|----------------|----------------|----------------------------------------|
| 1 -  6   | Record name    | "JRNL          |                                        |
| 13 - 16  | LString(4)     | "PUBL"         |                                        |
| 17 - 18  | Continuation   | continuation   | Allows long publisher and place names. |
| 20 - 70  | LString        | pub            | City  of publication and name of the   |
|          |                |                | publisher/institution.                 |


    "#],
    pub  jrnl_publ_record_parser<Record>,
    map!(jrnl_publ_line_folder, |jrnl_publ: Vec<u8>| {
        Record::JournalPublication(JournalPublication{ publication : String::from(str::from_utf8(jrnl_publ.as_slice()).unwrap())})
    })
);

named!(
    #[doc=r#"
Parses REFN sub-record of JRNL record. REFN is a group of fields that contain encoded
references to the citation. No continuation lines are possible. Each piece of coded
information has a designated field.If successfull returns [Record](../ast/types/enum.Record.html)
variant that contains [JournalCitation](../ast/types/struct.JournalCitation.html)

## Record Structure

| COLUMNS   | DATA TYPE    | FIELD    | DEFINITION                                 |
|-----------|--------------|----------|--------------------------------------------|
| 1 -  6    | Record name  | JRNL     |                                            |
| 13 - 16   | LString(4)   | REFN     |                                            |
| 36 - 39   | LString(4)   | ISSN or  | International Standard Serial Number or    |
|           |              | ESSN     | Electronic Standard Serial Number.         |
| 41 - 65   | LString      | issn     | ISSN number (final digit may be a          |
|           |              |          | letter and may contain one or more dashes).|
    "#],
    pub jrnl_refn_record_parser<Record>,
    do_parse!(
        jrnl >> space1
            >> tag!("REFN")
            >> space1
            >> serial_type : opt!(serial_number_type_parser)
            >> space0
            >> serial : opt!(till_line_ending)
            >> line_ending
            >> (
                Record::JournalCitation(JournalCitation{
                    serial_type,
                    serial : serial.map(|s| String::from_str(str::from_utf8(s).unwrap().trim()).unwrap())
                })
            )
    )
);

named!(
    #[doc=r#"
Parses PMID sub-record of JRNL record. MID lists the PubMed unique
accession number of the publication related to the entry. If successfull returns
[Record](../ast/types/enum.Record.html) variant containing [JournalPubMedId](../ast/types/struct.JournalPubMedId.html)

## Record Structure

| COLUMNS  | DATA  TYPE   | FIELD        | DEFINITION                                  |
|----------|--------------|--------------|---------------------------------------------|
| 1 -  6   | Record  name | JRNL         |                                             |
| 13 - 16  | LString(4)   | PMID         |                                             |
| 20 – 79  | Integer      | continuation | unique PubMed identifier number assigned to |
|          |              |              | the publication  describing the experiment. |
|          |              |              | Allows  for a long PubMed ID number.        |

    "#],
    pub  jrnl_pmid_record_parser<Record>,
    do_parse!(
        jrnl >> space1
            >> tag!("PMID")
            >> space1
            >> pmid_id : integer
            >> space0
            >> line_ending
            >> (
                Record::JournalPubMedId(JournalPubMedId{
                    id : pmid_id,
                })
            )
    )
);

named!(
    #[doc=r#"
Parses DOI sub-record of JRNL record. DOI is the Digital Object Identifier
for the related electronic publication (“e-pub”), if applicable. If successfull
returns [Record](../ast/types/enum.Record.html) variant containing
[JournalDoi](../ast/types/struct.JournalDoi.html)

## Record Structure

| COLUMNS  | DATA  TYPE   | FIELD            | DEFINITION                             |
|----------|--------------|------------------|----------------------------------------|
| 1 -  6   | Record  name | JRNL             |                                        |
| 13 - 16  | LString(4)   | DOI              |                                        |
| 20 – 79  | LString      | continuation     | Unique DOI assigned to the publication |
|          |              |                  | describing the experiment.             |
|          |              |                  | Allows for a long DOI string.          |
    "#],
    pub jrnl_doi_record_parser<Record>,
    do_parse!(
        jrnl >> space1
            >> tag!("DOI")
            >> space1
            >> id : till_line_ending
            >> line_ending
            >> (
                Record::JournalDoi(JournalDoi{
                    id : String::from_str(str::from_utf8(id).unwrap().trim()).unwrap(),
                })
            )
    )
);

#[cfg(test)]
mod test {
    use super::{jrnl_refn_record_parser, jrnl_title_record_parser};
    use crate::ast::types::{JournalCitation, Record, SerialNumber};

    #[test]
    fn test_refn_parser() {
        let res = jrnl_refn_record_parser(
            r#"JRNL        REFN                   ISSN 0027-8424                                             
"#
            .as_bytes(),
        );

        match res {
            Ok((_, r)) => {
                if let Record::JournalCitation(JournalCitation {
                    serial_type: st,
                    serial: s,
                }) = r
                {
                    println!("{:?}", s);
                    assert_eq!(st.unwrap(), SerialNumber::Issn);
                }
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_jrnl_title() {
        let res = jrnl_title_record_parser(
            r#"JRNL        TITL   THE CRYSTAL STRUCTURE OF  HUMAN DEOXYHAEMOGLOBIN AT           
JRNL        TITL 2 1.74 A RESOLUTION      
"#
            .as_bytes(),
        );

        match res {
            Ok((_, r)) => {
                if let Record::JournalTitle(title) = r {
                    assert_eq!(
                        title.title,
                        "THE CRYSTAL STRUCTURE OF  HUMAN DEOXYHAEMOGLOBIN AT 1.74 A RESOLUTION"
                    );
                }
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
}
