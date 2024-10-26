use nom::{sequence::pair, IResult, Parser};

use crate::{
    module::functions::{Function, LocalFunction},
    types::{wasm_vec, Expr, FuncIdx, GlobalType},
};

#[derive(Debug)]
pub struct GlobalSection(pub Vec<GlobalInitializerDeclaration>);

impl GlobalSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], GlobalSection> {
        let (input, globals) = wasm_vec(
            pair(GlobalType::parse, Expr::parse)
                .map(|(signature, init)| GlobalInitializerDeclaration { signature, init }),
        )(input)?;

        Ok((input, GlobalSection(globals)))
    }
}

#[derive(Debug)]
pub struct GlobalInitializer {
    pub signature: GlobalType,
    pub init: FuncIdx,
}

#[derive(Debug)]
pub struct GlobalInitializerDeclaration {
    pub signature: GlobalType,
    pub init: Expr,
}

impl GlobalInitializerDeclaration {
    pub fn add_to_module(self, functions: &mut Vec<Function<'_>>) -> GlobalInitializer {
        let idx = FuncIdx(functions.len() as u32);
        functions.push(Function::Local(LocalFunction::expr(self.init)));
        GlobalInitializer {
            signature: self.signature,
            init: idx,
        }
    }
}
