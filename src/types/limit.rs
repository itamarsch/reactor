use nom::number::complete::u8;
use nom::IResult;
use nom_leb128::leb128_u32;

#[derive(Debug)]
pub struct Limit {
    pub min: u32,
    pub max: Option<u32>,
}

impl Limit {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Limit> {
        let (input, flag) = u8(input)?;
        let (input, min) = leb128_u32(input)?;
        let (input, max) = match flag {
            0x00 => (input, None),
            0x01 => {
                let (input, max) = leb128_u32(input)?;
                (input, Some(max))
            }
            _ => {
                panic!("Invalid limit flag")
            }
        };
        Ok((input, Limit { min, max }))
    }
}
