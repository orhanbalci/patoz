use super::entity::*;
use nom::{alt, complete, fold_many0, named};

use super::{
    author::author_record_parser, caveat::caveat_record_parser, compnd::cmpnd_token_parser,
    expdta::expdata_record_parser, header::header_parser, keywds::keywds_parser,
    mdltyp::mdltyp_record_parser, nummdl::nummdl_record_parser, obslte::obslte_record_parser,
    source::source_token_parser, split::split_record_parser, sprsde::sprsde_record_parser,
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

    )
);

named!(
    pdb_records_parser<Vec<Record>>,
    fold_many0!(pdb_record_parser, Vec::new(), |mut acc, r: Record| {
        acc.push(r);
        acc
    })
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_header_parser() {
        let head = header_parser(
            "HEADER    PHOTOSYNTHESIS                          28-MAR-07   2UXK \n".as_bytes(),
        )
        .unwrap()
        .1;
        if let Record::Header {
            classification: class,
            ..
        } = head
        {
            assert_eq!(class, "PHOTOSYNTHESIS")
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_header_parser_2() {
        let head = header_parser(
            "HEADER    TRANSFERASE/TRANSFERASE                 28-MAR-07   2UXK \n".as_bytes(),
        )
        .unwrap()
        .1;
        if let Record::Header {
            classification: class,
            ..
        } = head
        {
            assert_eq!(class, "TRANSFERASE/TRANSFERASE")
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_obslte_parser() {
        let obs = obslte_record_parser("OBSLTE  02 31-JAN-94 1MBP      2MBP    \n".as_bytes())
            .unwrap()
            .1;

        if let Record::Obslte {
            replacement_ids: reps,
            ..
        } = obs
        {
            assert_eq!(reps[0], "1MBP");
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_title_parser() {
        let tit = title_record_parser(
            r#"TITLE     RHIZOPUSPEPSIN COMPLEXED WITH REDUCED PEPTIDE INHIBITOR
"#
            .as_bytes(),
        )
        .unwrap()
        .1;

        if let Record::Title { title } = tit {
            assert_eq!(
                title,
                "RHIZOPUSPEPSIN COMPLEXED WITH REDUCED PEPTIDE INHIBITOR"
            )
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_split_parser() {
        let splt = split_record_parser(
            "SPLIT      1VOQ 1VOR 1VOS 1VOU 1VOV 1VOW 1VOX 1VOY 1VP0 1VOZ \n".as_bytes(),
        )
        .unwrap()
        .1;

        if let Record::Split { id_codes } = splt {
            assert_eq!(id_codes[0], "1VOQ")
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_pdb_records_parser() {
        if let Ok((_, res)) = pdb_records_parser(
            r#"HEADER    HYDROLASE                               20-APR-99   1CJY
TITLE     HUMAN CYTOSOLIC PHOSPHOLIPASE A2
"#
            .as_bytes(),
        ) {
            if let Record::Header {
                classification: class,
                ..
            } = &res[0]
            {
                assert_eq!(class, "HYDROLASE");
            }

            if let Record::Title { title: tit } = &res[1] {
                assert_eq!(tit, "HUMAN CYTOSOLIC PHOSPHOLIPASE A2");
            }
        } else {
            assert!(false);
        }
    }
}
