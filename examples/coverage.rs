//! Reports how many lines of given pdb files patoz recognizes and which
//! record types it does not support yet.
//!
//! cargo run --example coverage -- res/*.pdb
use std::collections::BTreeMap;

fn main() {
    let mut unknown: BTreeMap<String, usize> = BTreeMap::new();
    let (mut total, mut unrecognized) = (0, 0);
    for path in std::env::args().skip(1) {
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}: {}", path, e);
                continue;
            }
        };
        let pdb = patoz::parse(&content);
        total += content.lines().count();
        for record in pdb.records() {
            if let patoz::Record::Unknown(l) = record {
                unrecognized += 1;
                let name = l.get(..6).unwrap_or(l).trim().to_owned();
                *unknown.entry(name).or_default() += 1;
            }
        }
    }
    if total == 0 {
        eprintln!("usage: coverage <pdb files>");
        return;
    }
    println!(
        "{} lines, {} unrecognized, {:.1}% recognized",
        total,
        unrecognized,
        100.0 * (total - unrecognized) as f64 / total as f64
    );
    let mut by_count: Vec<_> = unknown.into_iter().collect();
    by_count.sort_by_key(|a| std::cmp::Reverse(a.1));
    for (name, count) in by_count {
        println!("{:>8}  {}", count, name);
    }
}
