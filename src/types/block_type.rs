use nom::{number::complete::u8, IResult};

use super::ValueType;

#[derive(Debug)]
pub struct BlockType(pub Option<ValueType>);

impl BlockType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], BlockType> {
        let (input, value) = u8(input)?;
        Ok((
            input,
            BlockType(match value {
                0x40 => None,
                v => Some(ValueType::try_from(v).expect("Valid blocktype valuetype")),
            }),
        ))
    }
}
