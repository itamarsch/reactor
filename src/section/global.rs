use nom::{sequence::pair, IResult, Parser};

use crate::types::{wasm_vec, Expr, GlobalType};

#[derive(Debug)]
pub struct GlobalSection(pub Vec<GlobalInitializer>);

impl GlobalSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], GlobalSection> {
        let (input, globals) = wasm_vec(
            pair(GlobalType::parse, Expr::parse)
                .map(|(signature, init)| GlobalInitializer { signature, init }),
        )(input)?;

        Ok((input, GlobalSection(globals)))
    }
}

#[derive(Debug, Clone)]
pub struct GlobalInitializer {
    pub signature: GlobalType,
    pub init: Expr,
}
