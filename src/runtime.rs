use crate::{
    module::{self, functions::Function, Module},
    types::Instruction,
};

use self::{function_state::FunctionState, stack::Stack};

mod function_state;
pub mod local;
mod stack;
mod value;

pub struct Runtime<'a> {
    stack: Stack,
    module: Module<'a>,
    current_function_state: FunctionState,
}

impl<'a> Runtime<'a> {
    pub fn new(module: Module<'a>) -> Self {
        let (start_idx, Function::Local(starting_function)) = module.get_starting_function() else {
            panic!("Cannot start from imported function")
        };
        assert!(
            starting_function.signature.params.is_empty(),
            "_start function cannot take arguments"
        );

        Runtime {
            stack: Stack::new(),
            module,
            current_function_state: FunctionState {
                locals: vec![], // TODO
                instruction_index: 0,
                function_idx: start_idx,
            },
        }
    }

    pub fn execute(mut self) {
        loop {
            let Some(Function::Local(current_function)) = self
                .module
                .get_function(self.current_function_state.function_idx)
            else {
                unreachable!(
                    "Current runing function cannot be imported and its index has to exist"
                )
            };
            let instruction =
                &current_function.code.instructions[self.current_function_state.instruction_index];
            match instruction {
                Instruction::I32Const(value) => {
                    self.stack.push_i32(*value);
                }
                _ => panic!(
                    "Instruction: {:?} not implemented {:?}",
                    instruction, self.stack
                ),
            }
            self.current_function_state.instruction_index += 1;
            if self.current_function_state.instruction_index
                == current_function.code.instructions.len()
            {
                println!("Done");
                break;
            }
        }
    }
}
