use nom::{bytes::complete::take, number::complete::le_u8, sequence::pair, IResult};
use nom_leb128::leb128_u32;

use self::{
    code::CodeSection, export::ExportSection, function::FunctionSection, global::GlobalSection,
    import::ImportSection, memory::MemorySection, r#type::TypeSection, table::TableSection,
};

mod code;
mod export;
mod function;
mod global;
mod import;
mod memory;
mod table;
mod r#type;

#[derive(Debug)]
pub enum Section<'a> {
    Custom(&'a [u8]),
    Type(TypeSection),
    Import(ImportSection<'a>),
    Function(FunctionSection),
    Table(TableSection),
    Memory(MemorySection),
    Global(GlobalSection),
    Export(ExportSection<'a>),
    Start(&'a [u8]),
    Element(&'a [u8]),
    Code(CodeSection),
    Data(&'a [u8]),
}

fn parse_section(input: &[u8]) -> IResult<&[u8], (u8, &[u8])> {
    let (input, (code, size)) = pair(le_u8, leb128_u32)(input)?; // 1
    let (input, section_data) = take(size)(input)?;
    Ok((input, (code, section_data)))
}

impl<'a> Section<'a> {
    pub fn get_variant(&self) -> &str {
        match self {
            Section::Custom(_) => "Custom",
            Section::Type(_) => "Type",
            Section::Import(_) => "Import",
            Section::Function(_) => "Function",
            Section::Table(_) => "Table",
            Section::Memory(_) => "Memory",
            Section::Global(_) => "Global",
            Section::Export(_) => "Export",
            Section::Start(_) => "Start",
            Section::Element(_) => "Element",
            Section::Code(_) => "Code",
            Section::Data(_) => "Data",
        }
    }

    pub fn parse(input: &'a [u8]) -> IResult<&'a [u8], Section<'a>> {
        let (input, (code, section_data)) = parse_section(input)?;

        let section = match code {
            0 => Section::Custom(section_data),
            1 => {
                let (_, type_section) = TypeSection::parse(section_data)?;
                println!("{:#?}", type_section);

                Section::Type(type_section)
            }
            2 => {
                let (_, import_section) = ImportSection::parse(section_data)?;
                println!("{:#?}", import_section);

                Section::Import(import_section)
            }
            3 => {
                let (_, function_section) = FunctionSection::parse(section_data)?;
                println!("{:#?}", function_section);

                Section::Function(function_section)
            }
            4 => {
                let (_, table_section) = TableSection::parse(section_data)?;
                println!("{:#?}", table_section);

                Section::Table(table_section)
            }
            5 => {
                let (_, memory_section) = MemorySection::parse(section_data)?;
                println!("{:#?}", memory_section);
                Section::Memory(memory_section)
            }
            7 => {
                let (_, export_section) = ExportSection::parse(section_data)?;
                println!("{:#?}", export_section);
                Section::Export(export_section)
            }
            6 => {
                let (_, global_section) = GlobalSection::parse(section_data)?;
                println!("{:#?}", global_section);

                Section::Global(global_section)
            }
            8 => Section::Start(section_data),
            9 => Section::Element(section_data),
            10 => {
                let (_, code_section) = CodeSection::parse(section_data)?;
                println!("{:#?}", code_section);

                Section::Code(code_section)
            }
            11 => Section::Data(section_data),
            _ => {
                panic!("Invalid section type");
            }
        };

        Ok((input, section))
    }
}
