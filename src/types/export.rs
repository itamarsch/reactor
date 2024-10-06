use nom::{bytes::complete::take, number::complete::u8, IResult};
use nom_leb128::leb128_u32;

use super::{name, FuncIdx, GlobalIdx, MemoryIdx, TableIdx};

#[derive(Debug)]
pub struct Export<'a> {
    pub name: &'a str,
    pub desc: ExportDesc,
}

impl Export<'_> {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Export> {
        let (input, name) = name(input)?;
        let (input, desc) = ExportDesc::parse(input)?;
        Ok((input, Export { name, desc }))
    }
}

#[derive(Debug)]
pub enum ExportDesc {
    Func(FuncIdx),
    Table(TableIdx),
    Memory(MemoryIdx),
    Global(GlobalIdx),
}

impl ExportDesc {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ExportDesc> {
        let (input, flag) = u8(input)?;
        match flag {
            0x00 => {
                let (input, func_idx) = FuncIdx::parse(input)?;
                Ok((input, ExportDesc::Func(func_idx)))
            }
            0x01 => {
                let (input, table_idx) = TableIdx::parse(input)?;
                Ok((input, ExportDesc::Table(table_idx)))
            }
            0x02 => {
                let (input, memory_idx) = MemoryIdx::parse(input)?;
                Ok((input, ExportDesc::Memory(memory_idx)))
            }
            0x03 => {
                let (input, global_idx) = GlobalIdx::parse(input)?;
                Ok((input, ExportDesc::Global(global_idx)))
            }
            _ => {
                panic!("Invalid export type")
            }
        }
    }
}
