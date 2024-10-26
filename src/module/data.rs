use std::collections::HashMap;

use crate::{
    section::{Section, SectionType},
    types::Data,
};

use super::functions::Function;

pub fn take_datas<'a>(
    sections: &mut HashMap<SectionType<'a>, Section<'a>>,
    functions: &mut Vec<Function<'_>>,
) -> Vec<Data> {
    let Some(datas) = sections.remove(&SectionType::Data) else {
        return vec![];
    };
    let Section::Data(data_section) = datas else {
        unreachable!();
    };

    data_section
        .0
        .into_iter()
        .map(|e| e.add_to_module(functions))
        .collect()
}
