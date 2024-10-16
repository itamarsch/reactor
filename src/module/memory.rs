use std::collections::HashMap;

use crate::{
    section::{Section, SectionType},
    types::Limit,
};

pub fn take_memory_declaration<'a>(sections: &mut HashMap<SectionType<'a>, Section<'a>>) -> Limit {
    let Some(mem) = sections.get(&SectionType::Memory) else {
        panic!("wasi requires a memory to be declared in the module");
    };
    let Section::Memory(mem) = mem else {
        unreachable!();
    };
    mem.memories.first().expect("Has to have one element").0
}
