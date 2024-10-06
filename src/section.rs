use nom::{bytes::complete::take, number::complete::le_u8, sequence::pair, IResult};
use nom_leb128::leb128_u32;

use crate::types::name;

use self::{
    code::CodeSection, data::DataSection, element::ElementSection, export::ExportSection,
    function::FunctionSection, global::GlobalSection, import::ImportSection, memory::MemorySection,
    r#type::TypeSection, table::TableSection,
};

mod code;
mod data;
mod element;
mod export;
mod function;
mod global;
mod import;
mod memory;
mod table;
mod r#type;

#[derive(Debug)]
pub enum Section<'a> {
    Custom(&'a str, &'a [u8]),
    Type(TypeSection),
    Import(ImportSection<'a>),
    Function(FunctionSection),
    Table(TableSection),
    Memory(MemorySection),
    Global(GlobalSection),
    Export(ExportSection<'a>),
    Start(&'a [u8]),

    Element(ElementSection),
    Code(CodeSection),
    Data(DataSection),
    DataCount(u32),
}

fn parse_section(input: &[u8]) -> IResult<&[u8], (u8, &[u8])> {
    let (input, (code, size)) = pair(le_u8, leb128_u32)(input)?; // 1
    let (input, section_data) = take(size)(input)?;
    Ok((input, (code, section_data)))
}

impl<'a> Section<'a> {
    pub fn get_variant(&self) -> &'a str {
        match self {
            Section::Custom(name, _) => name,
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
            Section::DataCount(_) => "DataCount",
        }
    }

    pub fn parse(input: &'a [u8]) -> IResult<&'a [u8], Section<'a>> {
        let (input, (code, section_data)) = parse_section(input)?;
        let section = match code {
            0 => {
                let (section_data, name) = name(section_data)?;

                Section::Custom(name, section_data)
            }
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
            9 => {
                let (_, elements) = ElementSection::parse(section_data)?;
                println!("{:#?}", elements);

                Section::Element(elements)
            }
            10 => {
                let (_, code_section) = CodeSection::parse(section_data)?;
                println!("{:#?}", code_section);

                Section::Code(code_section)
            }
            11 => {
                let (_, data_section) = DataSection::parse(section_data)?;
                println!("{:#?}", data_section);

                Section::Data(data_section)
            }
            12 => {
                let (_, amount_of_datas) = leb128_u32(section_data)?;
                println!(
                    "DataCount Section {{ amount_of_datas: {:?} }}",
                    amount_of_datas
                );
                Section::DataCount(amount_of_datas)
            }
            _ => {
                panic!("Invalid section type {}", code);
            }
        };

        Ok((input, section))
    }
}
