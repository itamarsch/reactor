use nom::IResult;
use nom_leb128::leb128_u32;

use super::Limit;

#[derive(Debug)]
pub struct MemoryIdx(pub u32);
impl MemoryIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MemoryIdx> {
        leb128_u32(input).map(|(input, value)| (input, MemoryIdx(value)))
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
