use nom::{multi::count, IResult};
use nom_leb128::leb128_u32;

use crate::types::Export;

#[derive(Debug)]
pub struct ExportSection<'a> {
    pub exports: Vec<Export<'a>>,
}

impl ExportSection<'_> {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ExportSection> {
        let (input, amount_of_exports) = leb128_u32(input)?;
        let (input, exports) = count(Export::parse, amount_of_exports as usize)(input)?;
        Ok((input, ExportSection { exports }))
    }
}
