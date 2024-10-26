use std::collections::HashMap;

use crate::{
    section::{start::StartSection, Section, SectionType},
    types::{Export, ExportDesc, FuncIdx},
};

pub fn get_main_index(sections: &HashMap<SectionType, Section>) -> Option<FuncIdx> {
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

pub fn take_start_index(sections: &mut HashMap<SectionType, Section>) -> Option<FuncIdx> {
    let start = sections.remove(&SectionType::Start);
    start.map(|s| {
        let Section::Start(StartSection { start_from }) = s else {
            unreachable!()
        };
        start_from
    })
}
