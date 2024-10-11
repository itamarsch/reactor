use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::types::{BlockIdx, FuncIdx, LocalIdx};

use super::{locals::Locals, value::Value};

#[derive(Debug, Clone, Copy)]
pub enum InstructionIndex {
    IndexInFunction(usize),
    IndexInBlock(BlockIdx, usize),
}

#[derive(Debug)]
pub struct InstructionPosition(FuncIdx, InstructionIndex);

#[derive(Debug)]
pub struct FunctionState {
    locals: Rc<RefCell<Locals>>,
    instruction_position: InstructionPosition,
}

impl FunctionState {
    pub fn new_function(locals: Locals, index: FuncIdx) -> Self {
        Self {
            locals: Rc::new(RefCell::new(locals)),
            instruction_position: InstructionPosition(index, InstructionIndex::IndexInFunction(0)),
        }
    }

    pub fn new_block(&self, block_idx: BlockIdx) -> Self {
        Self {
            locals: self.locals.clone(),
            instruction_position: InstructionPosition(
                self.instruction_position.0,
                InstructionIndex::IndexInBlock(block_idx, 0),
            ),
        }
    }

    pub fn get_local_value(&self, idx: LocalIdx) -> Value {
        self.locals.deref().borrow().get_value(idx)
    }

    pub fn set_local_value(&mut self, idx: LocalIdx, value: Value) {
        self.locals.deref().borrow_mut().set_value(idx, value);
    }

    pub fn function_idx(&self) -> FuncIdx {
        self.instruction_position.0
    }

    pub fn instruction_index(&self) -> InstructionIndex {
        self.instruction_position.1
    }

    pub fn in_block(&self) -> bool {
        matches!(
            self.instruction_position.1,
            InstructionIndex::IndexInBlock(_, _)
        )
    }

    pub fn next_instruction(&mut self) {
        match &mut self.instruction_position.1 {
            InstructionIndex::IndexInFunction(ref mut i) => {
                *i += 1;
            }
            InstructionIndex::IndexInBlock(_, ref mut i) => {
                *i += 1;
            }
        }
    }
}
