use std::collections::HashMap;

use crate::{
    section::{Section, SectionType},
    types::FuncIdx,
};

use self::functions::{take_functions, Function};

mod functions;
mod start;

#[derive(Debug)]
pub struct Module<'a> {
    functions: Vec<Function<'a>>,
}

impl<'t> Module<'t> {
    pub fn new(mut sections: HashMap<SectionType<'t>, Section<'t>>) -> Self {
        let functions = take_functions(&mut sections);
        Self { functions }
    }
}
