use nom::{number::complete::u8, IResult};
use nom_leb128::leb128_u32;

use crate::module::functions::{self, Function};

use super::{wasm_vec, Expr, FuncIdx, MemoryIdx};

#[derive(Debug)]
pub struct DataDeclaration {
    init: Vec<u8>,
    mode: DataDeclarationMode,
}

#[derive(Debug)]
enum DataDeclarationMode {
    Passive,
    Active { memidx: MemoryIdx, offset: Expr },
}

#[derive(Debug)]
pub struct Data {
    pub init: Vec<u8>,
    pub mode: DataMode,
}

#[derive(Debug)]
pub enum DataMode {
    Passive,
    Active { memidx: MemoryIdx, offset: FuncIdx },
}

impl DataDeclaration {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DataDeclaration> {
        let (input, variant) = leb128_u32(input)?;
        let (input, data) = match variant {
            0 => {
                let (input, offset) = Expr::parse(input)?;
                let (input, init) = wasm_vec(u8)(input)?;
                (
                    input,
                    DataDeclaration {
                        init,
                        mode: DataDeclarationMode::Active {
                            memidx: MemoryIdx(0),
                            offset,
                        },
                    },
                )
            }
            1 => {
                let (input, init) = wasm_vec(u8)(input)?;
                (
                    input,
                    DataDeclaration {
                        init,
                        mode: DataDeclarationMode::Passive,
                    },
                )
            }
            2 => {
                let (input, memidx) = MemoryIdx::parse(input)?;
                let (input, offset) = Expr::parse(input)?;
                let (input, init) = wasm_vec(u8)(input)?;
                (
                    input,
                    DataDeclaration {
                        init,
                        mode: DataDeclarationMode::Active { memidx, offset },
                    },
                )
            }
            _ => panic!("Invalid data variant {}", variant),
        };
        Ok((input, data))
    }

    pub fn add_to_module(self, functions: &mut Vec<Function<'_>>) -> Data {
        Data {
            init: self.init,
            mode: match self.mode {
                DataDeclarationMode::Passive => DataMode::Passive,
                DataDeclarationMode::Active { memidx, offset } => {
                    let idx = FuncIdx(functions.len() as u32);
                    functions.push(Function::Local(functions::LocalFunction::expr(offset)));
                    DataMode::Active {
                        memidx,
                        offset: idx,
                    }
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct DataIdx(pub u32);
impl DataIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DataIdx> {
        leb128_u32(input).map(|(input, value)| (input, DataIdx(value)))
    }
}
