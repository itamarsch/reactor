use nom::IResult;

use crate::types::{wasm_vec, DataDeclaration};

#[derive(Debug)]
pub struct DataSection(pub Vec<DataDeclaration>);

impl DataSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DataSection> {
        wasm_vec(DataDeclaration::parse)(input).map(|(input, datas)| (input, DataSection(datas)))
    }
}
