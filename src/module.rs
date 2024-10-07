use std::collections::HashMap;

use crate::{
    section::{Section, SectionType},
    types::FuncIdx,
};

use self::{
    functions::{take_functions, Function},
    start::get_starting_function_index,
};

pub mod functions;
mod start;

#[derive(Debug)]
pub struct Module<'a> {
    functions: Vec<Function<'a>>,
    start: FuncIdx,
}

impl<'t> Module<'t> {
    pub fn new(mut sections: HashMap<SectionType<'t>, Section<'t>>) -> Self {
        let functions = take_functions(&mut sections);
        let starting_point = get_starting_function_index(&mut sections)
            .expect("Wasi module expected to export a function _start");
        Self {
            functions,
            start: starting_point,
        }
    }

    pub fn get_starting_function(&self) -> (FuncIdx, &Function) {
        (
            self.start,
            self.get_function(self.start)
                .expect("starting index should be valid"),
        )
    }

    pub fn get_function(&self, FuncIdx(idx): FuncIdx) -> Option<&Function> {
        self.functions.get(idx as usize)
    }
}
