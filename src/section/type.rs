use crate::types::{wasm_vec, FuncType};
use nom::IResult;

#[derive(Debug)]
pub struct TypeSection {
    pub funcs: Vec<FuncType>,
}

impl TypeSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TypeSection> {
        let (input, funcs) = wasm_vec(FuncType::parse)(input)?;

        Ok((input, TypeSection { funcs }))
    }
}
