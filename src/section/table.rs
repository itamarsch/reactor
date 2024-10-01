use nom::{multi::count, IResult};
use nom_leb128::leb128_u32;

use crate::types::TableType;

#[derive(Debug)]
pub struct TableSection {
    pub tables: Vec<TableType>,
}

impl TableSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TableSection> {
        let (input, amount_of_tables) = leb128_u32(input)?;
        let (input, tables) = count(TableType::parse, amount_of_tables as usize)(input)?;
        Ok((input, TableSection { tables }))
    }
}
