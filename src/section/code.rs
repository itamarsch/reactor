use nom::IResult;

use crate::types::{wasm_vec, FunctionCode};

#[derive(Debug)]
pub struct CodeSection {
    pub functions: Vec<FunctionCode>,
}
impl CodeSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], CodeSection> {
        let (input, functions) = wasm_vec(FunctionCode::parse)(input)?;
        Ok((input, CodeSection { functions }))
    }
}
