use crate::{section::Section, types::FuncIdx};

use self::functions::{take_functions, Function};

mod functions;

#[derive(Debug)]
pub struct Module<'a> {
    functions: Vec<Function<'a>>,
}

impl<'t> Module<'t> {
    pub fn new(mut sections: Vec<Section<'t>>) -> Self {
        let functions = take_functions(&mut sections);
        Self { functions }
    }
}
