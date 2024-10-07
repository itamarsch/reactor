use nom::{
    bytes::complete::take,
    error::{ContextError, ParseError},
    multi::count,
    IResult, Parser,
};
use nom_leb128::leb128_u32;

mod block_type;
mod code;
mod data;
mod element;
mod export;
mod func;
mod global;
mod import;
mod label_index;
mod limit;
mod memory;
mod ref_type;
mod table;
mod value;

pub use func::{FuncIdx, FuncType, FuncTypeIdx};

pub use global::{GlobalIdx, GlobalType};

pub use limit::Limit;

pub use memory::{MemoryIdx, MemoryType};

pub use table::{TableIdx, TableType};

pub use value::{NumericValueType, ValueType};

pub use import::{Import, ImportDesc};

pub use code::{FunctionCode, Instruction, LocalIdx, Locals, MemoryArgument};

pub use export::{Export, ExportDesc};

pub use block_type::BlockType;

pub use label_index::LabelIdx;

pub use element::{Element, ElementIdx, ElementMode};

pub use ref_type::RefType;

pub use data::{Data, DataIdx, DataMode};

pub use code::Expr;

pub fn name(input: &[u8]) -> IResult<&[u8], &str> {
    let (input, name_len) = leb128_u32(input)?;
    let (input, name) = take(name_len)(input)?;
    let name = std::str::from_utf8(name).unwrap();
    Ok((input, name))
}

pub fn wasm_vec<'a, T, F, E>(mut parse: F) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<T>, E>
where
    T: 'a,
    F: Parser<&'a [u8], T, E>,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
{
    move |input| {
        let (input, len) = leb128_u32(input)?;
        let (input, values) = count(|input| parse.parse(input), len as usize)(input)?;
        Ok((input, values))
    }
}
