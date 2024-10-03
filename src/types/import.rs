use nom::{bytes::complete::take, number::complete::u8, IResult};
use nom_leb128::leb128_u32;

use crate::types::{FuncTypeIdx, GlobalType, MemoryType, TableType};

#[derive(Debug)]
pub struct Import<'a> {
    pub mod_name: &'a str,
    pub name: &'a str,
    pub desc: ImportDesc,
}

#[derive(Debug)]
pub enum ImportDesc {
    Func(FuncTypeIdx),
    Table(TableType),
    Memory(MemoryType),

    Global(GlobalType),
}

impl Import<'_> {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Import> {
        let (input, mod_name_len) = leb128_u32(input)?;
        let (input, mod_name) = take(mod_name_len)(input)?;
        let mod_name = std::str::from_utf8(mod_name).unwrap();

        let (input, import_name_len) = leb128_u32(input)?;
        let (input, import_name) = take(import_name_len)(input)?;
        let import_name = std::str::from_utf8(import_name).unwrap();

        let (input, import_type) = u8(input)?;
        let (input, import_desc) = match import_type {
            0x00 => {
                //Function
                let (input, func_type_index) = leb128_u32(input)?;
                println!("IDX: {func_type_index}");
                (input, ImportDesc::Func(FuncTypeIdx(func_type_index)))
            }
            0x01 => {
                let (input, table_type) = TableType::parse(input)?;

                println!("{:?}", table_type);
                (input, ImportDesc::Table(table_type))
            }
            0x02 => {
                let (input, memory_type) = MemoryType::parse(input)?;
                println!("{:?}", memory_type);
                (input, ImportDesc::Memory(memory_type))
            }
            0x03 => {
                let (input, global_type) = GlobalType::parse(input)?;
                println!("{:?} {:?}", global_type, input);
                (input, ImportDesc::Global(global_type))
            }
            _ => {
                panic!("Invalid import_type: {import_type}")
            }
        };

        Ok((
            input,
            Import {
                mod_name,
                name: import_name,
                desc: import_desc,
            },
        ))
    }
}