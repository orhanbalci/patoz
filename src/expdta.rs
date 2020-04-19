/*!
Contains parsers related to [Expdta](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#EXPDTA) records.

The EXPDTA record identifies the experimental technique used. This may refer to the type of radiation and sample, or include the spectroscopic or modeling technique. Permitted values include:

- X-RAY  DIFFRACTION
- FIBER  DIFFRACTION
- NEUTRON  DIFFRACTION
- ELECTRON  CRYSTALLOGRAPHY
- ELECTRON  MICROSCOPY
- SOLID-STATE  NMR
- SOLUTION  NMR
- SOLUTION  SCATTERING
*/
use super::{ast::types::*, primitive::*};
use nom::{
    alt,
    bytes::complete::tag,
    character::complete::{line_ending, space0, space1},
    do_parse, fold_many1, map,
    multi::separated_list,
    named, opt, tag, IResult,
};

use crate::make_line_folder;

use std::{marker::PhantomData, str, str::FromStr};

#[allow(dead_code)]
struct ExpdataLine;
named!(
#[doc=r#"
parses single experimental technique. Returns
[ExperimentalTechnique](../enum.ExperimentalTechnique.html)
"#],
    pub experimental_technique_parser<ExperimentalTechnique>,
    alt!(
        do_parse!(
            space0
                >> tag!("X-RAY DIFFRACTION")
                >> space0
                >> (ExperimentalTechnique::XRayDiffraction)
        ) | do_parse!(
            space0
                >> tag!("FIBER DIFFRACTION")
                >> space0
                >> (ExperimentalTechnique::FiberDiffraction)
        ) | do_parse!(
            space0
                >> tag!("NEUTRON DIFFRACTION")
                >> space0
                >> (ExperimentalTechnique::NeutronDiffraction)
        ) | do_parse!(
            space0
                >> tag!("ELECTRON CRYSTALLOGRAPHY")
                >> space0
                >> (ExperimentalTechnique::ElectronCrystallography)
        ) | do_parse!(
            space0
                >> tag!("ELECTRON MICROSCOPY")
                >> space0
                >> (ExperimentalTechnique::ElectronMicroscopy)
        ) | do_parse!(
            space0 >> tag!("SOLID-STATE NMR") >> space0 >> (ExperimentalTechnique::SolidStateNmr)
        ) | do_parse!(
            space0 >> tag!("SOLUTION NMR") >> space0 >> (ExperimentalTechnique::SolutionNmr)
        ) | do_parse!(
            space0
                >> tag!("SOLUTION SCATTERING")
                >> space0
                >> (ExperimentalTechnique::SolutionScattering)
        )
    )
);

/// parses ; separated list of experimental techniques
pub fn experimental_technique_list_parser(s: &[u8]) -> IResult<&[u8], Vec<ExperimentalTechnique>> {
    separated_list(tag(";"), experimental_technique_parser)(s)
}
named!(
    expdata_line_parser<Continuation<ExpdataLine>>,
    do_parse!(
        expdta
            >> space1
            >> cont: opt!(integer)
            >> space0
            >> rest: till_line_ending
            >> line_ending
            >> (Continuation::<ExpdataLine> {
                continuation: if let Some(cc) = cont { cc } else { 0 },
                remaining: String::from_str(str::from_utf8(rest).unwrap()).unwrap(),
                phantom: PhantomData,
            })
    )
);

make_line_folder!(expdata_line_folder, expdata_line_parser, ExpdataLine);

named!(
#[doc=r#"
Parses EXPDTA records which is a continuation type of record which may span multi-lines.
Record contains list of `;` seperated experimental techniques. If seuccesfull returns [Record](../ast/types/enum.Record.html) variant containing [ExperimentalTechniques](../ast/types/struct.Experimental.html)

Record structure:

| COLUMNS | DATA TYPE     | FIELD        | DEFINITION                                |
|---------|---------------|--------------|-------------------------------------------|
| 1 -  6  | Record name   | EXPDTA       |                                           |
| 9 - 10  | Continuation  | continuation | Allows concatenation of multiple records. |
| 11 - 79 | SList         | technique    | The experimental technique(s) with        |
|         |                              | optional comment desc                     |
"#],
    pub expdata_record_parser<Record>,
    map!(expdata_line_folder, |v: Vec<u8>| {
        experimental_technique_list_parser(v.as_slice())
            .map(|res| Record::Experimental(Experimental { techniques: res.1 }))
            .expect("Can not parse expdta records")
    })
);
