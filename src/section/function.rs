use nom::IResult;

use crate::types::{wasm_vec, FuncTypeIdx};

#[derive(Debug)]
pub struct FunctionSection {
    pub functions: Vec<FuncTypeIdx>,
}

impl FunctionSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FunctionSection> {
        let (input, functions) = wasm_vec(FuncTypeIdx::parse)(input)?;
        Ok((input, FunctionSection { functions }))
    }
}
