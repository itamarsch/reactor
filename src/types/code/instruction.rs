use nom::{
    bytes::complete::{tag, take_until},
    combinator::cut,
    multi::{many0, many_till},
    number::{
        complete::{f32, f64, u8},
        Endianness,
    },
    IResult,
};
use nom_leb128::{leb128_i32, leb128_i64};

use crate::types::{wasm_vec, BlockType, FuncIdx, FuncTypeIdx, LabelIdx};

#[derive(Debug)]
pub enum Instruction {
    Unreachable,
    Nop,
    Block(BlockType, Vec<Instruction>),
    Loop(BlockType, Vec<Instruction>),

    If {
        block_type: BlockType,
        if_instructions: Vec<Instruction>,
        else_instructions: Vec<Instruction>,
    },
    Break(LabelIdx),
    BreakIf(LabelIdx),
    BreakTable {
        labels: Vec<LabelIdx>,
        default: LabelIdx,
    },
    Return,
    Call(FuncIdx),
    CallIndirect(FuncTypeIdx),
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Add,
}

impl Instruction {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Instruction> {
        let (input, value) = u8(input)?;
        let (input, instruction) = match value {
            0x00 => (input, Instruction::Unreachable),
            0x01 => (input, Instruction::Nop),
            0x02 | 0x03 => {
                let (mut input, block_type) = BlockType::parse(input)?;
                let mut instructions = vec![];
                loop {
                    let instruction;
                    if input[0] == 0x0B {
                        (input, _) = tag(&[0x0B][..])(input)?;
                        break;
                    }
                    (input, instruction) = Instruction::parse(input)?;
                    instructions.push(instruction);
                }

                (
                    input,
                    if value == 0x02 {
                        Instruction::Block(block_type, instructions)
                    } else {
                        Instruction::Loop(block_type, instructions)
                    },
                )
            }
            0x04 => {
                let (mut input, block_type) = BlockType::parse(input)?;
                let mut if_instructions = vec![];
                let mut else_instructions = vec![];
                let mut reached_else = false;
                loop {
                    let instruction;
                    if input[0] == 0x05 {
                        reached_else = true;
                        (input, _) = tag(&[0x05][..])(input)?;
                        continue;
                    }
                    if input[0] == 0x0B {
                        (input, _) = tag(&[0x0B][..])(input)?;
                        break;
                    }
                    (input, instruction) = Instruction::parse(input)?;
                    if reached_else {
                        else_instructions.push(instruction);
                    } else {
                        if_instructions.push(instruction);
                    }
                }

                (
                    input,
                    Instruction::If {
                        block_type,
                        if_instructions,
                        else_instructions,
                    },
                )
            }
            0x0c | 0x0d => {
                let (input, label_idx) = LabelIdx::parse(input)?;
                (
                    input,
                    if value == 0x0c {
                        Instruction::Break(label_idx)
                    } else {
                        Instruction::BreakIf(label_idx)
                    },
                )
            }
            0x0e => {
                let (input, labels) = wasm_vec(LabelIdx::parse)(input)?;
                let (input, default_label) = LabelIdx::parse(input)?;
                (
                    input,
                    Instruction::BreakTable {
                        labels,
                        default: default_label,
                    },
                )
            }
            0x0f => (input, Instruction::Return),
            0x10 => {
                let (input, func_idx) = FuncIdx::parse(input)?;
                (input, Instruction::Call(func_idx))
            }
            0x11 => {
                let (input, func_idx) = FuncTypeIdx::parse(input)?;
                let (input, _) = tag(&[0x00][..])(input)?;
                (input, Instruction::CallIndirect(func_idx))
            }
            0x41 => {
                let (input, value) = leb128_i32(input)?;
                (input, Instruction::I32Const(value))
            }
            0x42 => {
                let (input, value) = leb128_i64(input)?;
                (input, Instruction::I64Const(value))
            }
            0x43 => {
                let (input, value) = f32(Endianness::Little)(input)?;
                (input, Instruction::F32Const(value))
            }
            0x44 => {
                let (input, value) = f64(Endianness::Little)(input)?;
                (input, Instruction::F64Const(value))
            }
            0x6a => (input, Instruction::I32Add),
            _ => panic!("Invalid instruction: 0x{:x}", value),
        };
        Ok((input, instruction))
    }
}
