use super::{function_state::FunctionState, value::Value};

#[derive(Debug)]
pub struct Stack {
    stack: Vec<StackValue>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn push_i32(&mut self, value: i32) {
        self.stack.push(StackValue::Value(Value::I32(value)))
    }

    pub fn pop_i32(&mut self) -> i32 {
        if let Some(StackValue::Value(Value::I32(value))) = self.stack.pop() {
            value
        } else {
            panic!("Tried popping i32 from stack but failed")
        }
    }

    pub fn push_function_state(&mut self, function_state: FunctionState) {
        self.stack.push(StackValue::Function(function_state))
    }
}

#[derive(Debug)]
pub enum StackValue {
    Value(Value),
    Function(FunctionState),
}
