use nom::IResult;
use nom_leb128::leb128_u32;

use super::Limit;

#[derive(Debug)]
pub struct MemoryIdx(u32);
impl MemoryIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MemoryIdx> {
        let (input, memory_index) = leb128_u32(input)?;
        Ok((input, MemoryIdx(memory_index)))
    }
}

#[derive(Debug)]
pub struct MemoryType(pub Limit);

impl MemoryType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MemoryType> {
        let (input, limit) = Limit::parse(input)?;
        Ok((input, MemoryType(limit)))
    }
}
