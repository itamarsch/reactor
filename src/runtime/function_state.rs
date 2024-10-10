use crate::types::{FuncIdx, LocalIdx};

use super::{locals::Locals, value::Value};

#[derive(Debug, Clone, Copy)]
pub struct InstructionIndex(pub usize);
#[derive(Debug)]
pub struct InstructionPosition(FuncIdx, InstructionIndex);

#[derive(Debug)]
pub struct FunctionState {
    locals: Locals,
    instruction_position: InstructionPosition,
}

impl FunctionState {
    pub fn new(locals: Locals, index: FuncIdx) -> Self {
        Self {
            locals,
            instruction_position: InstructionPosition(index, InstructionIndex(0)),
        }
    }

    pub fn get_local_value(&self, idx: LocalIdx) -> Value {
        self.locals.get_value(idx)
    }

    pub fn set_local_value(&mut self, idx: LocalIdx, value: Value) {
        self.locals.set_value(idx, value);
    }

    pub fn function_idx(&self) -> FuncIdx {
        self.instruction_position.0
    }

    pub fn instruction_index(&self) -> InstructionIndex {
        self.instruction_position.1
    }

    pub fn next_instruction(&mut self) {
        self.instruction_position.1 .0 += 1;
    }
}
