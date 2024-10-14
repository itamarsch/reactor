use std::{cell::RefCell, ops::Deref, rc::Rc};

use crate::{
    module::functions::LocalFunction,
    types::{BlockIdx, FuncIdx, LocalIdx},
};

use super::{locals::Locals, value::Value};

#[derive(Debug, Clone, Copy)]
pub enum InstructionIndex {
    IndexInFunction(usize),
    IndexInBlock {
        block_idx: BlockIdx,
        index_in_block: usize,
        is_loop: bool,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct InstructionPosition(FuncIdx, InstructionIndex);

#[derive(Debug, Clone)]
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

    pub fn new_block(&self, block_idx: BlockIdx, is_loop: bool) -> Self {
        Self {
            locals: self.locals.clone(),
            instruction_position: InstructionPosition(
                self.instruction_position.0,
                InstructionIndex::IndexInBlock {
                    block_idx,
                    index_in_block: 0,
                    is_loop,
                },
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
            InstructionIndex::IndexInBlock { .. }
        )
    }

    pub fn next_instruction(&mut self, code: &LocalFunction) {
        match &mut self.instruction_position.1 {
            InstructionIndex::IndexInFunction(ref mut i) => {
                *i += 1;
            }
            InstructionIndex::IndexInBlock {
                block_idx,
                index_in_block: ref mut i,
                is_loop,
            } => {
                *i += 1;
                if *is_loop {
                    let amount_of_instructions_in_block = code
                        .code
                        .instructions
                        .amount_of_instructions_in_block(*block_idx);
                    *i %= amount_of_instructions_in_block;
                }
            }
        }
    }
}
