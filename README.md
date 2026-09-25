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
fn main() {
    let content = std::fs::read_to_string("res/1BYI.pdb").unwrap();
    let mut pdb = patoz::parse(&content);

    println!("Classification : {:?}", pdb.header().header().unwrap().classification);
    println!("Id Code : {:?}", pdb.header().header().unwrap().id_code);
    println!("Keywords : {:?}", pdb.header().keywds().unwrap().keywords);
}
```
Every line of the file ends up in a record. Lines of record types that are not supported yet, or that do not
match the specification, are kept as `Record::Unknown`. To see how much of your files patoz understands:
```
cargo run --example coverage -- path/to/*.pdb
```
# 🕸️ WebAssembly
`patoz-wasm` exposes the parser to JavaScript. Build it with [wasm-pack](https://rustwasm.github.io/wasm-pack/):
```
wasm-pack build patoz-wasm --target web
```
```js
import init, { parse } from "./pkg/patoz_wasm.js";

await init();
const { records } = parse(pdbText);
const header = records.find((r) => r.Header)?.Header;
console.log(header.classification, header.id_code);
```
Each record is an object keyed by its record type, e.g. `{ "Keywds": { "keywords": [...] } }`.
Lines of record types that are not supported yet come back as `{ "Unknown": "<line>" }`.

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
- [x] [Remarks](http://www.wwpdb.org/documentation/file-format-content/format33/remarks.html)
    - kept as text; REMARK 2 (resolution), 350 (biological assemblies) and 465 (missing residues) are also interpreted
### Primary Structure Section
- [x] [Dbref](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF)
- [x] [Dbref1](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF1)
- [x] [Seqadv](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQADV)
- [x] [Seqres](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQRES)
- [x] [Modres](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#MODRES)
### Heterogen Section
- [x] [Het](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HET)
- [x] [Formul](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#FORMUL)
- [x] [Hetnam](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HETNAM)
- [x] [Hetsyn](http://www.wwpdb.org/documentation/file-format-content/format33/sect4.html#HETSYN)
### Secondary Structure Section
- [x] [Helix](http://www.wwpdb.org/documentation/file-format-content/format33/sect5.html#HELIX)
- [x] [Sheet](http://www.wwpdb.org/documentation/file-format-content/format33/sect5.html#SHEET)
### Connectivity Annotation Section
- [x] [Ssbond](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#SSBOND)
- [x] [Link](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#LINK)
- [x] [Cispep](http://www.wwpdb.org/documentation/file-format-content/format33/sect6.html#CISPEP)
### Miscellaneous Features Section
- [x] [Site](http://www.wwpdb.org/documentation/file-format-content/format33/sect7.html#SITE)
### Crystallographic and Coordinate Transformation Section
- [x] [Cryst1](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#CRYST1)
- [x] [MtrixN](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#MTRIXn)
- [x] [OrigxN](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#ORIGXn)
- [x] [ScaleN](http://www.wwpdb.org/documentation/file-format-content/format33/sect8.html#SCALEn)
### Coordinate Section
- [x] [Model](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#MODEL)
- [x] [Atom](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ATOM)
- [x] [Anisou](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ANISOU)
- [x] [Ter](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#TER)
- [x] [Hetatm](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#HETATM)
- [x] [Endmdl](http://www.wwpdb.org/documentation/file-format-content/format33/sect9.html#ENDMDL)
### Connectivity Section
- [x] [Conect](http://www.wwpdb.org/documentation/file-format-content/format33/sect10.html#CONECT)
### Bookkeeping Section
- [x] [Master](http://www.wwpdb.org/documentation/file-format-content/format33/sect11.html#MASTER)
- [x] [End](http://www.wwpdb.org/documentation/file-format-content/format33/sect11.html#END)

## 📝 License

Licensed under MIT License ([LICENSE](LICENSE)).

### 🚧 Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the MIT license, shall be licensed as above, without any additional terms or conditions.
