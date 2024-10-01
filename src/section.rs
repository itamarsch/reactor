use nom::{bytes::complete::take, number::complete::le_u8, sequence::pair, IResult};
use nom_leb128::leb128_u32;

use self::{
    function::FunctionSection, import::ImportSection, r#type::TypeSection, table::TableSection,
};

mod function;
mod import;
mod table;
mod r#type;

#[derive(Debug)]
pub enum Section<'a> {
    Custom(&'a [u8]),
    Type(TypeSection),
    Import(ImportSection<'a>),
    Function(FunctionSection),
    Table(TableSection),
    Memory(&'a [u8]),
    Global(&'a [u8]),
    Export(&'a [u8]),
    Start(&'a [u8]),
    Element(&'a [u8]),
    Code(&'a [u8]),
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
            5 => Section::Memory(section_data),
            6 => Section::Global(section_data),
            7 => Section::Export(section_data),
            8 => Section::Start(section_data),
            9 => Section::Element(section_data),
            10 => Section::Code(section_data),
            11 => Section::Data(section_data),
            _ => {
                panic!("Invalid section type");
            }
        };

        Ok((input, section))
    }
}
