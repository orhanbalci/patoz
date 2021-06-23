# Patoz
Protein Data Bank (pdb) file parser

![Build Status](https://github.com/orhanbalci/patoz/workflows/CI/badge.svg)
![License](https://img.shields.io/github/license/orhanbalci/patoz.svg)

⚠️ WIP This is a work in progress. Expect breaking changes frequently. Right now use at your own risk

# 📦 Cargo.toml
```
patoz = "0.1.0"
```
# 🔧 Examples
```rust

use std::{
    fs::File,
    io::{BufReader, Read},
    path::PathBuf,
};

use patoz::parse;

fn main() {
    let mut current_file_path = PathBuf::from(file!());
    current_file_path.pop();
    current_file_path.pop();
    current_file_path.push("1BYI.pdb");
    let content = read_file(&current_file_path);
    if let Ok((_, mut res)) = parse(&content) {
        println!(
            "Classification : {:?}",
            res.header().header().unwrap().classification
        );
        println!("Id Code : {:?}", res.header().header().unwrap().id_code);
        println!("Keywords : {:?}", res.header().keywds().unwrap().keywords);
    }
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
```
# 📊  Status
## Record Parser Status
### Title Section
- [x] [Header](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#HEADER)
- [x] [Obslte](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#OBSLTE)
- [x] [Title](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#TITLE)
- [x] [Splt](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SPLIT)
- [x] [Caveat](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#CAVEAT)
- [x] [Compnd](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#COMPND)
- [x] [Source](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SOURCE)
- [x] [Keywds](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#KEYWDS)
- [x] [Expdta](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#EXPDTA)
- [x] [Nummdl](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#NUMMDL)
- [x] [Mdltyp](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#MDLTYP)
- [x] [Author](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#AUTHOR)
- [x] [Sprsde](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#SPRSDE)
- [x] [Revdat](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#REVDAT)
- [x] [Jrnl](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#JRNL)
    - [x] Auth
    - [x] Titl
    - [x] Edit
    - [x] Ref
    - [x] Publ
    - [x] Refn
    - [x] Pmid
    - [x] Doi
- [ ] [Remarks](http://www.wwpdb.org/documentation/file-format-content/format33/remarks.html)
### Primary Structure Section
- [x] [Dbref](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF)
- [x] [Dbref1](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF1)
- [x] [Seqadv](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQADV)
- [x] [Seqres](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQRES)
- [ ] [Modres](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#MODRES)
### Heterogen Section
- [ ] [Het](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HET)
- [ ] [Formul](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#FORMUL)
- [ ] [Hetnam](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HETNAM)
- [ ] [Hetsyn](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HETSYN)
### Secondary Structure Section
- [ ] [Helix](http://www.wwpdb.org/documentation/file-format-content/format33/sect5.html#HELIX)
- [ ] [Sheet](http://www.wwpdb.org/documentation/file-format-content/format33/sect5.html#SHEET)
### Connectivity Annotation Section
- [ ] [Ssbond](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#SSBOND)
- [ ] [Link](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#LINK)
- [ ] [Cispep](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#CISPEP)
### Miscellaneous Features Section
- [ ] [Site](http://www.wwpdb.org/documentation/file-format-content/format33/sect7.html#SITE)
### Crystallographic and Coordinate Transformation Section
- [ ] [Cryst1](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#CRYST1)
- [ ] [MtrixN](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#MTRIXn)
- [ ] [OrigxN](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#ORIGXn)
- [ ] [ScaleN](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#SCALEn)
### Coordinate Section
- [ ] [Model](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#MODEL)
- [ ] [Atom](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ATOM)
- [ ] [Anisou](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU)
- [ ] [Ter](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#TER)
- [ ] [Hetatm](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#HETATM)
- [ ] [Endmdl](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ENDMDL)
### Connectivity Section
- [ ] [Conect](http://www.wwpdb.org/documentation/file-format-content/format33/sect10.html#CONECT)
### Bookkeeping Section
- [ ] [Master](http://www.wwpdb.org/documentation/file-format-content/format33/sect11.html#MASTER)
- [ ] [End](http://www.wwpdb.org/documentation/file-format-content/format33/sect11.html#END)

## 📝 License

Licensed under MIT License ([LICENSE](LICENSE)).

### 🚧 Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
