use nom::{bytes::complete::tag, IResult};
use nom_leb128::leb128_u32;

use super::Limit;

#[derive(Debug)]
pub struct TableIdx(u32);
impl TableIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TableIdx> {
        let (input, table_idx) = leb128_u32(input)?;
        Ok((input, TableIdx(table_idx)))
    }
}

#[derive(Debug)]
pub struct TableType(pub Limit);

impl TableType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TableType> {
        let (input, _) = tag([0x70])(input)?;
        let (input, limit) = Limit::parse(input)?;
        Ok((input, TableType(limit)))
    }
}
