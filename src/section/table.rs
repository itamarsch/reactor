use nom::IResult;

use crate::types::{wasm_vec, TableType};

#[derive(Debug)]
pub struct TableSection {
    pub tables: Vec<TableType>,
}

impl TableSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TableSection> {
        let (input, tables) = wasm_vec(TableType::parse)(input)?;
        Ok((input, TableSection { tables }))
    }
}
