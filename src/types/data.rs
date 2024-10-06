use nom::{number::complete::u8, IResult};
use nom_leb128::leb128_u32;

use super::{wasm_vec, Expr, MemoryIdx};

#[derive(Debug)]
pub struct Data {
    pub init: Vec<u8>,
    pub mode: DataMode,
}

impl Data {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Data> {
        let (input, variant) = leb128_u32(input)?;
        let (input, data) = match variant {
            0 => {
                let (input, offset) = Expr::parse(input)?;
                let (input, init) = wasm_vec(u8)(input)?;
                (
                    input,
                    Data {
                        init,
                        mode: DataMode::Active {
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
                    Data {
                        init,
                        mode: DataMode::Passive,
                    },
                )
            }
            2 => {
                let (input, memidx) = MemoryIdx::parse(input)?;
                let (input, offset) = Expr::parse(input)?;
                let (input, init) = wasm_vec(u8)(input)?;
                (
                    input,
                    Data {
                        init,
                        mode: DataMode::Active { memidx, offset },
                    },
                )
            }
            _ => panic!("Invalid data variant {}", variant),
        };
        Ok((input, data))
    }
}

#[derive(Debug)]
pub enum DataMode {
    Passive,
    Active { memidx: MemoryIdx, offset: Expr },
}

#[derive(Debug)]
pub struct DataIdx(pub u32);
impl DataIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DataIdx> {
        leb128_u32(input).map(|(input, value)| (input, DataIdx(value)))
    }
}
