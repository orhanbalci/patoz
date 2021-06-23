/*!
Patoz is a strict, type safe PDB file parser. Converts text PDB file into a
traversable record struct.
 */
#![recursion_limit = "128"]

extern crate nom;

mod ast;
pub mod author;
pub mod caveat;
pub mod compnd;
pub mod dbref;
pub mod dbref1;
pub mod expdta;
pub mod header;
pub mod jrnl;
pub mod keywds;
pub mod mdltyp;
pub mod nummdl;
pub mod obslte;
pub mod primitive;
mod record;
pub mod remark;
pub mod revdat;
pub mod seqadv;
pub mod seqres;
pub mod source;
pub mod split;
pub mod sprsde;
pub mod title;

pub use ast::{pdb_file::*, types::*};
pub use nom::IResult;
pub use record::parse;
