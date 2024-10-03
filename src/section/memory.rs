use nom::{multi::count, IResult};
use nom_leb128::leb128_u32;

use crate::types::{wasm_vec, MemoryType};

#[derive(Debug)]
pub struct MemorySection {
    pub memories: Vec<MemoryType>,
}

impl MemorySection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MemorySection> {
        let (input, mems) = wasm_vec(MemoryType::parse)(input)?;
        Ok((input, MemorySection { memories: mems }))
    }
}
