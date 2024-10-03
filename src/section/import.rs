use nom::{multi::count, IResult};
use nom_leb128::leb128_u32;

use crate::types::Import;

#[derive(Debug)]
pub struct ImportSection<'a>(pub Vec<Import<'a>>);

impl ImportSection<'_> {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ImportSection> {
        let (input, amount_of_imports) = leb128_u32(input)?;
        println!("Amount of imports: {amount_of_imports}");
        let (input, imports) = count(Import::parse, amount_of_imports as usize)(input)?;
        Ok((input, ImportSection(imports)))
    }
}
