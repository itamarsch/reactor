use crate::types::Mutability;

use super::{value::Value, variable::Variable};

pub struct Global {
    variable: Variable,
    mutability: Mutability,
}
impl Global {
    fn get_value(&self) -> Value {
        self.variable.value()
    }

    fn set_value(&mut self, value: Value) {
        match self.mutability {
            Mutability::Mutable => self.variable.set_value(value),
            Mutability::Const => {
                panic!("Tried setting const global")
            }
        }
    }
}

pub struct Globals(Vec<Global>);
