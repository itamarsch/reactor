use std::rc::Rc;

use crate::types::{wasm_vec, FuncType, FuncTypeIdx};
use nom::IResult;

#[derive(Debug)]
pub struct TypeSection {
    funcs: Vec<Rc<FuncType>>,
}

impl TypeSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], TypeSection> {
        let (input, funcs) = wasm_vec(|input| {
            FuncType::parse(input).map(|(input, func_type)| (input, Rc::new(func_type)))
        })(input)?;

        Ok((input, TypeSection { funcs }))
    }

    pub fn get_function_type(&self, FuncTypeIdx(idx): FuncTypeIdx) -> Option<Rc<FuncType>> {
        self.funcs.get(idx as usize).map(|e| e.clone())
    }
}
