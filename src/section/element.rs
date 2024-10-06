use nom::IResult;

use crate::types::{wasm_vec, Element};

#[derive(Debug)]
pub struct ElementSection(pub Vec<Element>);
impl ElementSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ElementSection> {
        wasm_vec(Element::parse)(input).map(|(input, elements)| (input, ElementSection(elements)))
    }
}
