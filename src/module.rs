use std::rc::Rc;

use crate::{
    parse_sections,
    section::{global::GlobalInitializer, r#type::TypeSection},
    types::{
        Data, Element, Expr, FuncIdx, FuncType, FuncTypeIdx, FunctionCode, Limit, LocalTypes,
        TableType,
    },
};

use self::{
    data::take_datas,
    elements::take_element_declarations,
    functions::{take_functions, Function, LocalFunction},
    globals::take_globals,
    memory::take_memory_declaration,
    start::get_starting_function_index,
    tables::take_table_declarations,
};

mod data;
mod elements;
pub mod functions;
mod globals;
mod memory;
mod start;
mod tables;

#[derive(Debug)]
pub struct Module<'a> {
    functions: Vec<Function<'a>>,
    function_types: TypeSection,
    elements: Option<Vec<Element>>,
    datas: Vec<Data>,
    globals: Vec<GlobalInitializer>,

    tables: Vec<TableType>,
    start: FuncIdx,
    memory: Limit,
}

impl<'t> Module<'t> {
    pub fn new(input: &'t [u8]) -> Self {
        let (_, mut sections) = parse_sections(input).unwrap();

        let (functions, function_types) = take_functions(&mut sections);
        let starting_point = get_starting_function_index(&mut sections)
            .expect("Wasi module expected to export a function _start");
        let memory = take_memory_declaration(&mut sections);
        let datas = take_datas(&mut sections);
        let globals = take_globals(&mut sections);
        let tables = take_table_declarations(&mut sections);
        let elements = take_element_declarations(&mut sections);

        Self {
            elements: Some(elements),
            globals,
            datas,
            functions,
            function_types,
            tables,
            start: starting_point,
            memory,
        }
    }

    pub fn elements(&mut self) -> Vec<Element> {
        let elements = self.elements.take();
        if let Some(elements) = elements {
            elements
        } else {
            panic!("Cannot take elements twice");
        }
    }

    pub fn function_signature(&self, idx: FuncTypeIdx) -> Option<Rc<FuncType>> {
        self.function_types.get_function_type(idx)
    }

    pub fn tables(&self) -> &[TableType] {
        &self.tables
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

    pub fn global_initializers(&self) -> &[GlobalInitializer] {
        &self.globals
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
