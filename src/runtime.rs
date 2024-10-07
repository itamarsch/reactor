use std::{
    borrow::{Borrow, BorrowMut},
    cell::{Cell, RefCell},
    ops::DerefMut,
    process::exit,
};

use crate::{
    module::{functions::Function, Module},
    runtime::locals::Locals,
    types::{FuncIdx, Instruction},
};

use self::{function_state::FunctionState, stack::Stack};

mod function_state;
mod local;
mod locals;
mod stack;
mod value;

pub struct Runtime<'a> {
    stack: RefCell<Stack>,
    module: Module<'a>,
    current_function_state: RefCell<FunctionState>,
    function_depth: Cell<usize>,
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

        let stack = RefCell::new(Stack::new());
        let initial_function_state = RefCell::new(FunctionState::new(
            Locals::new_no_function_parameters(&starting_function.code.locals),
            start_idx,
        ));

        Runtime {
            stack,
            module,
            current_function_state: initial_function_state,
            function_depth: Cell::new(0),
        }
    }

    fn wasi_function(&self, name: &str) {
        match name {
            "proc_exit" => {
                let exit_code = self.stack.borrow_mut().pop_i32();
                exit(exit_code);
            }
            _ => {
                panic!("Unknown wasi function: {}", name);
            }
        }
    }

    fn call_function(&self, func_idx: FuncIdx) {
        let next_function = self
            .module
            .get_function(func_idx)
            .expect("Call instruction to have a valid function index");
        match next_function {
            Function::Local(function) => {
                self.function_depth.set(self.function_depth.get() + 1);
                let locals = Locals::new(
                    &function.code.locals,
                    &function.signature.params,
                    self.stack.borrow_mut().deref_mut(),
                );

                let mut new_function_state = FunctionState::new(locals, func_idx);

                std::mem::swap(
                    &mut new_function_state,
                    self.current_function_state.borrow_mut().deref_mut(),
                );
                self.stack
                    .borrow_mut()
                    .push_function_state(new_function_state);
            }
            Function::Imported(function) => {
                if function.mod_name == "wasi_snapshot_preview1" {
                    self.wasi_function(function.name);
                }
            }
        }
    }

    pub fn execute(self) {
        loop {
            let Some(Function::Local(current_function)) = self
                .module
                .get_function(self.current_function_state.borrow().function_idx())
            else {
                unreachable!(
                    "Current runing function cannot be imported and its index has to exist"
                )
            };
            let instruction = &current_function.code.instructions
                [self.current_function_state.borrow().instruction_index()];
            println!(
                "Executing: {:?}, current state: {:?}, stack: {:?}",
                instruction,
                self.current_function_state,
                self.stack.borrow()
            );
            self.current_function_state.borrow_mut().next_instruction();
            match instruction {
                Instruction::I32Const(value) => {
                    self.stack.borrow_mut().push_i32(*value);
                }
                Instruction::I32Add => {
                    let a = self.stack.borrow_mut().pop_i32();
                    let b = self.stack.borrow_mut().pop_i32();
                    self.stack.borrow_mut().push_i32(a.wrapping_add(b));
                }
                Instruction::LocalGet(idx) => {
                    let value = self.current_function_state.borrow().get_value(*idx);
                    self.stack.borrow_mut().push_value(value);
                }
                Instruction::Drop => {
                    self.stack.borrow_mut().drop();
                }
                Instruction::Call(func_idx) => {
                    self.call_function(*func_idx);
                }
                _ => panic!(
                    "Instruction: {:?} not implemented {:?}",
                    instruction, self.stack
                ),
            }

            if self.current_function_state.borrow().instruction_index()
                == current_function.code.instructions.len()
            {
                if self.function_depth.get() > 0 {
                    let amount_of_returns = current_function.signature.returns.len();
                    let mut returns = Vec::with_capacity(amount_of_returns);
                    for _ in 0..amount_of_returns {
                        returns.push(self.stack.borrow_mut().pop_value());
                    }

                    let function_state = self.stack.borrow_mut().pop_function_state();
                    for _ in 0..amount_of_returns {
                        self.stack
                            .borrow_mut()
                            .push_value(returns.pop().expect("Pushed enough elements"));
                    }
                    *self.current_function_state.borrow_mut() = function_state;
                    self.function_depth.set(self.function_depth.get() - 1);
                } else {
                    break;
                }
            }
        }
    }
}
