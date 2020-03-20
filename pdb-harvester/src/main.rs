use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};

fn main() {
    println!("Welcome to PDB harvester");
    get_pdb_identifiers()
        .iter()
        .take(100)
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|s| {
            if !std::path::Path::new(&format!("./res/downloads/{}.pdb",s)).exists() {
                save_file(
                    &download_pdb_file(s),
                    std::path::Path::new(&format!("./res/downloads/{}.pdb", s)),
                )
                .expect("Can not save pdb file")
            }
        });
    //println!("{:?}", get_pdb_identifiers());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn save_file(s: &str, path: &Path) -> io::Result<()> {
    let mut f = File::create(path)?;
    write!(f, "{}", s)
}

fn download_pdb_file(pdb_entry: &str) -> String {
    println!("Downloading pdb {}", pdb_entry);
    let url = url::Url::parse(&format!(
        "https://files.rcsb.org/download/{}.pdb",
        pdb_entry
    ))
    .expect("Unable to parse rcsb url");
    reqwest::blocking::get(url.as_str())
        .expect("Can not download file")
        .text()
        .expect("Can not get body of response")
}

fn get_pdb_identifiers() -> Vec<String> {
    let mut result = Vec::new();
    if let Ok(lines) = read_lines("./res/pdb_select.txt") {
        for line in lines {
            if let Ok(l) = line {
                if !l.starts_with('#') {
                    result.push(
                        l.split_whitespace()
                            .skip(1)
                            .take(1)
                            .collect::<String>()
                            .chars()
                            .take(4)
                            .collect(),
                    );
                }
            }
        }
    }

    result
}
