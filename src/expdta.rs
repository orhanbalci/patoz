use super::entity::*;
use super::primitive::*;
use nom::character::complete::{line_ending, space0, space1};
use nom::{alt, do_parse, fold_many1, map, named, opt, separated_list, tag};

use crate::make_line_folder;

use std::marker::PhantomData;
use std::str;
use std::str::FromStr;

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

named!(
    experimental_technique_list_parser<Vec<ExperimentalTechnique>>,
    separated_list!(tag!(";"), experimental_technique_parser)
);

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
    expdata_parser<Vec<ExperimentalTechnique>>,
    map!(
        expdata_line_folder,
        |v: Vec<u8>| match experimental_technique_list_parser(v.as_slice()) {
            Ok((_, res)) => res,
            Err(_err) => Vec::new(),
        }
    )
);
