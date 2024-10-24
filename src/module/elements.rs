use std::collections::HashMap;

use crate::{
    section::{Section, SectionType},
    types::Element,
};

pub fn take_element_declarations<'a>(
    sections: &mut HashMap<SectionType<'a>, Section<'a>>,
) -> Vec<Element> {
    if let Some(elements) = sections.remove(&SectionType::Element) {
        let Section::Element(elements) = elements else {
            unreachable!();
        };
        elements.0
    } else {
        vec![]
    }
}
