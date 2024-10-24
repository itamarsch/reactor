use std::collections::HashMap;

use crate::{
    section::{Section, SectionType},
    types::TableType,
};

pub fn take_table_declarations<'a>(
    sections: &mut HashMap<SectionType<'a>, Section<'a>>,
) -> Vec<TableType> {
    if let Some(tables) = sections.remove(&SectionType::Table) {
        let Section::Table(tables) = tables else {
            unreachable!();
        };
        tables.tables
    } else {
        vec![]
    }
}
