use nom::{
    multi::{count, many0},
    IResult,
};
use nom_leb128::leb128_u32;

use crate::types::{wasm_vec, FuncType};

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
