use nom::IResult;

use crate::types::{wasm_vec, FuncIdx, FuncTypeIdx};

#[derive(Debug)]
pub struct FunctionSection {
    functions: Vec<FuncTypeIdx>,
}

impl FunctionSection {
    pub fn get_func_type_idx(&self, FuncIdx(idx): FuncIdx) -> Option<FuncTypeIdx> {
        self.functions.get(idx as usize).copied()
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], FunctionSection> {
        let (input, functions) = wasm_vec(FuncTypeIdx::parse)(input)?;
        Ok((input, FunctionSection { functions }))
    }
}
