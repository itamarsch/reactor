use nom::IResult;

use crate::types::{wasm_vec, MemoryType};

#[derive(Debug)]
pub struct MemorySection {
    pub memories: Vec<MemoryType>,
}

impl MemorySection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MemorySection> {
        let (input, mems) = wasm_vec(MemoryType::parse)(input)?;
        assert_eq!(mems.len(), 1, "Only one memory supported");
        Ok((input, MemorySection { memories: mems }))
    }
}
