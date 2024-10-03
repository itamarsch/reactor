use nom::{multi::count, IResult};
use nom_leb128::leb128_u32;

use crate::types::MemoryType;

#[derive(Debug)]
pub struct MemorySection {
    pub memories: Vec<MemoryType>,
}

impl MemorySection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MemorySection> {
        let (input, amount_of_mems) = leb128_u32(input)?;
        let (input, mems) = count(MemoryType::parse, amount_of_mems as usize)(input)?;
        Ok((input, MemorySection { memories: mems }))
    }
}
