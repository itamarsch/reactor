use nom::{sequence::pair, IResult};
use nom_leb128::leb128_u32;

#[derive(Debug)]
pub struct MemoryArgument {
    pub align: u32,
    pub offset: u32,
}

impl MemoryArgument {
    pub fn parse(input: &[u8]) -> IResult<&[u8], MemoryArgument> {
        pair(leb128_u32, leb128_u32)(input)
            .map(|(input, (align, offset))| (input, MemoryArgument { align, offset }))
    }
}
