use core::panic;
use std::cell::OnceCell;

use crate::types::{GlobalIdx, Mutability};

use super::{value::Value, variable::Variable};

#[derive(Debug)]
pub struct Global {
    variable: Variable,
    mutability: Mutability,
}

impl Global {
    pub fn new(value: Value, mutability: Mutability) -> Self {
        Self {
            variable: Variable::from_value(value),
            mutability,
        }
    }

    pub fn get_value(&self) -> Value {
        self.variable.value()
    }

    pub fn set_value(&mut self, value: Value) {
        match self.mutability {
            Mutability::Mutable => self.variable.set_value(value),
            Mutability::Const => {
                panic!("Tried setting const global")
            }
        }
    }
}

pub struct Globals(OnceCell<Vec<Global>>);
impl Globals {
    pub fn new() -> Globals {
        Globals(OnceCell::new())
    }

    pub fn fill(&mut self, values: Vec<Global>) {
        assert!(self.0.get().is_none(), "Can't fill, already has globals");

        self.0
            .set(values)
            .expect("Already checked in asseryt above");
    }

    pub fn set(&mut self, value: Value, GlobalIdx(global_idx): GlobalIdx) {
        self.0.get_mut().expect("Globals to be initialized")[global_idx as usize].set_value(value);
    }
    pub fn get(&self, GlobalIdx(global_idx): GlobalIdx) -> Value {
        self.0.get().expect("Globals to be initialized")[global_idx as usize].get_value()
    }
}
