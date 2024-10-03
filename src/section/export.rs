use nom::IResult;

use crate::types::{wasm_vec, Export};

#[derive(Debug)]
pub struct ExportSection<'a> {
    pub exports: Vec<Export<'a>>,
}

impl ExportSection<'_> {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ExportSection> {
        let (input, exports) = wasm_vec(Export::parse)(input)?;
        Ok((input, ExportSection { exports }))
    }
}
