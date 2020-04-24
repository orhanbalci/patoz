use super::ast::{pdb_file::*, types::*};
use nom::{alt, complete, fold_many0, map, named};

use super::{
    author::author_record_parser,
    caveat::caveat_record_parser,
    compnd::cmpnd_token_parser,
    expdta::expdata_record_parser,
    header::header_parser,
    jrnl::{
        jrnl_author_record_parser, jrnl_doi_record_parser, jrnl_edit_record_parser,
        jrnl_pmid_record_parser, jrnl_publ_record_parser, jrnl_ref_record_parser,
        jrnl_refn_record_parser, jrnl_title_record_parser,
    },
    keywds::keywds_parser,
    mdltyp::mdltyp_record_parser,
    nummdl::nummdl_record_parser,
    obslte::obslte_record_parser,
    revdat::revdat_record_parser,
    source::source_token_parser,
    split::split_record_parser,
    sprsde::sprsde_record_parser,
    title::title_record_parser,
};

named!(
    pub pdb_record_parser<Record>,
    alt!(
        complete!(header_parser)
            | complete!(obslte_record_parser)
            | complete!(title_record_parser)
            | complete!(split_record_parser)
            | complete!(caveat_record_parser)
            | complete!(sprsde_record_parser)
            | complete!(cmpnd_token_parser)
            | complete!(source_token_parser)
            | complete!(keywds_parser)
            | complete!(expdata_record_parser)
            | complete!(nummdl_record_parser)
            | complete!(mdltyp_record_parser)
            | complete!(author_record_parser)
            | complete!(revdat_record_parser)
            | complete!(jrnl_author_record_parser)
            | complete!(jrnl_title_record_parser)
            | complete!(jrnl_edit_record_parser)
            | complete!(jrnl_refn_record_parser)
            | complete!(jrnl_ref_record_parser)
            | complete!(jrnl_publ_record_parser)
            | complete!(jrnl_pmid_record_parser)
            | complete!(jrnl_doi_record_parser)
    )
);

named!(
    pdb_records_parser<PdbFile<Vec<Record>>>,
    map!(
        fold_many0!(pdb_record_parser, Vec::new(), |mut acc, r: Record| {
            acc.push(r);
            acc
        }),
        |vr: Vec<Record>| vr.to_pdb_file()
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use std::{
        fs::File,
        io::{BufReader, Read},
        path::PathBuf,
    };

    #[test]
    fn header_parser() {
        let head = super::header_parser(
            "HEADER    PHOTOSYNTHESIS                          28-MAR-07   2UXK \n".as_bytes(),
        )
        .unwrap()
        .1;
        if let Record::Header(Header {
            classification: class,
            ..
        }) = head
        {
            assert_eq!(class, "PHOTOSYNTHESIS")
        } else {
            assert!(false);
        }
    }

    #[test]
    fn header_parser_2() {
        let head = super::header_parser(
            "HEADER    TRANSFERASE/TRANSFERASE                 28-MAR-07   2UXK \n".as_bytes(),
        )
        .unwrap()
        .1;
        if let Record::Header(Header {
            classification: class,
            ..
        }) = head
        {
            assert_eq!(class, "TRANSFERASE/TRANSFERASE")
        } else {
            assert!(false);
        }
    }

    #[test]
    fn obslte_parser() {
        let obs = obslte_record_parser("OBSLTE  02 31-JAN-94 1MBP      2MBP    \n".as_bytes())
            .unwrap()
            .1;

        if let Record::Obslte(Obslte {
            replacement_ids: reps,
            ..
        }) = obs
        {
            assert_eq!(reps[0], "1MBP");
        } else {
            assert!(false)
        }
    }

    #[test]
    fn title_parser() {
        let tit = title_record_parser(
            r#"TITLE     RHIZOPUSPEPSIN COMPLEXED WITH REDUCED PEPTIDE INHIBITOR
"#
            .as_bytes(),
        )
        .unwrap()
        .1;

        if let Record::Title(title) = tit {
            assert_eq!(
                title.title,
                "RHIZOPUSPEPSIN COMPLEXED WITH REDUCED PEPTIDE INHIBITOR"
            )
        } else {
            assert!(false)
        }
    }

    #[test]
    fn split_parser() {
        let splt = split_record_parser(
            "SPLIT      1VOQ 1VOR 1VOS 1VOU 1VOV 1VOW 1VOX 1VOY 1VP0 1VOZ \n".as_bytes(),
        )
        .unwrap()
        .1;

        if let Record::Split(split) = splt {
            assert_eq!(split.id_codes[0], "1VOQ")
        } else {
            assert!(false)
        }
    }

    #[test]
    fn pdb_records_parser() {
        if let Ok((_, mut res)) = super::pdb_records_parser(
            r#"HEADER    HYDROLASE                               20-APR-99   1CJY   
TITLE     HUMAN CYTOSOLIC PHOSPHOLIPASE A2
"#
            .as_bytes(),
        ) {
            if let Some(Header {
                classification: class,
                ..
            }) = &mut res.header().header()
            {
                assert_eq!(class, "HYDROLASE");
            }

            if let Some(tit) = &mut res.header().title() {
                assert_eq!(tit.title, "HUMAN CYTOSOLIC PHOSPHOLIPASE A2");
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn ejg_header() {
        if let Ok((_, mut res)) = super::pdb_records_parser(
            r#"HEADER    PLANT PROTEIN                           02-MAR-00   1EJG
TITLE     CRAMBIN AT ULTRAHIGH RESOLUTION VALENCE ELECTRON DENSITY
COMPND    MOL_ID: 1;
COMPND   2 MOLECULE: CRAMBIN (PRO22,SER22/LEU25,ILE25);
COMPND   3 CHAIN: A;
COMPND   4 FRAGMENT: CRAMBIN
SOURCE    MOL_ID: 1;
SOURCE   2 ORGANISM_SCIENTIFIC: CRAMBE HISPANICA SUBSP ABYSSINICA;
SOURCE   3 STRAIN: SUBSP ABYSSINICA
KEYWDS    VALENCE ELECTRON DENSITY, MULTI-SUBSTATE, MULTIPOLE REFINEMENT, PLANT
KEYWDS   2 PROTEIN
AUTHOR    C.JELSCH,M.M.TEETER,V.LAMZIN,V.PICHON-LESME,B.BLESSING,C.LECOMTE
JRNL        AUTH   C.JELSCH,M.M.TEETER,V.LAMZIN,V.PICHON-PESME,R.H.BLESSING,
JRNL        AUTH 2 C.LECOMTE
JRNL        TITL   ACCURATE PROTEIN CRYSTALLOGRAPHY AT ULTRA-HIGH RESOLUTION:
JRNL        TITL 2 VALENCE ELECTRON DISTRIBUTION IN CRAMBIN.
JRNL        REF    PROC.NATL.ACAD.SCI.USA        V.  97  3171 2000
JRNL        REFN                   ISSN 0027-8424
JRNL        PMID   10737790
JRNL        DOI    10.1073/PNAS.97.7.3171
"#
            .as_bytes(),
        ) {
            let pubmedid = &mut res.header().journal().pubmedid().unwrap();

            assert_eq!(pubmedid.id, 10737790);
        } else {
            assert!(false)
        }
    }

    fn get_test_file_path(file_name: &str) -> PathBuf {
        let mut current_file_path = PathBuf::from(file!());
        current_file_path.pop();
        current_file_path.pop();
        current_file_path.push("res");
        current_file_path.push(file_name);
        current_file_path
    }

    fn read_file(path: &PathBuf) -> String {
        let file = File::open(path).unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        if let Ok(_read_res) = buf_reader.read_to_string(&mut contents) {
            contents
        } else {
            "".to_owned()
        }
    }
    fn parse_from_file(pdb_entry: &str) {
        use serde_json::Value;

        let test_file_path = get_test_file_path(&format!("{}.pdb", pdb_entry));
        let expected_file_path = get_test_file_path(&format!("{}.exp", pdb_entry));
        let contents = read_file(&test_file_path);
        let expected = read_file(&expected_file_path);
        let expected_val: Value = serde_json::from_str(&expected).unwrap();
        let mut pdb_parsed = super::pdb_records_parser(contents.as_bytes()).unwrap().1;

        assert_eq!(
            expected_val["header.classification"],
            pdb_parsed.header().header().unwrap().classification
        );
        assert_eq!(
            expected_val["header.id_code"],
            pdb_parsed.header().header().unwrap().id_code
        );
        assert_eq!(
            expected_val["header.keywords"][0],
            pdb_parsed.header().keywds().unwrap().keywords[0]
        );
        assert_eq!(
            expected_val["header.title"],
            pdb_parsed.header().title().unwrap().title
        );
        assert_eq!(
            expected_val["header.experimental"][0]
                .as_str()
                .unwrap()
                .parse::<ExperimentalTechnique>()
                .unwrap(),
            pdb_parsed.header().expdta().unwrap().techniques[0]
        );

        assert_eq!(
            Author(
                expected_val["header.authors"][0]
                    .as_str()
                    .unwrap()
                    .to_owned()
            ),
            pdb_parsed.header().authors().unwrap().authors[0]
        );

        assert_eq!(
            Author(
                expected_val["header.journal.authors"][0]
                    .as_str()
                    .unwrap()
                    .to_owned()
            ),
            pdb_parsed.header().journal().authors().unwrap().authors[0]
        );

        assert_eq!(
            expected_val["header.journal.title"],
            pdb_parsed.header().journal().title().unwrap().title
        );

        assert_eq!(
            expected_val["header.journal.reference.publication_name"],
            pdb_parsed
                .header()
                .journal()
                .reference()
                .unwrap()
                .publication_name
        );

        assert_eq!(
            expected_val["header.journal.reference.volume"],
            pdb_parsed
                .header()
                .journal()
                .reference()
                .unwrap()
                .volume
                .unwrap()
        );

        assert_eq!(
            expected_val["header.journal.reference.page"],
            pdb_parsed
                .header()
                .journal()
                .reference()
                .unwrap()
                .page
                .unwrap()
        );
    }

    #[test]
    fn bxo() {
        parse_from_file("1BXO");
    }

    #[test]
    fn nls() {
        parse_from_file("1NLS")
    }

    #[test]
    fn byi() {
        parse_from_file("1BYI")
    }
}
