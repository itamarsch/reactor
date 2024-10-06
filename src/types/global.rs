use nom::{number::complete::u8, IResult};
use nom_leb128::leb128_u32;

use super::ValueType;

#[derive(Debug)]
pub struct GlobalIdx(pub u32);
impl GlobalIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], GlobalIdx> {
        leb128_u32(input).map(|(input, value)| (input, GlobalIdx(value)))
    }
}

#[derive(Debug)]
pub struct GlobalType {
    pub valtype: ValueType,
    pub mutability: Mutability,
}

impl GlobalType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], GlobalType> {
        let (input, value_type) = ValueType::parse(input)?;
        let (input, mutability) = Mutability::parse(input)?;
        Ok((
            input,
            GlobalType {
                valtype: value_type,
                mutability,
            },
        ))
    }
}

#[derive(Debug)]
pub enum Mutability {
    Mutable,
    Const,
}

impl Mutability {
    fn parse(input: &[u8]) -> IResult<&[u8], Mutability> {
        let (input, value) = u8(input)?;
        Ok((input, value.try_into().unwrap()))
    }
}

impl TryFrom<u8> for Mutability {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Const),
            0x01 => Ok(Self::Mutable),
            _ => Err(()),
        }
    }
}
