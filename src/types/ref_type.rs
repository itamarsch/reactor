use nom::{number::complete::u8, IResult};

#[derive(Debug, Clone, Copy)]
pub enum RefType {
    FuncRef,
    ExternRef,
}

impl RefType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], RefType> {
        let (input, value) = u8(input)?;
        let reftype = value.try_into().expect("Invalid reftype value");
        Ok((input, reftype))
    }
}

impl TryFrom<u8> for RefType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x70 => RefType::FuncRef,
            0x6F => RefType::ExternRef,
            _ => return Err(()),
        })
    }
}
