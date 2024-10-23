use crate::types::{LocalIdx, LocalTypes, NumericValueType, ValueType};

use super::{stack::Stack, value::Value, variable::Variable};

fn local_types_to_defaults(local_types: &LocalTypes) -> impl Iterator<Item = Variable> + '_ {
    local_types.0.iter().map(|e| match e {
        ValueType::Numeric(NumericValueType::I32) => Variable::from_i32_default(),
        ValueType::Numeric(NumericValueType::I64) => Variable::from_i64_default(),
        ValueType::Numeric(NumericValueType::F32) => Variable::from_f32_default(),
        ValueType::Numeric(NumericValueType::F64) => Variable::from_f64_default(),
        ValueType::Ref(_) => todo!(),
    })
}

#[derive(Debug)]
pub struct Locals(Vec<Variable>);

impl Locals {
    pub fn set_value(&mut self, LocalIdx(idx): LocalIdx, value: Value) {
        self.0[idx as usize].set_value(value);
    }
    pub fn get_value(&self, LocalIdx(idx): LocalIdx) -> Value {
        self.0[idx as usize].value()
    }

    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn new_no_function_parameters(non_function_parameter_types: &LocalTypes) -> Locals {
        let locals = local_types_to_defaults(non_function_parameter_types);
        Locals(locals.collect())
    }

    pub fn new(
        non_function_parameter_types: &LocalTypes,
        function_parameters: &[ValueType],
        stack: &mut Stack,
    ) -> Locals {
        let locals = local_types_to_defaults(non_function_parameter_types);

        let mut params = function_parameters
            .iter()
            .rev()
            .map(|e| match e {
                ValueType::Numeric(NumericValueType::I32) => Variable::from_i32(stack.pop_i32()),
                ValueType::Numeric(NumericValueType::I64) => Variable::from_i64(stack.pop_i64()),
                ValueType::Numeric(NumericValueType::F32) => Variable::from_f32(stack.pop_f32()),
                ValueType::Numeric(NumericValueType::F64) => Variable::from_f64(stack.pop_f64()),
                ValueType::Ref(_) => todo!(),
            })
            .collect::<Vec<_>>();
        params.reverse();
        params.extend(locals);
        Locals(params)
    }
}
