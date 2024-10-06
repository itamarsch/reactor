use nom::IResult;

use crate::types::{wasm_vec, Data};

#[derive(Debug)]
pub struct DataSection(pub Vec<Data>);

impl DataSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DataSection> {
        wasm_vec(Data::parse)(input).map(|(input, datas)| (input, DataSection(datas)))
    }
}
