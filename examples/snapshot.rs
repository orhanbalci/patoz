//! Writes parsed records of each given pdb file to `<out dir>/<name>.txt`
//! for comparing parser output between versions. Unrecognized lines are
//! summarized per record name.
//!
//! cargo run --example snapshot -- <out dir> <pdb files>
use std::{collections::BTreeMap, fmt::Write, path::Path};

fn main() {
    let mut args = std::env::args().skip(1);
    let out_dir = args.next().expect("usage: snapshot <out dir> <pdb files>");
    std::fs::create_dir_all(&out_dir).unwrap();
    for path in args {
        let content = std::fs::read_to_string(&path).unwrap();
        let mut out = String::new();
        match patoz::parse(&content) {
            Ok((_, pdb)) => {
                let mut unknown: BTreeMap<&str, usize> = BTreeMap::new();
                for record in pdb.records() {
                    match record {
                        patoz::Record::Unknown(l) => {
                            *unknown.entry(l.get(..6).unwrap_or(l).trim()).or_default() += 1
                        }
                        r => writeln!(out, "{:?}", r).unwrap(),
                    }
                }
                writeln!(out, "Unknown {:?}", unknown).unwrap();
            }
            Err(e) => writeln!(out, "Error {:?}", e).unwrap(),
        }
        let name = Path::new(&path).file_stem().unwrap().to_string_lossy();
        std::fs::write(Path::new(&out_dir).join(format!("{}.txt", name)), out).unwrap();
    }
}
