use std::{cell::RefCell, ops::DerefMut};

use crate::{
    module::{functions::Function, Module},
    types::{FuncIdx, Instruction, NumericValueType, ValueType},
};

use self::{function_state::FunctionState, local::Local, stack::Stack};

mod function_state;
pub mod local;
mod stack;
mod value;

pub struct Runtime<'a> {
    stack: RefCell<Stack>,
    module: Module<'a>,
    current_function_state: RefCell<FunctionState>,
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
            stack: RefCell::new(Stack::new()),
            module,
            current_function_state: RefCell::new(FunctionState {
                locals: vec![], // TODO
                instruction_index: 0,
                function_idx: start_idx,
            }),
        }
    }

    fn call_function(&self, func_idx: FuncIdx) {
        let next_function = self
            .module
            .get_function(func_idx)
            .expect("Call instruction to have a valid function index");
        match next_function {
            Function::Local(function) => {
                let locals = function.code.locals.0.iter().map(|e| match e {
                    ValueType::Numeric(NumericValueType::I32) => Local::from_i32_default(),
                    ValueType::Numeric(NumericValueType::I64) => todo!(),
                    ValueType::Numeric(NumericValueType::F32) => todo!(),
                    ValueType::Numeric(NumericValueType::F64) => todo!(),
                    ValueType::Ref(_) => todo!(),
                });

                let mut params = function
                    .signature
                    .params
                    .iter()
                    .map(|e| match e {
                        ValueType::Numeric(NumericValueType::I32) => {
                            Local::from_i32(self.stack.borrow_mut().pop_i32())
                        }
                        ValueType::Numeric(NumericValueType::I64) => todo!(),
                        ValueType::Numeric(NumericValueType::F32) => todo!(),
                        ValueType::Numeric(NumericValueType::F64) => todo!(),
                        ValueType::Ref(_) => todo!(),
                    })
                    .collect::<Vec<_>>();
                params.extend(locals);

                let mut new_function_state = FunctionState {
                    function_idx: func_idx,
                    instruction_index: 0,
                    locals: params,
                };

                std::mem::swap(
                    &mut new_function_state,
                    self.current_function_state.borrow_mut().deref_mut(),
                );
                self.stack
                    .borrow_mut()
                    .push_function_state(new_function_state);
            }
            Function::Imported(_) => todo!(),
        }
    }

    pub fn execute(self) {
        loop {
            let Some(Function::Local(current_function)) = self
                .module
                .get_function(self.current_function_state.borrow().function_idx)
            else {
                unreachable!(
                    "Current runing function cannot be imported and its index has to exist"
                )
            };
            let instruction = &current_function.code.instructions
                [self.current_function_state.borrow().instruction_index];
            println!(
                "Executing: {:?}, current state: {:?}",
                instruction, self.current_function_state
            );
            match instruction {
                Instruction::I32Const(value) => {
                    self.stack.borrow_mut().push_i32(*value);
                }
                Instruction::Call(func_idx) => {
                    self.call_function(*func_idx);
                    continue;
                }
                _ => panic!(
                    "Instruction: {:?} not implemented {:?}",
                    instruction, self.stack
                ),
            }
            self.current_function_state.borrow_mut().instruction_index += 1;
            if self.current_function_state.borrow().instruction_index
                == current_function.code.instructions.len()
            {
                println!("Done");
                break;
            }
        }
    }
}
