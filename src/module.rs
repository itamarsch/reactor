use std::{collections::HashMap, rc::Rc};

use crate::{
    section::{Section, SectionType},
    types::{Data, Expr, FuncIdx, FuncType, FunctionCode, Limit, LocalTypes},
};

use self::{
    data::take_datas,
    functions::{take_functions, Function, LocalFunction},
    memory::take_memory_declaration,
    start::get_starting_function_index,
};

mod data;
pub mod functions;
mod globals;
mod memory;
mod start;

#[derive(Debug)]
pub struct Module<'a> {
    functions: Vec<Function<'a>>,
    datas: Vec<Data>,
    start: FuncIdx,
    memory: Limit,
}

impl<'t> Module<'t> {
    pub fn new(mut sections: HashMap<SectionType<'t>, Section<'t>>) -> Self {
        let functions = take_functions(&mut sections);
        let starting_point = get_starting_function_index(&mut sections)
            .expect("Wasi module expected to export a function _start");
        let memory = take_memory_declaration(&mut sections);
        let datas = take_datas(&mut sections);
        Self {
            datas,
            functions,
            start: starting_point,
            memory,
        }
    }

    pub fn memory_limit(&self) -> Limit {
        self.memory
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

    pub fn datas(&self) -> &[Data] {
        &self.datas
    }

    pub fn add_expr(&mut self, expr: Expr) -> FuncIdx {
        let idx = self.functions.len();
        self.functions
            .push(Function::Local(functions::LocalFunction {
                signature: Rc::new(FuncType {
                    params: vec![],
                    returns: vec![],
                }),
                code: FunctionCode {
                    locals: LocalTypes(vec![]),
                    instructions: expr,
                },
            }));
        FuncIdx(idx as u32)
    }
    pub fn remove_expr(&mut self, func_idx: FuncIdx) -> Expr {
        let Function::Local(LocalFunction { signature, code }) =
            self.functions.remove(func_idx.0 as usize)
        else {
            panic!("Can't remove function of the program only expressions used for, elements,globals, data")
        };
        assert!(
            signature.params.is_empty() && signature.returns.is_empty() && code.locals.0.is_empty(),
            "This is a function not an expression"
        );
        code.instructions
    }
}
