# Changelog

## 0.2.0

A near rewrite: patoz now parses every record type of the wwPDB format v3.3,
writes files back, and builds a structure model. Checked on 486 entries from
PDB select plus NMR and large entries against the files' MASTER records and
RCSB's mmCIF files.

### Added
- Parsers for all remaining record types: DBREF, DBREF1/DBREF2, SEQADV,
  MODRES, the coordinate section (ATOM,
  HETATM, ANISOU, TER, MODEL, ENDMDL), HELIX, SHEET, SITE, SSBOND, LINK,
  CISPEP, CONECT, HET, HETNAM, HETSYN, FORMUL, CRYST1, ORIGXn, SCALEn,
  MTRIXn, SIGATM, SIGUIJ, MASTER and END. REMARK lines keep their text.
- `patoz::write` turns records back into PDB content. 99.998% of lines of
  the 486 test entries are byte identical to the originals.
- `patoz::structure::Structure`, built with `PdbFile::structure`: metadata,
  entities, models, chain segments, residues and atoms. `Structure::to_pdb`
  converts it back.
- `PdbFile` accessors: `records`, `coordinates`, `master`, `resolution`,
  `missing_residues` (REMARK 465), `biological_assemblies` (REMARK 350).
- Optional `serde` feature deriving `Serialize` for all types.
- `patoz-wasm`, WebAssembly bindings with `parse` and `structure`
  (not published yet).
- Examples: `coverage`, `roundtrip`, `snapshot`.

### Changed
- `parse` returns `PdbFile` directly and never fails: lines that are not
  understood become `Record::Unknown` instead of stopping the parse.
- Records are read by the fixed columns of the specification. Text is no
  longer cut at parentheses, hyphens or colons, continued lines are joined
  as wwPDB wraps them, and blank insertion codes are `None`.
- Residue numbers are `i32`; SEQADV `sequence_number` is optional; DBREF
  database positions are `i32`.
- `Record::Remark` holds a `Remark` with the remark number and text.
- `ModificationType::UnknownModification` keeps its number.
- `Obslte` gains `id_code`; `Token` gains `Other` for unknown keys.
- Record parser modules are private; the public API is `parse`, `write`,
  the record types, `PdbFile` and `structure`.
- Two digit years are read as 19xx from 71 and 20xx below.
- nom 8, Rust edition 2021, minimum Rust version 1.77.

### Fixed
- Parsing no longer stops silently at the first unsupported record.
- SEQRES records are parsed (they were implemented but never used).
- Unexpected values no longer panic.

## 0.1.0

First release: title section records (HEADER, OBSLTE, TITLE, SPLIT, CAVEAT,
COMPND, SOURCE, KEYWDS, EXPDTA, NUMMDL, MDLTYP, AUTHOR, SPRSDE, REVDAT and
JRNL).
