use nom::IResult;
use nom_leb128::leb128_u32;

use super::{Limit, RefType};

#[derive(Debug, Clone, Copy)]
pub struct TableIdx(pub u32);
impl TableIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TableIdx> {
        leb128_u32(input).map(|(input, value)| (input, TableIdx(value)))
    }
}

#[derive(Debug)]
pub struct TableType(pub RefType, pub Limit);

impl TableType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TableType> {
        let (input, reftype) = RefType::parse(input)?;
        let (input, limit) = Limit::parse(input)?;
        Ok((input, TableType(reftype, limit)))
    }
}
