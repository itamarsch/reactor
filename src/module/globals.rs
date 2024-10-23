use std::collections::HashMap;

use crate::section::{global::GlobalInitializer, Section, SectionType};

pub fn take_globals<'a>(
    sections: &mut HashMap<SectionType<'a>, Section<'a>>,
) -> Vec<GlobalInitializer> {
    let globals = sections.remove(&SectionType::Global);

    if let Some(globals) = globals {
        let Section::Global(globals) = globals else {
            unreachable!();
        };
        globals.0
    } else {
        vec![]
    }
}
