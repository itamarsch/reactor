use nom::IResult;

use super::Limit;

#[derive(Debug)]
pub struct MemoryType(pub Limit);

impl MemoryType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MemoryType> {
        let (input, limit) = Limit::parse(input)?;
        Ok((input, MemoryType(limit)))
    }
}
