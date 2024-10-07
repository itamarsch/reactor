use nom::{bytes::complete::take, number::complete::le_u8, sequence::pair, IResult};
use nom_leb128::leb128_u32;

use crate::types::name;

use self::{
    code::CodeSection, data::DataSection, data_count::DataCountSection, element::ElementSection,
    export::ExportSection, function::FunctionSection, global::GlobalSection, import::ImportSection,
    memory::MemorySection, r#type::TypeSection, start::StartSection, table::TableSection,
};

pub mod code;
pub mod data;
pub mod data_count;
pub mod element;
pub mod export;
pub mod function;
pub mod global;
pub mod import;
pub mod memory;
pub mod start;
pub mod table;
pub mod r#type;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum SectionType<'a> {
    Custom(&'a str),
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,

    Element,
    Code,
    Data,
    DataCount,
}

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
    Start(StartSection),

    Element(ElementSection),
    Code(CodeSection),
    Data(DataSection),
    DataCount(DataCountSection),
}

fn parse_section(input: &[u8]) -> IResult<&[u8], (u8, &[u8])> {
    let (input, (code, size)) = pair(le_u8, leb128_u32)(input)?; // 1
    let (input, section_data) = take(size)(input)?;
    Ok((input, (code, section_data)))
}

impl<'a> Section<'a> {
    pub fn get_variant(&self) -> SectionType<'a> {
        match self {
            Section::Custom(name, _) => SectionType::Custom(name),
            Section::Type(_) => SectionType::Type,
            Section::Import(_) => SectionType::Import,
            Section::Function(_) => SectionType::Function,
            Section::Table(_) => SectionType::Table,
            Section::Memory(_) => SectionType::Memory,
            Section::Global(_) => SectionType::Global,
            Section::Export(_) => SectionType::Export,
            Section::Start(_) => SectionType::Start,
            Section::Element(_) => SectionType::Element,
            Section::Code(_) => SectionType::Code,
            Section::Data(_) => SectionType::Data,
            Section::DataCount(_) => SectionType::DataCount,
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
                Section::Type(type_section)
            }
            2 => {
                let (_, import_section) = ImportSection::parse(section_data)?;
                Section::Import(import_section)
            }
            3 => {
                let (_, function_section) = FunctionSection::parse(section_data)?;
                Section::Function(function_section)
            }
            4 => {
                let (_, table_section) = TableSection::parse(section_data)?;
                Section::Table(table_section)
            }
            5 => {
                let (_, memory_section) = MemorySection::parse(section_data)?;
                Section::Memory(memory_section)
            }
            7 => {
                let (_, export_section) = ExportSection::parse(section_data)?;
                Section::Export(export_section)
            }
            6 => {
                let (_, global_section) = GlobalSection::parse(section_data)?;
                Section::Global(global_section)
            }
            8 => {
                let (_, start_section) = StartSection::parse(section_data)?;
                Section::Start(start_section)
            }
            9 => {
                let (_, elements) = ElementSection::parse(section_data)?;
                Section::Element(elements)
            }
            10 => {
                let (_, code_section) = CodeSection::parse(section_data)?;
                Section::Code(code_section)
            }
            11 => {
                let (_, data_section) = DataSection::parse(section_data)?;
                Section::Data(data_section)
            }
            12 => {
                let (_, amount_of_datas) = DataCountSection::parse(section_data)?;
                Section::DataCount(amount_of_datas)
            }
            _ => {
                panic!("Invalid section type {}", code);
            }
        };

        Ok((input, section))
    }
}
