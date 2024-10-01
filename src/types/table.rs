use nom::{bytes::complete::tag, IResult};

use super::Limit;

#[derive(Debug)]
pub struct TableType(pub Limit);

impl TableType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TableType> {
        let (input, _) = tag([0x70])(input)?;
        let (input, limit) = Limit::parse(input)?;
        Ok((input, TableType(limit)))
    }
}
