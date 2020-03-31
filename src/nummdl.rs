/*!
Contains parsers related to [Nummdl](http://www.wwpdb.org/documentation/file-format-content/format33/sect2.html#NUMMDL) records
Nummdl record is a single line record having number of models inside the pdb entry. There is only one record per pdb.
*/
use super::{
    ast::types::{Nummdl, Record},
    primitive::*,
};
use nom::{character::complete::space0, do_parse, named};

named!(#[doc=r#"Parses sinle line of Nummdl record.
If succesfull returns [Record](../ast/types/enum.Record.html) variant containing [Nummdl](../ast/types/struct.Nummdl.html) instance.

Record structure is given below

| COLUMNS  | DATA TYPE   | FIELD       | DEFINITION        |               
|----------|-------------| ------------|-------------------|
|  1 -  6  | Record name | NUMMDL      |                   |                 
| 11 - 14  | Integer     | modelNumber | Number of models. | 
"#],
    pub nummdl_record_parser<Record>,
    do_parse!(nummdl >> space0 >> model_number: integer >> (Record::Nummdl (Nummdl{ num: model_number })))
);
