use crate::module::Module;

use self::value_stack::ValueStack;

mod call_stack;
mod function_state;
mod local;
mod value;
mod value_stack;

pub struct Runtime<'a> {
    value_stack: ValueStack,
    module: Module<'a>,
}

impl<'a> Runtime<'a> {
    pub fn new(module: Module<'a>) -> Self {
        Runtime {
            value_stack: ValueStack::empty(),
            module,
        }
    }

    pub fn execute(self) {
        todo!()
    }
}
