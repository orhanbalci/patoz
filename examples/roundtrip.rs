//! Writes parsed pdb files back and checks the result: parsing the written
//! file must give the same records, and per record type it reports how
//! many written lines are identical to the original lines.
//!
//! cargo run --release --example roundtrip -- path/to/*.pdb
use std::collections::BTreeMap;

fn record_name(line: &str) -> String {
    line.get(..6).unwrap_or(line).trim_end().to_owned()
}

/// lines grouped by record name, padded to 80 columns like written lines
fn lines_by_record(content: &str) -> BTreeMap<String, Vec<String>> {
    let mut lines: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in content.lines() {
        lines
            .entry(record_name(line))
            .or_default()
            .push(format!("{:<80}", line));
    }
    lines
}

fn main() {
    let mut identical: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let (mut files, mut failed) = (0, 0);
    for path in std::env::args().skip(1) {
        let content = std::fs::read_to_string(&path).unwrap();
        let pdb = patoz::parse(&content);
        let written = patoz::write(&pdb);
        let reparsed = patoz::parse(&written);
        files += 1;
        if reparsed.records() != pdb.records() {
            failed += 1;
            let first = pdb
                .records()
                .iter()
                .zip(reparsed.records())
                .find(|(a, b)| a != b);
            println!("{}: records differ, first difference {:#?}", path, first);
        }
        let original = lines_by_record(&content);
        let written = lines_by_record(&written);
        for (name, lines) in &original {
            let entry = identical.entry(name.clone()).or_default();
            entry.1 += lines.len();
            if let Some(written) = written.get(name) {
                entry.0 += lines.iter().zip(written).filter(|(a, b)| a == b).count();
            }
        }
    }
    println!(
        "{} files, {} with differing records after round trip",
        files, failed
    );
    println!("{:>8} {:>9} {:>7}  record", "lines", "identical", "");
    for (name, (same, total)) in identical {
        println!(
            "{:>8} {:>9} {:>6.1}%  {}",
            total,
            same,
            100.0 * same as f64 / total as f64,
            name
        );
    }
}
