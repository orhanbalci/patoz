use super::entity::*;
use nom::{alt, complete, fold_many0, named};

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
            | complete!(jrnl_ref_record_parser)
            | complete!(jrnl_publ_record_parser)
            | complete!(jrnl_refn_record_parser)
            | complete!(jrnl_pmid_record_parser)
            | complete!(jrnl_doi_record_parser)
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

    #[test]
    fn ejg_header_test() {
        if let Ok((_, res)) = pdb_records_parser(
            r#"HEADER    PLANT PROTEIN                           02-MAR-00   1EJG               
TITLE     CRAMBIN AT ULTRAHIGH RESOLUTION VALENCE ELECTRON DENSITY        
COMPND    MOL_ID: 1;                                                            
COMPND   2 MOLECULE: CRAMBIN (PRO22,SER22/LEU25,ILE25);                         
COMPND   3 CHAIN: A;                                                            
COMPND   4 FRAGMENT: CRAMBIN                                                    
SOURCE    MOL_ID: 1;                                                            
SOURCE   2 ORGANISM_SCIENTIFIC: CRAMBE HISPANICA SUBSP. ABYSSINICA;                                             
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
            println!("Length {}", res.len());
            println!("{:?}", res);
            assert!(true);
        } else {
            assert!(false)
        }
    }
}
