use nom::{
    multi::{count, many0},
    IResult,
};
use nom_leb128::leb128_u32;

use crate::types::FuncType;

#[derive(Debug)]
pub struct TypeSection {
    pub funcs: Vec<FuncType>,
}

impl TypeSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TypeSection> {
        let (input, num_functions) = leb128_u32(input)?;

        let (input, funcs) = count(FuncType::parse, num_functions as usize)(input)?;
        assert_eq!(num_functions as usize, funcs.len());

        Ok((input, TypeSection { funcs }))
    }
}
