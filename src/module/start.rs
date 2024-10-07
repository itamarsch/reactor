use std::collections::HashMap;

use crate::{
    section::{Section, SectionType},
    types::{Export, ExportDesc, FuncIdx},
};

pub fn get_starting_function_index(
    sections: &mut HashMap<SectionType, Section>,
) -> Option<FuncIdx> {
    let export = sections.get(&SectionType::Export);
    if let Some(Section::Export(export_section)) = export {
        export_section.exports.iter().find_map(|e| match e {
            Export {
                name: "_start",
                desc: ExportDesc::Func(f),
            } => Some(*f),
            _ => None,
        })
    } else {
        None
    }
}
