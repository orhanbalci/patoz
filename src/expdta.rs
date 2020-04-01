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
    experimental_technique_parser<ExperimentalTechnique>,
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
    pub (crate) expdata_record_parser<Record>,
    map!(expdata_line_folder, |v: Vec<u8>| {
        experimental_technique_list_parser(v.as_slice())
            .map(|res| Record::Experimental(Experimental { techniques: res.1 }))
            .expect("Can not parse expdta records")
    })
);
