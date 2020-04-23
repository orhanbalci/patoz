# Patoz
Protein Data Bank (pdb) file parser 

![Build Status](https://github.com/orhanbalci/rust-protein/workflows/CI/badge.svg)
![License](https://img.shields.io/github/license/orhanbalci/rust-protein.svg)

⚠️ WIP This is a work in progress. Expect breaking changes frequently. Right now use at your own risk


# Status
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
- [ ] [Dbref](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF)
- [ ] [Dbref1](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#DBREF1)
- [ ] [Seqadv](http://www.wwpdb.org/documentation/file-format-content/format33/sect3.html#SEQADV)
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
