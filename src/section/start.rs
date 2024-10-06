use nom::IResult;

use crate::types::FuncIdx;

#[derive(Debug)]
pub struct StartSection {
    pub start_from: FuncIdx,
}

impl StartSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], StartSection> {
        let (input, func) = FuncIdx::parse(input)?;

        Ok((input, StartSection { start_from: func }))
    }
}
