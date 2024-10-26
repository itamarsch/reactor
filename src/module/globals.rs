use std::collections::HashMap;

use crate::section::{global::GlobalInitializer, Section, SectionType};

use super::functions::Function;

pub fn take_globals<'a>(
    sections: &mut HashMap<SectionType<'a>, Section<'a>>,
    functions: &mut Vec<Function<'_>>,
) -> Vec<GlobalInitializer> {
    let globals = sections.remove(&SectionType::Global);

    if let Some(globals) = globals {
        let Section::Global(globals) = globals else {
            unreachable!();
        };
        globals
            .0
            .into_iter()
            .map(|g| g.add_to_module(functions))
            .collect()
    } else {
        vec![]
    }
}
