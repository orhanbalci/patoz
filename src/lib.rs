/*!
Patoz is a strict, type safe PDB file parser. Converts text PDB file into a
traversable record struct.

```
let mut pdb = patoz::parse("HEADER    PLANT PROTEIN                           02-MAR-00   1EJG\n");
assert_eq!(pdb.header().header().unwrap().id_code, "1EJG");
```
 */
mod ast;
mod atom;
mod author;
mod caveat;
mod compnd;
mod connectivity;
mod crystal;
mod dbref;
mod dbref1;
mod expdta;
mod header;
mod heterogen;
mod jrnl;
mod keywds;
mod master;
mod mdltyp;
mod modres;
mod nummdl;
mod obslte;
mod primitive;
mod record;
mod remark;
mod revdat;
mod secondary;
mod seqadv;
mod seqres;
mod source;
mod split;
mod sprsde;
mod title;

pub use ast::{pdb_file::*, types::*};
pub use record::{parse, write};

#[cfg(test)]
mod test_util {
    use crate::Record;

    /// Parses `content` which must contain exactly one record.
    pub fn single_record(content: &str) -> Record {
        let pdb = crate::parse(content);
        match pdb.records() {
            [record] => record.clone(),
            records => panic!("expected one record, got {:?}", records),
        }
    }
}
