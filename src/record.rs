use crate::{
    ast::{pdb_file::*, types::*},
    atom, author, caveat, compnd, connectivity, dbref, dbref1, expdta, header, heterogen, jrnl,
    keywds, master, mdltyp, modres, nummdl, obslte,
    primitive::Line,
    revdat, secondary, seqadv, seqres, source, split, sprsde, title,
};

/// Parses pdb file content. Every line ends up in a record: lines of record
/// types that are not supported yet, or that do not match the
/// specification, become [Record::Unknown].
pub fn parse(content: &str) -> PdbFile<Vec<Record>> {
    let lines: Vec<Line> = content.lines().map(Line).collect();
    let mut records = Vec::new();
    let mut rest = &lines[..];
    while !rest.is_empty() {
        let (group, next) = rest.split_at(group_len(rest));
        match parse_group(group) {
            Some(record) => records.push(record),
            None => records.extend(group.iter().map(|l| Record::Unknown(l.0.to_owned()))),
        }
        rest = next;
    }
    records.to_pdb_file()
}

/// Number of lines that belong to the record starting at `lines[0]`.
fn group_len(lines: &[Line]) -> usize {
    let first = lines[0];
    let name = first.record_name();
    let continues = |same: &dyn Fn(&Line) -> bool| {
        1 + lines[1..]
            .iter()
            .take_while(|l| l.record_name() == name && same(l))
            .count()
    };
    match name {
        "OBSLTE" | "TITLE" | "SPLIT" | "CAVEAT" | "COMPND" | "SOURCE" | "KEYWDS" | "EXPDTA"
        | "MDLTYP" | "AUTHOR" | "SPRSDE" | "REVDAT" => continues(&|_| true),
        "JRNL" => continues(&|l| l.cols(13, 16) == first.cols(13, 16)),
        "SEQRES" => continues(&|l| l.cols(12, 12) == first.cols(12, 12)),
        "SITE" | "HETNAM" | "HETSYN" => continues(&|l| l.cols(12, 14) == first.cols(12, 14)),
        "FORMUL" => continues(&|l| l.cols(13, 15) == first.cols(13, 15)),
        "DBREF1" if lines.get(1).is_some_and(|l| l.record_name() == "DBREF2") => 2,
        _ => 1,
    }
}

fn parse_group(lines: &[Line]) -> Option<Record> {
    let first = lines[0];
    match first.record_name() {
        "HEADER" => header::parse(first),
        "OBSLTE" => obslte::parse(lines),
        "TITLE" => title::parse(lines),
        "SPLIT" => split::parse(lines),
        "CAVEAT" => caveat::parse(lines),
        "COMPND" => compnd::parse(lines),
        "SOURCE" => source::parse(lines),
        "KEYWDS" => keywds::parse(lines),
        "EXPDTA" => expdta::parse(lines),
        "NUMMDL" => nummdl::parse(first),
        "MDLTYP" => mdltyp::parse(lines),
        "AUTHOR" => author::parse(lines),
        "REVDAT" => revdat::parse(lines),
        "SPRSDE" => sprsde::parse(lines),
        "JRNL" => jrnl::parse(lines),
        "REMARK" => Some(Record::Remark),
        "DBREF" => dbref::parse(first),
        "DBREF1" if lines.len() == 2 => dbref1::parse(first, lines[1]),
        "SEQADV" => seqadv::parse(first),
        "SEQRES" => seqres::parse(lines),
        "MODRES" => modres::parse(first),
        "MODEL" => atom::model(first),
        "ATOM" => atom::atom(first).map(Record::Atom),
        "HETATM" => atom::atom(first).map(Record::Hetatm),
        "ANISOU" => atom::anisou(first),
        "TER" => atom::ter(first),
        "ENDMDL" => Some(Record::Endmdl),
        "MASTER" => master::parse(first),
        "HET" => heterogen::het(first),
        "HETNAM" => heterogen::hetnam(lines),
        "HETSYN" => heterogen::hetsyn(lines),
        "FORMUL" => heterogen::formul(lines),
        "HELIX" => secondary::helix(first),
        "SHEET" => secondary::sheet(first),
        "SITE" => secondary::site(lines),
        "SSBOND" => connectivity::ssbond(first),
        "LINK" => connectivity::link(first),
        "CISPEP" => connectivity::cispep(first),
        "CONECT" => connectivity::conect(first),
        "END" => Some(Record::End),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn unknown_lines_are_kept() {
        let pdb = parse("HEADER    HYDROLASE                               20-APR-99   1CJY\nATOM      1  N\n\n");
        let [Record::Header(_), Record::Unknown(atom), Record::Unknown(blank)] = pdb.records()
        else {
            panic!()
        };
        assert_eq!(atom, "ATOM      1  N");
        assert_eq!(blank, "");
    }

    #[test]
    fn failed_group_falls_back_to_unknown_lines() {
        let pdb = parse("REVDAT   1   14-XXX-98 1BXO    0\nREVDAT   2   14-OCT-98 1BXO    1\n");
        assert_eq!(pdb.records().len(), 2);
        assert!(pdb
            .records()
            .iter()
            .all(|r| matches!(r, Record::Unknown(_))));
    }

    #[test]
    fn dbref1_without_dbref2_is_unknown() {
        let pdb = parse("DBREF1 1ABC A   61   322  UNIMES               UPI000148A153\n");
        assert!(matches!(pdb.records(), [Record::Unknown(_)]));
    }

    #[test]
    fn ejg_header() {
        let mut pdb = parse(
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
"#,
        );
        assert!(!pdb
            .records()
            .iter()
            .any(|r| matches!(r, Record::Unknown(_))));
        assert_eq!(pdb.records().len(), 12);
        assert_eq!(pdb.header().journal().pubmedid().unwrap().id, 10737790);
        assert_eq!(pdb.header().journal().authors().unwrap().authors.len(), 6);
    }

    fn parse_from_file(pdb_entry: &str) {
        use serde_json::Value;

        let res = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res");
        let contents = std::fs::read_to_string(res.join(format!("{}.pdb", pdb_entry))).unwrap();
        let expected = std::fs::read_to_string(res.join(format!("{}.exp", pdb_entry))).unwrap();
        let expected_val: Value = serde_json::from_str(&expected).unwrap();
        let mut pdb_parsed = parse(&contents);

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
        let journal_ref = pdb_parsed.header().journal().reference().unwrap();
        assert_eq!(
            expected_val["header.journal.reference.publication_name"],
            journal_ref.publication_name
        );
        assert_eq!(
            expected_val["header.journal.reference.volume"],
            journal_ref.volume.unwrap()
        );
        assert_eq!(
            expected_val["header.journal.reference.page"],
            journal_ref.page.unwrap()
        );
        assert_eq!(
            expected_val["primary.dbref.database"],
            pdb_parsed.primary().dbreference().unwrap().database
        );
    }

    /// MASTER numCoord is every ATOM and HETATM record in older files, as the
    /// specification says. Current wwPDB files count non-hydrogen atoms of
    /// the first model once, whatever their alternate locations.
    fn assert_master_counts(pdb_entry: &str) {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("res")
            .join(format!("{}.pdb", pdb_entry));
        let pdb = parse(&std::fs::read_to_string(path).unwrap());
        let master = pdb.master().unwrap();
        let coordinates = pdb.coordinates();
        let mut seen = std::collections::HashSet::new();
        let num_coord = coordinates
            .atoms()
            .chain(coordinates.hetero_atoms())
            .filter(|a| !matches!(a.element.as_deref(), Some("H") | Some("D")))
            .filter(|a| {
                seen.insert((
                    &a.name,
                    &a.residue_name,
                    a.chain_id,
                    a.residue_seq,
                    a.insertion_code,
                ))
            })
            .count();
        let count = |f: fn(&Record) -> bool| pdb.records().iter().filter(|r| f(r)).count();
        let all_coord = count(|r| matches!(r, Record::Atom(_) | Record::Hetatm(_)));
        assert!(
            [num_coord, all_coord].contains(&(master.num_coord as usize)),
            "numCoord {} matches neither {} nor {}",
            master.num_coord,
            num_coord,
            all_coord
        );
        assert_eq!(
            master.num_ter as usize,
            count(|r| matches!(r, Record::Ter(_)))
        );
        assert_eq!(
            master.num_het as usize,
            count(|r| matches!(r, Record::Het(_)))
        );
        assert_eq!(
            master.num_helix as usize,
            count(|r| matches!(r, Record::Helix(_)))
        );
        assert_eq!(
            master.num_sheet as usize,
            count(|r| matches!(r, Record::Sheet(_)))
        );
        assert_eq!(
            master.num_conect as usize,
            count(|r| matches!(r, Record::Conect(_)))
        );
        assert_eq!(
            master.num_remark as usize,
            count(|r| matches!(r, Record::Remark))
        );
    }

    #[test]
    fn master_counts() {
        for entry in ["1BXO", "1NLS", "1BYI"] {
            assert_master_counts(entry);
        }
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
