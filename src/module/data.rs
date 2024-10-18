use std::collections::HashMap;

use crate::{
    section::{Section, SectionType},
    types::Data,
};

pub fn take_datas<'a>(sections: &mut HashMap<SectionType<'a>, Section<'a>>) -> Vec<Data> {
    let Some(datas) = sections.remove(&SectionType::Data) else {
        return vec![];
    };
    let Section::Data(data_section) = datas else {
        unreachable!();
    };

    data_section.0
}
