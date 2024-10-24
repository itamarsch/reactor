use nom::{number::complete::u8, IResult};

use super::RefType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    Numeric(NumericValueType),
    // Vector(VectorType),
    Ref(RefType),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumericValueType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

#[derive(Debug, Clone, Copy)]
pub enum VectorType {
    V128 = 0x7B,
}

impl TryFrom<u8> for ValueType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        NumericValueType::try_from(value)
            .map(Self::Numeric)
            // .or_else(|_| VectorType::try_from(value).map(Self::Vector))
            .or_else(|_| RefType::try_from(value).map(Self::Ref))
    }
}

impl ValueType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ValueType> {
        let (input, value) = u8(input)?;
        Ok((input, value.try_into().expect("Invalid value type")))
    }
}

impl TryFrom<u8> for VectorType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x7B => Ok(VectorType::V128),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for NumericValueType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x7F => Ok(NumericValueType::I32),
            0x7E => Ok(NumericValueType::I64),
            0x7D => Ok(NumericValueType::F32),
            0x7C => Ok(NumericValueType::F64),
            _ => Err(()),
        }
    }
}
