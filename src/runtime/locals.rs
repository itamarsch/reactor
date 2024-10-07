use crate::types::{LocalIdx, LocalTypes, NumericValueType, ValueType};

use super::{local::Local, stack::Stack, value::Value};

fn local_types_to_defaults(local_types: &LocalTypes) -> impl Iterator<Item = Local> + '_ {
    local_types.0.iter().map(|e| match e {
        ValueType::Numeric(NumericValueType::I32) => Local::from_i32_default(),
        ValueType::Numeric(NumericValueType::I64) => todo!(),
        ValueType::Numeric(NumericValueType::F32) => todo!(),
        ValueType::Numeric(NumericValueType::F64) => todo!(),
        ValueType::Ref(_) => todo!(),
    })
}

#[derive(Debug)]
pub struct Locals(Vec<Local>);

impl Locals {
    pub fn get_value(&self, LocalIdx(idx): LocalIdx) -> Value {
        self.0[idx as usize].value()
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
            .map(|e| match e {
                ValueType::Numeric(NumericValueType::I32) => Local::from_i32(stack.pop_i32()),
                ValueType::Numeric(NumericValueType::I64) => todo!(),
                ValueType::Numeric(NumericValueType::F32) => todo!(),
                ValueType::Numeric(NumericValueType::F64) => todo!(),
                ValueType::Ref(_) => todo!(),
            })
            .collect::<Vec<_>>();
        params.extend(locals);
        Locals(params)
    }
}
