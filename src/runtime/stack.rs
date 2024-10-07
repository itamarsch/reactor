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
}

#[derive(Debug)]
pub enum StackValue {
    Value(Value),
    Function(FunctionState),
}
