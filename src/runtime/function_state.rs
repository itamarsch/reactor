use crate::types::{FuncIdx, LocalIdx};

use super::{locals::Locals, value::Value};

#[derive(Debug)]
pub struct FunctionState {
    locals: Locals,
    instruction_index: usize,
    function_idx: FuncIdx,
}

impl FunctionState {
    pub fn new(locals: Locals, index: FuncIdx) -> Self {
        Self {
            locals,
            function_idx: index,
            instruction_index: 0,
        }
    }

    pub fn get_local_value(&self, idx: LocalIdx) -> Value {
        self.locals.get_value(idx)
    }

    pub fn set_local_value(&mut self, idx: LocalIdx, value: Value) {
        self.locals.set_value(idx, value);
    }

    pub fn function_idx(&self) -> FuncIdx {
        self.function_idx
    }

    pub fn instruction_index(&self) -> usize {
        self.instruction_index
    }

    pub fn next_instruction(&mut self) {
        self.instruction_index += 1;
    }
}
