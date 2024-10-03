use nom::{number::complete::u8, IResult};

#[derive(Debug, Clone, Copy)]
pub enum ValueType {
    I32 = 0x7F,
    I64 = 0x7E,
    F32 = 0x7D,
    F64 = 0x7C,
}

impl ValueType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ValueType> {
        let (input, value) = u8(input)?;
        Ok((input, value.try_into().expect("Invalid value type")))
    }
}

impl TryFrom<u8> for ValueType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x7F => Ok(ValueType::I32),
            0x7E => Ok(ValueType::I64),
            0x7D => Ok(ValueType::F32),
            0x7C => Ok(ValueType::F64),
            _ => Err(()),
        }
    }
}
