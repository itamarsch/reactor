use nom::IResult;

use crate::types::{wasm_vec, ElementDeclaration};

#[derive(Debug)]
pub struct ElementSection(pub Vec<ElementDeclaration>);
impl ElementSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ElementSection> {
        wasm_vec(ElementDeclaration::parse)(input)
            .map(|(input, elements)| (input, ElementSection(elements)))
    }
}
