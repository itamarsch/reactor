use std::collections::HashMap;

use crate::{
    section::{Section, SectionType},
    types::Element,
};

use super::functions::Function;

pub fn take_element_declarations<'a>(
    sections: &mut HashMap<SectionType<'a>, Section<'a>>,
    functions: &mut Vec<Function<'_>>,
) -> Vec<Element> {
    if let Some(elements) = sections.remove(&SectionType::Element) {
        let Section::Element(elements) = elements else {
            unreachable!();
        };
        elements
            .0
            .into_iter()
            .map(|e| e.add_to_module(functions))
            .collect()
    } else {
        vec![]
    }
}
