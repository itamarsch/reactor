use core::panic;

use crate::types::BlockIdx;

use super::{
    function_state::{FunctionState, InstructionIndex},
    value::Value,
};

#[derive(Debug)]
pub struct Stack {
    stack: Vec<StackValue>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn push_bool(&mut self, value: bool) {
        let value = if value { 1 } else { 0 };
        self.push_i32(value);
    }

    pub fn push_i32(&mut self, value: i32) {
        self.stack.push(StackValue::Value(Value::I32(value)))
    }

    pub fn push_i64(&mut self, value: i64) {
        self.stack.push(StackValue::Value(Value::I64(value)))
    }

    pub fn push_f32(&mut self, value: f32) {
        self.stack.push(StackValue::Value(Value::F32(value)))
    }

    pub fn push_f64(&mut self, value: f64) {
        self.stack.push(StackValue::Value(Value::F64(value)))
    }

    pub fn pop_bool(&mut self) -> bool {
        let value = self.pop_i32();
        value != 0
    }

    pub fn pop_i32(&mut self) -> i32 {
        if let Some(StackValue::Value(Value::I32(value))) = self.stack.pop() {
            value
        } else {
            panic!("Tried popping i32 from stack but failed")
        }
    }

    pub fn pop_i64(&mut self) -> i64 {
        if let Some(StackValue::Value(Value::I64(value))) = self.stack.pop() {
            value
        } else {
            panic!("Tried popping i64 from stack but failed")
        }
    }

    pub fn pop_f32(&mut self) -> f32 {
        if let Some(StackValue::Value(Value::F32(value))) = self.stack.pop() {
            value
        } else {
            panic!("Tried popping f32 from stack but failed")
        }
    }

    pub fn pop_f64(&mut self) -> f64 {
        if let Some(StackValue::Value(Value::F64(value))) = self.stack.pop() {
            value
        } else {
            panic!("Tried popping f64 from stack but failed")
        }
    }

    pub fn push_value(&mut self, value: Value) {
        self.stack.push(StackValue::Value(value));
    }
    pub fn pop_value(&mut self) -> Value {
        if let Some(StackValue::Value(value)) = self.stack.pop() {
            value
        } else {
            panic!("Tried popping value from stack but failed")
        }
    }

    pub fn push_function_state(&mut self, function_state: FunctionState) {
        self.stack.push(StackValue::Function(function_state))
    }

    pub fn pop_function_state(&mut self) -> FunctionState {
        if let Some(StackValue::Function(value)) = self.stack.pop() {
            value
        } else {
            panic!("Tried popping value from stack but failed")
        }
    }

    pub fn drop(&mut self) {
        self.stack.pop();
    }

    pub fn break_from_block(&mut self, block_idx: BlockIdx) -> FunctionState {
        let mut found_block = false;
        loop {
            let pop = self.stack.pop().expect("Popped all stack while breaking");
            match pop {
                StackValue::Value(_) => {
                    continue;
                }
                StackValue::Function(f) => {
                    if !found_block {
                        match f.instruction_index() {
                            InstructionIndex::IndexInFunction(_) => {
                                panic!("Break out of function call not possible")
                            }
                            InstructionIndex::IndexInBlock {
                                block_idx: poped_block_index,
                                ..
                            } => {
                                found_block = poped_block_index == block_idx;
                            }
                        }
                    } else {
                        return f;
                    }
                }
            };
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum StackValue {
    Value(Value),
    Function(FunctionState),
}
