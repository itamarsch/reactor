use nom::{bytes::complete::tag, IResult};
use nom_leb128::leb128_u32;

use crate::module::functions::{Function, LocalFunction};

use super::{wasm_vec, Expr, FuncIdx, Instruction, RefType, TableIdx};

#[derive(Debug)]
pub struct Element {
    pub ref_type: RefType,
    pub init: Vec<FuncIdx>,
    pub mode: ElementMode,
}

#[derive(Debug)]
pub struct ElementDeclaration {
    pub ref_type: RefType,
    pub init: Vec<Expr>,
    pub mode: ElementModeDeclaration,
}

#[derive(Debug)]
pub enum ElementModeDeclaration {
    Passive,
    Active {
        table: TableIdx,
        offset_in_table: Expr,
    },
    Declarative,
}

#[derive(Debug)]
pub enum ElementMode {
    Passive,
    Active {
        table: TableIdx,
        offset_in_table: FuncIdx,
    },
    Declarative,
}
fn func_idx_to_initializer(functions: Vec<FuncIdx>) -> Vec<Expr> {
    functions
        .into_iter()
        .map(|func| Expr::from_raw_instructions(vec![Instruction::PushFuncRef(func)]))
        .collect()
}

impl ElementDeclaration {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ElementDeclaration> {
        let (input, variant) = leb128_u32(input)?;
        let (input, element) = match variant {
            0 => {
                let (input, offset_in_table) = Expr::parse(input)?;
                let (input, functions) = wasm_vec(FuncIdx::parse)(input)?;
                (
                    input,
                    ElementDeclaration {
                        ref_type: RefType::FuncRef,
                        init: func_idx_to_initializer(functions),
                        mode: ElementModeDeclaration::Active {
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
                    ElementDeclaration {
                        ref_type: RefType::FuncRef,
                        init: func_idx_to_initializer(functions),
                        mode: ElementModeDeclaration::Passive,
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
                    ElementDeclaration {
                        ref_type: RefType::FuncRef,
                        init: func_idx_to_initializer(functions),
                        mode: ElementModeDeclaration::Active {
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
                    ElementDeclaration {
                        ref_type: RefType::FuncRef,
                        init: func_idx_to_initializer(functions),
                        mode: ElementModeDeclaration::Declarative,
                    },
                )
            }
            4 => {
                let (input, offset) = Expr::parse(input)?;
                let (input, init) = wasm_vec(Expr::parse)(input)?;
                (
                    input,
                    ElementDeclaration {
                        ref_type: RefType::FuncRef,
                        init,
                        mode: ElementModeDeclaration::Active {
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
                    ElementDeclaration {
                        ref_type,
                        init,
                        mode: ElementModeDeclaration::Passive,
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
                    ElementDeclaration {
                        ref_type,
                        init,
                        mode: ElementModeDeclaration::Active {
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
                    ElementDeclaration {
                        ref_type,
                        init,
                        mode: ElementModeDeclaration::Declarative,
                    },
                )
            }
            _ => panic!("Invailid element variant {}", variant),
        };

        Ok((input, element))
    }

    pub fn add_to_module(self, functions: &mut Vec<Function<'_>>) -> Element {
        Element {
            init: self
                .init
                .into_iter()
                .map(|e| {
                    let idx = FuncIdx(functions.len() as u32);
                    functions.push(Function::Local(LocalFunction::expr(e)));
                    idx
                })
                .collect(),
            ref_type: self.ref_type,
            mode: match self.mode {
                ElementModeDeclaration::Passive => ElementMode::Passive,
                ElementModeDeclaration::Active {
                    table,
                    offset_in_table,
                } => {
                    let idx = FuncIdx(functions.len() as u32);
                    functions.push(Function::Local(LocalFunction::expr(offset_in_table)));
                    ElementMode::Active {
                        offset_in_table: idx,
                        table,
                    }
                }
                ElementModeDeclaration::Declarative => ElementMode::Declarative,
            },
        }
    }
}

#[derive(Debug)]
pub struct ElementIdx(pub u32);
impl ElementIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ElementIdx> {
        leb128_u32(input).map(|(input, value)| (input, ElementIdx(value)))
    }
}
