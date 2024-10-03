use nom::{multi::count, IResult};
use nom_leb128::leb128_u32;

use crate::types::{wasm_vec, Import};

#[derive(Debug)]
pub struct ImportSection<'a>(pub Vec<Import<'a>>);

impl ImportSection<'_> {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ImportSection> {
        let (input, imports) = wasm_vec(Import::parse)(input)?;
        Ok((input, ImportSection(imports)))
    }
}
