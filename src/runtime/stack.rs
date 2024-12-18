use core::panic;

use crate::types::{BlockIdx, NumericValueType, ValueType};

use super::{
    function_state::{FunctionState, InstructionIndex},
    table::TableElementIdx,
    value::{Ref, Value},
};

#[derive(Debug)]
pub struct Stack {
    stack: Vec<StackValue>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push_bool(&mut self, value: bool) {
        let value = if value { 1 } else { 0 };
        self.push_i32(value);
    }

    pub fn push_i32(&mut self, value: i32) {
        self.stack.push(StackValue::Value(Value::I32(value)))
    }

    pub fn push_u32(&mut self, value: u32) {
        self.push_i32(i32::from_le_bytes(value.to_le_bytes()))
    }

    pub fn push_i64(&mut self, value: i64) {
        self.stack.push(StackValue::Value(Value::I64(value)))
    }

    pub fn push_u64(&mut self, value: u64) {
        self.push_i64(i64::from_le_bytes(value.to_le_bytes()))
    }

    pub fn push_f32(&mut self, value: f32) {
        self.stack.push(StackValue::Value(Value::F32(value)))
    }

    pub fn push_f64(&mut self, value: f64) {
        self.stack.push(StackValue::Value(Value::F64(value)))
    }

    pub fn push_ref(&mut self, value: Ref) {
        self.stack.push(StackValue::Value(Value::Ref(value)))
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

    pub fn pop_table_element_idx(&mut self) -> TableElementIdx {
        TableElementIdx(self.pop_u32() as usize)
    }

    pub fn pop_u32(&mut self) -> u32 {
        u32::from_le_bytes(self.pop_i32().to_le_bytes())
    }

    pub fn pop_i64(&mut self) -> i64 {
        if let Some(StackValue::Value(Value::I64(value))) = self.stack.pop() {
            value
        } else {
            panic!("Tried popping i64 from stack but failed")
        }
    }

    pub fn pop_u64(&mut self) -> u64 {
        u64::from_le_bytes(self.pop_i64().to_le_bytes())
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

    pub fn pop_ref(&mut self) -> Ref {
        if let Some(StackValue::Value(Value::Ref(value))) = self.stack.pop() {
            value
        } else {
            panic!("Tried popping i32 from stack but failed")
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

    pub fn pop_value_by_type(&mut self, value_type: ValueType) -> Value {
        let poped_value = self.stack.pop();
        if let Some(StackValue::Value(value)) = poped_value {
            match (value_type, value) {
                (ValueType::Numeric(NumericValueType::I32), Value::I32(_))
                | (ValueType::Numeric(NumericValueType::I64), Value::I64(_))
                | (ValueType::Numeric(NumericValueType::F32), Value::F32(_))
                | (ValueType::Numeric(NumericValueType::F64), Value::F64(_)) => {}
                _ => {
                    panic!("Tried popping: {:?} received: {:?}", value_type, value);
                }
            }
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

    pub fn drop_value(&mut self) {
        let Some(StackValue::Value(_)) = self.stack.pop() else {
            panic!("Dropped something that isn't value");
        };
    }

    pub fn pop_until_function_state(&mut self, current_function: &FunctionState) -> FunctionState {
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
                                found_block = f.function_idx() == current_function.function_idx();
                            }
                            InstructionIndex::IndexInBlock { .. } => {
                                continue;
                            }
                        }
                    } else {
                        return f;
                    }
                }
            };
        }
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
