use std::rc::Rc;

use crate::{
    parse_sections,
    section::{global::GlobalInitializer, r#type::TypeSection},
    types::{Data, Element, FuncIdx, FuncType, FuncTypeIdx, FunctionCode, Limit, TableType},
};

use self::{
    data::take_datas,
    elements::take_element_declarations,
    functions::{take_functions, Function},
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
    elements: Vec<Element>,
    datas: Vec<Data>,
    globals: Vec<GlobalInitializer>,

    tables: Vec<TableType>,
    start: FuncIdx,
    memory: Limit,
}

impl<'t> Module<'t> {
    pub fn new(input: &'t [u8]) -> Self {
        let (_, mut sections) = parse_sections(input).unwrap();

        let (mut functions, function_types) = take_functions(&mut sections);

        let starting_point = get_starting_function_index(&mut sections)
            .expect("Wasi module expected to export a function _start");
        let memory = take_memory_declaration(&mut sections);

        let datas = take_datas(&mut sections, &mut functions);

        let globals = take_globals(&mut sections, &mut functions);
        let tables = take_table_declarations(&mut sections);
        let elements = take_element_declarations(&mut sections, &mut functions);

        Self {
            elements,
            globals,
            datas,
            functions,
            function_types,
            tables,
            start: starting_point,
            memory,
        }
    }

    pub fn elements(&self) -> &[Element] {
        &self.elements
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
}
