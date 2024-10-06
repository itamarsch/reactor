use nom::{bytes::complete::tag, IResult};
use nom_leb128::leb128_u32;

use super::{wasm_vec, Expr, FuncIdx, Instruction, RefType, TableIdx};

#[derive(Debug)]
pub struct Element {
    pub ref_type: RefType,
    pub init: Vec<Expr>,
    pub mode: ElementMode,
}

fn func_idx_to_initializer(functions: Vec<FuncIdx>) -> Vec<Expr> {
    functions
        .into_iter()
        .map(|func| Expr(vec![Instruction::PushFuncRef(func)]))
        .collect()
}

impl Element {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Element> {
        let (input, variant) = leb128_u32(input)?;
        let (input, element) = match variant {
            0 => {
                let (input, offset_in_table) = Expr::parse(input)?;
                let (input, functions) = wasm_vec(FuncIdx::parse)(input)?;
                (
                    input,
                    Element {
                        ref_type: RefType::FuncRef,
                        init: func_idx_to_initializer(functions),
                        mode: ElementMode::Active {
                            table: TableIdx(0),
                            offset_in_table,
                        },
                    },
                )
            }
            1 => {
                let (input, _) = tag(&[0][..])(input)?;
                let (input, functions) = wasm_vec(FuncIdx::parse)(input)?;
                (
                    input,
                    Element {
                        ref_type: RefType::FuncRef,
                        init: func_idx_to_initializer(functions),
                        mode: ElementMode::Passive,
                    },
                )
            }
            2 => {
                let (input, table) = TableIdx::parse(input)?;
                let (input, offset) = Expr::parse(input)?;
                let (input, _) = tag(&[0][..])(input)?;
                let (input, functions) = wasm_vec(FuncIdx::parse)(input)?;
                (
                    input,
                    Element {
                        ref_type: RefType::FuncRef,
                        init: func_idx_to_initializer(functions),
                        mode: ElementMode::Active {
                            table,
                            offset_in_table: offset,
                        },
                    },
                )
            }
            3 => {
                let (input, _) = tag(&[0][..])(input)?;
                let (input, functions) = wasm_vec(FuncIdx::parse)(input)?;
                (
                    input,
                    Element {
                        ref_type: RefType::FuncRef,
                        init: func_idx_to_initializer(functions),
                        mode: ElementMode::Declarative,
                    },
                )
            }
            4 => {
                let (input, offset) = Expr::parse(input)?;
                let (input, init) = wasm_vec(Expr::parse)(input)?;
                (
                    input,
                    Element {
                        ref_type: RefType::FuncRef,
                        init,
                        mode: ElementMode::Active {
                            table: TableIdx(0),
                            offset_in_table: offset,
                        },
                    },
                )
            }
            5 => {
                let (input, ref_type) = RefType::parse(input)?;
                let (input, init) = wasm_vec(Expr::parse)(input)?;
                (
                    input,
                    Element {
                        ref_type,
                        init,
                        mode: ElementMode::Passive,
                    },
                )
            }
            6 => {
                let (input, table_idx) = TableIdx::parse(input)?;
                let (input, offset) = Expr::parse(input)?;
                let (input, ref_type) = RefType::parse(input)?;
                let (input, init) = wasm_vec(Expr::parse)(input)?;
                (
                    input,
                    Element {
                        ref_type,
                        init,
                        mode: ElementMode::Active {
                            table: table_idx,
                            offset_in_table: offset,
                        },
                    },
                )
            }
            7 => {
                let (input, ref_type) = RefType::parse(input)?;
                let (input, init) = wasm_vec(Expr::parse)(input)?;
                (
                    input,
                    Element {
                        ref_type,
                        init,
                        mode: ElementMode::Declarative,
                    },
                )
            }
            _ => panic!("Invailid element variant {}", variant),
        };

        Ok((input, element))
    }
}

#[derive(Debug)]
pub enum ElementMode {
    Passive,
    Active {
        table: TableIdx,
        offset_in_table: Expr,
    },
    Declarative,
}

#[derive(Debug)]
pub struct ElementIdx(pub u32);
impl ElementIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ElementIdx> {
        leb128_u32(input).map(|(input, value)| (input, ElementIdx(value)))
    }
}
