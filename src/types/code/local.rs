use nom::{sequence::pair, IResult};
use nom_leb128::leb128_u32;

use crate::types::{wasm_vec, ValueType};

#[derive(Debug)]
pub struct LocalTypes(pub Vec<ValueType>);

impl LocalTypes {
    pub fn parse(input: &[u8]) -> IResult<&[u8], LocalTypes> {
        let (input, locals) = wasm_vec(pair(leb128_u32, ValueType::parse))(input)?;
        Ok((
            input,
            LocalTypes(
                locals
                    .into_iter()
                    .flat_map(|(num, value_type)| std::iter::repeat(value_type).take(num as usize))
                    .collect(),
            ),
        ))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LocalIdx(pub u32);
impl LocalIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], LocalIdx> {
        leb128_u32(input).map(|(input, value)| (input, LocalIdx(value)))
    }
}
