use nom::{multi::count, IResult};
use nom_leb128::leb128_u32;

use crate::types::FuncTypeIdx;

#[derive(Debug)]
pub struct FunctionSection {
    pub functions: Vec<FuncTypeIdx>,
}

impl FunctionSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FunctionSection> {
        let (input, amount_of_functions) = leb128_u32(input)?;
        let (input, functions) = count(FuncTypeIdx::parse, amount_of_functions as usize)(input)?;
        Ok((input, FunctionSection { functions }))
    }
}
