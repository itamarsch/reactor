use nom::{multi::many0, IResult};
use nom_leb128::leb128_u32;

use crate::types::FuncType;

#[derive(Debug)]
pub struct TypeSection {
    pub funcs: Vec<FuncType>,
}

impl TypeSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TypeSection> {
        let (input, num_functions) = leb128_u32(input)?;

        let (input, funcs) = many0(FuncType::parse)(input)?;
        assert_eq!(num_functions as usize, funcs.len());

        Ok((input, TypeSection { funcs }))
    }
}
