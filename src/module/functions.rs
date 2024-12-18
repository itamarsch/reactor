use std::{collections::HashMap, rc::Rc};

use crate::{
    section::{import::ImportSection, r#type::TypeSection, Section, SectionType},
    types::{Expr, FuncIdx, FuncType, FunctionCode, ImportDesc, LocalTypes},
};

#[derive(Debug)]
pub struct LocalFunction {
    pub signature: Rc<FuncType>,
    pub code: FunctionCode,
}

impl LocalFunction {
    pub fn expr(expr: Expr) -> Self {
        Self {
            signature: Rc::new(FuncType::empty()),
            code: super::FunctionCode {
                locals: LocalTypes::empty(),
                instructions: expr,
            },
        }
    }
}

#[derive(Debug)]
pub struct ImportedFunction<'a> {
    pub mod_name: &'a str,
    pub name: &'a str,
    pub signature: Rc<FuncType>,
}

#[derive(Debug)]
pub enum Function<'a> {
    Local(LocalFunction),
    Imported(ImportedFunction<'a>),
}

impl Function<'_> {
    pub fn signature(&self) -> Rc<FuncType> {
        match self {
            Function::Local(l) => l.signature.clone(),
            Function::Imported(i) => i.signature.clone(),
        }
    }
}

pub fn take_functions<'a>(
    sections: &mut HashMap<SectionType<'a>, Section<'a>>,
) -> (Vec<Function<'a>>, TypeSection) {
    let type_section = sections.remove(&SectionType::Type);

    let function_section = sections.remove(&SectionType::Function);

    let code_section = sections.remove(&SectionType::Code);

    if type_section.is_none() && function_section.is_none() && code_section.is_none() {
        return (vec![], TypeSection::empty());
    }

    if let (
        Some(Section::Type(type_section)),
        Some(Section::Function(function_section)),
        Some(Section::Code(code_section)),
    ) = (type_section, function_section, code_section)
    {
        let imports = sections.get_mut(&SectionType::Import);

        let mut imported_functions = if let Some(Section::Import(ImportSection(imports))) = imports
        {
            let mut imported_functions = vec![];
            for import in imports {
                match import.desc {
                    ImportDesc::Func(signature) => {
                        imported_functions.push(Function::Imported(ImportedFunction {
                            mod_name: import.mod_name,
                            name: import.name,
                            signature: type_section
                                .get_function_type(signature)
                                .expect("Import function type index to be valid")
                                .clone(),
                        }));
                    }

                    _ => continue,
                }
            }
            imported_functions
        } else {
            vec![]
        };

        let functions = code_section
            .functions
            .into_iter()
            .enumerate()
            .map(|(i, code)| {
                Function::Local(LocalFunction {
                    signature: type_section
                        .get_function_type(
                            function_section
                                .get_func_type_idx(FuncIdx(i as u32))
                                .expect("Function section len should local len"),
                        )
                        .expect("Every type idx in the funciton section should be valid"),
                    code,
                })
            })
            .collect::<Vec<_>>();

        imported_functions.extend(functions);
        (imported_functions, type_section)
    } else {
        panic!("Invalid function sections some missing")
    }
}
