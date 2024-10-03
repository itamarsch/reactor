mod code;
mod export;
mod func;
mod global;
mod import;
mod limit;
mod memory;
mod table;
mod value;

pub use func::{FuncIdx, FuncType, FuncTypeIdx};

pub use global::{GlobalIdx, GlobalType};

pub use limit::Limit;

pub use memory::{MemoryIdx, MemoryType};

use nom::{multi::count, IResult};
use nom_leb128::leb128_u32;
pub use table::{TableIdx, TableType};

pub use value::ValueType;

pub use import::{Import, ImportDesc};

pub use code::{FunctionCode, Instruction, Locals};
pub use export::{Export, ExportDesc};

pub fn wasm_vec<'a, T>(
    mut parse: impl FnMut(&'a [u8]) -> IResult<&'a [u8], T>,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<T>>
where
    T: 'a,
{
    move |input| {
        let (input, len) = leb128_u32(input)?;
        let (input, values) = count(&mut parse, len as usize)(input)?;
        Ok((input, values))
    }
}
