use rayon::prelude::*;
use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
    process::Command,
    str,
    str::FromStr,
};

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("pdb-harvester")
        .version("1.0")
        .author("Orhan B. <orhanbalci@gmail.com>")
        .about("Harvests odb files online and transforms them")
        .subcommand(
            SubCommand::with_name("download")
                .about("Downloads pdb files from ncsb")
                .version("1.0")
                .author("Orhan B. <orhanbalci@gmail.com>")
                .arg(
                    Arg::with_name("count")
                        .short("c")
                        .help("Number of files to be downloaded"),
                ),
        )
        .subcommand(
            SubCommand::with_name("extract")
                .about("Extracts parts of files and stores them in other files")
                .version("1.0")
                .arg(
                    Arg::with_name("tag")
                        .short("d")
                        .help("Which tags will be extracted"),
                ),
        )
        .get_matches();

    if matches.is_present("download") {
        download_command();
    }

    if matches.is_present("extract") {}
}

fn download_command() {
    get_pdb_identifiers()
        .iter()
        .take(100)
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|s| {
            if !std::path::Path::new(&format!("./res/downloads/{}.pdb", s)).exists() {
                save_file(
                    &download_pdb_file(s),
                    std::path::Path::new(&format!("./res/downloads/{}.pdb", s)),
                )
                .expect("Can not save pdb file")
            }
        });
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

fn run_ripgrep(file_name: &str) -> String {
    let mut rg = Command::new("rg");
    rg.arg("-e")
        .arg("^HEADER")
        .arg("-e")
        .arg("^TITLE")
        .arg(file_name);
    let output = rg.output().ok().expect("Can not run rg");
    String::from_str(str::from_utf8(&output.stdout[..]).unwrap()).unwrap()
}
