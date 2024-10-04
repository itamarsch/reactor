use nom::{sequence::pair, IResult};
use nom_leb128::leb128_u32;

use crate::types::{wasm_vec, ValueType};

#[derive(Debug)]
pub struct Locals(pub Vec<ValueType>);

impl Locals {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Locals> {
        let (input, locals) = wasm_vec(pair(leb128_u32, ValueType::parse))(input)?;
        Ok((
            input,
            Locals(
                locals
                    .into_iter()
                    .flat_map(|(num, value_type)| std::iter::repeat(value_type).take(num as usize))
                    .collect(),
            ),
        ))
    }
}

#[derive(Debug)]
pub struct LocalIdx(pub u32);
impl LocalIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], LocalIdx> {
        leb128_u32(input).map(|(input, value)| (input, LocalIdx(value)))
    }
}
