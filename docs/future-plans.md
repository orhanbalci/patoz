# Future plans

Ideas and decisions collected in September 2026, after the PDB parser, writer
and structure model were finished. None of this is scheduled yet; it records
where the project could go next and what was already decided.

## Decided direction

- **Audience: browser and web apps.** patoz aims to be a small, viewer
  independent structure analysis engine compiled to WebAssembly, usable from
  any web app, notebook or worker, and as a native Rust crate.
- **Edits are immutable.** Every edit returns a new `Structure`; selections
  made on the old structure stay valid for it.
- **mmCIF comes after query and edit features.** Until then bonds come from
  residue templates and distances.

Implications: a columnar atom layout (one array per property plus residue and
chain start indices) behind the current tree API, selections as sorted atom
index lists (`Uint32Array` in JS), built-in chemistry tables instead of the
full Chemical Component Dictionary, cargo features to keep the wasm bundle
small.

## Capability groups

Surveyed in Biopython, Biotite, gemmi, MDAnalysis, MDTraj, ProDy, PyMOL,
pdb-tools, PDBFixer, BioStructures.jl, PDBTools.jl, ProtoSyn.jl, chemfiles,
FreeSASA, libcifpp/DSSP and pdbtbx.

1. **Selection:** predicate selectors (backbone, water, acidic, helix...),
   a string selection language, spatial index (cell list).
2. **Geometry:** distances, angles, dihedrals, phi/psi/omega/chi, centre of
   mass, radius of gyration, contact maps, Kabsch superposition and RMSD.
3. **Annotation:** SASA (Shrake-Rupley), DSSP, hydrogen bonds, chain breaks,
   sequences with gaps, SEQRES to model alignment.
4. **Bonds:** bond graph from residue templates plus distances.
5. **Clean-up edits:** remove waters/hydrogens/ligands, pick alternate
   locations, renumber, rename, split and merge.
6. **Geometric edits:** transformations, biological assembly generation from
   REMARK 350 and MTRIX, setting torsion angles.
7. **Chemistry edits:** mutation, adding hydrogens and missing atoms. Needs
   chemistry data; lowest priority for the browser direction.

Dependencies: everything builds on atom addressing and the spatial index;
bonds unlock `bonded` queries, torsion editing, hydrogen bonds and mutation.

## Proposed milestones

| Milestone | Contents |
|---|---|
| M1 Foundation | columnar core with tree views, immutable edit plumbing, spatial index, JS API with typed arrays, benchmarks and bundle size budget |
| M2 Selection | the `molsel` crate below |
| M3 Geometry | group 2 |
| M4 Annotation | SASA, DSSP, hydrogen bonds |
| M5 Edits | groups 5 and 6, assemblies, writing back |
| M6 Bonds | bond graph, `bonded`, torsion editing |
| Later | mmCIF and PDB to mmCIF conversion (including branched sugar entities from LINK records), chemistry edits |

## molsel: a standalone selection crate

A query language crate independent of patoz's data model, published separately
(the names `molsel`, `atomsel` and `molselect` were free on crates.io). patoz
would be its first user; a second adapter (e.g. for pdbtbx) should validate the
trait before publishing. No existing Rust crate offers this: molar has VMD-like
selections tied to its own data model.

Lessons from VMD, MDAnalysis, ProDy, GROMACS, chemfiles, MDTraj, PyMOL, gemmi,
Mol* (MolQL) and molar:

- compile once, evaluate many times
- keep the surface syntax separate from a public, serde-serializable internal
  form (MolQL), so several syntaxes and JSON from JavaScript compile to it
- single-atom queries return sorted indices; multi-atom queries (chemfiles
  `pairs:`, `bonds:`) return fixed size matches
- parse errors carry offset and span for highlighting in a UI
- data sources implement optional capabilities (properties, grouping, bonds);
  molsel builds its own spatial index from positions
- separate coordinate dependent parts so the rest can be cached (GROMACS
  static/dynamic selections)
- plan evaluation: cheap filters first, distance conditions via the spatial
  index
- variables (`$ligand`), macros, `explain()` for the compiled plan

Sketch:

```rust
let sel = molsel::Selection::parse("protein and within 5 of $ligand")?;
let atoms: Vec<u32> = sel.atoms_with(&source, &Vars::new().set("ligand", &lig))?;

trait AtomSource {
    fn len(&self) -> usize;
    fn text(&self, p: TextProperty, i: usize) -> Option<&str>;
    fn number(&self, p: NumberProperty, i: usize) -> Option<f64>;
    fn position(&self, i: usize) -> [f64; 3];
}
trait Grouping { fn residue(&self, i: usize) -> usize; fn chain(&self, i: usize) -> usize; }
trait Bonding { fn bonded(&self, i: usize) -> &[u32]; }
```

Open questions: base dialect (MDAnalysis/VMD suggested), 0-based `index` with
PDB numbers as `serial`, case-insensitive keywords with case-sensitive values,
residue class tables with defaults in molsel that sources can override.

## SQL-like dialect

An alternative or additional front end, favoured for the web audience: mmCIF
is relational by design and SQL returns computed tables (per residue averages,
contact lists), not only atom sets. Prior art: duckdb-mmcif loads mmCIF into
DuckDB, but is a full database without molecular semantics or 3D spatial
joins.

```sql
name = 'CA' AND chain = 'A' AND resid BETWEEN 10 AND 50   -- shorthand for atoms WHERE

SELECT a.index, b.index, DISTANCE(a, b) AS d
FROM atoms a JOIN atoms b ON DISTANCE(a, b) < 4.0
WHERE a.chain = 'A' AND b.chain = 'B'

SELECT chain, resid, resname, AVG(bfactor) AS b FROM atoms
GROUP BY chain, resid, resname ORDER BY b DESC LIMIT 10

DELETE FROM atoms WHERE resname = 'HOH' OR is_hydrogen    -- returns a new Structure
```

Tables `atoms`, `residues`, `chains`, `bonds` as views over the columnar core.
Both the SQL and a VMD style front end would compile into one small relational
internal form (scan, filter, join, aggregate, project, delete, update) with a
planner that turns distance conditions into spatial index lookups. Not a
database: no storage, transactions or window functions; export atoms as Apache
Arrow for users who need full SQL in DuckDB.

Staging: 0.1 shorthand and `SELECT ... WHERE ... ORDER BY ... LIMIT` with
`DISTANCE` and `EXISTS`; 0.2 self joins, `GROUP BY`, bonds; 0.3 `DELETE` and
`UPDATE` as immutable edits, Arrow export. Main risk is scope creep: freeze the
supported subset per version.
