use nom::{
    bytes::complete::tag,
    number::{
        complete::{f32, f64, u8},
        Endianness,
    },
    IResult,
};
use nom_leb128::{leb128_i32, leb128_i64, leb128_u32};

use crate::types::{
    wasm_vec, BlockType, DataIdx, ElementIdx, Expr, FuncIdx, FuncTypeIdx, GlobalIdx, LabelIdx,
    LocalIdx, MemoryArgument, RefType, TableIdx, ValueType,
};

#[derive(Debug)]
pub enum Instruction {
    Unreachable,
    Nop,
    Block(BlockType, Expr),
    Loop(BlockType, Expr),

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
    CallIndirect(FuncTypeIdx, TableIdx),
    Drop,
    Select,
    SelectTyped(Vec<ValueType>),
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),

    TableGet(TableIdx),
    TableSet(TableIdx),

    I32Load(MemoryArgument),
    I64Load(MemoryArgument),
    F32Load(MemoryArgument),
    F64Load(MemoryArgument),
    I32Load8S(MemoryArgument),
    I32Load8U(MemoryArgument),
    I32Load16S(MemoryArgument),
    I32Load16U(MemoryArgument),
    I64Load8S(MemoryArgument),
    I64Load8U(MemoryArgument),
    I64Load16S(MemoryArgument),
    I64Load16U(MemoryArgument),
    I64Load32S(MemoryArgument),
    I64Load32U(MemoryArgument),
    I32Store(MemoryArgument),
    I64Store(MemoryArgument),
    F32Store(MemoryArgument),
    F64Store(MemoryArgument),
    I32Store8(MemoryArgument),
    I32Store16(MemoryArgument),
    I64Store8(MemoryArgument),
    I64Store16(MemoryArgument),
    I64Store32(MemoryArgument),

    MemorySize,
    MemoryGrow,

    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I64Clz,
    I64Ctz,
    I64Popcnt,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,
    I64Rotl,
    I64Rotr,
    F32Abs,
    F32Neg,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Min,
    F32Max,
    F32Copysign,
    F64Abs,
    F64Neg,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Min,
    F64Max,
    F64Copysign,
    I32WrapI64,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64ExtendI32S,
    I64ExtendI32U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F32DemoteF64,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
    F64PromoteF32,
    I32ReinterpretF32,
    I64ReinterpretF64,
    F32ReinterpretI32,
    F64ReinterpretI64,
    I32Extend8S,
    I32Extend16S,
    I64Extend8S,
    I64Extend16S,
    I64Extend32S,

    PushNullRef(RefType),
    RefIsNull,
    PushFuncRef(FuncIdx),

    MemoryInit(DataIdx),
    DataDrop(DataIdx),
    Memcpy,
    Memfill,

    TableInit(ElementIdx, TableIdx),
    ElementDrop(ElementIdx),
    TableCopy(TableIdx, TableIdx),
    TableGrow(TableIdx),
    TableSize(TableIdx),
    TableFill(TableIdx),
}

impl Instruction {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Instruction> {
        let (input, value) = u8(input)?;
        let (input, instruction) = match value {
            0x00 => (input, Instruction::Unreachable),
            0x01 => (input, Instruction::Nop),
            0x02 | 0x03 => {
                let (mut input, block_type) = BlockType::parse(input)?;
                let (input, expr) = Expr::parse(input)?;

                (
                    input,
                    if value == 0x02 {
                        Instruction::Block(block_type, expr)
                    } else {
                        Instruction::Loop(block_type, expr)
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
                let (input, table_idx) = TableIdx::parse(input)?;
                (input, Instruction::CallIndirect(func_idx, table_idx))
            }
            0x1A => (input, Instruction::Drop),
            0x1B => (input, Instruction::Select),
            0x1C => {
                let (input, operand_types) = wasm_vec(ValueType::parse)(input)?;
                (input, Instruction::SelectTyped(operand_types))
            }
            0x20..=0x22 => {
                let (input, local_idx) = LocalIdx::parse(input)?;
                (
                    input,
                    match value {
                        0x20 => Instruction::LocalGet(local_idx),
                        0x21 => Instruction::LocalSet(local_idx),
                        0x22 => Instruction::LocalTee(local_idx),
                        _ => unreachable!(),
                    },
                )
            }
            0x23 | 0x24 => {
                let (input, global_idx) = GlobalIdx::parse(input)?;
                (
                    input,
                    if value == 0x23 {
                        Instruction::GlobalGet(global_idx)
                    } else {
                        Instruction::GlobalSet(global_idx)
                    },
                )
            }
            0x25 | 0x26 => {
                let (input, global_idx) = TableIdx::parse(input)?;
                (
                    input,
                    if value == 0x25 {
                        Instruction::TableGet(global_idx)
                    } else {
                        Instruction::TableSet(global_idx)
                    },
                )
            }
            0x28..=0x3E => {
                let (input, memory_argument) = MemoryArgument::parse(input)?;

                let instruction = match value {
                    0x28 => Instruction::I32Load(memory_argument),
                    0x29 => Instruction::I64Load(memory_argument),
                    0x2A => Instruction::F32Load(memory_argument),
                    0x2B => Instruction::F64Load(memory_argument),
                    0x2C => Instruction::I32Load8S(memory_argument),
                    0x2D => Instruction::I32Load8U(memory_argument),
                    0x2E => Instruction::I32Load16S(memory_argument),
                    0x2F => Instruction::I32Load16U(memory_argument),
                    0x30 => Instruction::I64Load8S(memory_argument),
                    0x31 => Instruction::I64Load8U(memory_argument),
                    0x32 => Instruction::I64Load16S(memory_argument),
                    0x33 => Instruction::I64Load16U(memory_argument),
                    0x34 => Instruction::I64Load32S(memory_argument),
                    0x35 => Instruction::I64Load32U(memory_argument),
                    0x36 => Instruction::I32Store(memory_argument),
                    0x37 => Instruction::I64Store(memory_argument),
                    0x38 => Instruction::F32Store(memory_argument),
                    0x39 => Instruction::F64Store(memory_argument),
                    0x3A => Instruction::I32Store8(memory_argument),
                    0x3B => Instruction::I32Store16(memory_argument),
                    0x3C => Instruction::I64Store8(memory_argument),
                    0x3D => Instruction::I64Store16(memory_argument),
                    0x3E => Instruction::I64Store32(memory_argument),
                    _ => unreachable!(),
                };
                (input, instruction)
            }
            0x3F | 0x40 => {
                let (input, _) = tag(&[0][..])(input)?;
                let instruction = match value {
                    0x3f => Instruction::MemorySize,
                    0x40 => Instruction::MemoryGrow,
                    _ => unreachable!(),
                };
                (input, instruction)
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
            0x45 => (input, Instruction::I32Eqz),
            0x46 => (input, Instruction::I32Eq),
            0x47 => (input, Instruction::I32Ne),
            0x48 => (input, Instruction::I32LtS),
            0x49 => (input, Instruction::I32LtU),
            0x4A => (input, Instruction::I32GtS),
            0x4B => (input, Instruction::I32GtU),
            0x4C => (input, Instruction::I32LeS),
            0x4D => (input, Instruction::I32LeU),
            0x4E => (input, Instruction::I32GeS),
            0x4F => (input, Instruction::I32GeU),
            0x50 => (input, Instruction::I64Eqz),
            0x51 => (input, Instruction::I64Eq),
            0x52 => (input, Instruction::I64Ne),
            0x53 => (input, Instruction::I64LtS),
            0x54 => (input, Instruction::I64LtU),
            0x55 => (input, Instruction::I64GtS),
            0x56 => (input, Instruction::I64GtU),
            0x57 => (input, Instruction::I64LeS),
            0x58 => (input, Instruction::I64LeU),
            0x59 => (input, Instruction::I64GeS),
            0x5A => (input, Instruction::I64GeU),
            0x5B => (input, Instruction::F32Eq),
            0x5C => (input, Instruction::F32Ne),
            0x5D => (input, Instruction::F32Lt),
            0x5E => (input, Instruction::F32Gt),
            0x5F => (input, Instruction::F32Le),
            0x60 => (input, Instruction::F32Ge),
            0x61 => (input, Instruction::F64Eq),
            0x62 => (input, Instruction::F64Ne),
            0x63 => (input, Instruction::F64Lt),
            0x64 => (input, Instruction::F64Gt),
            0x65 => (input, Instruction::F64Le),
            0x66 => (input, Instruction::F64Ge),
            0x67 => (input, Instruction::I32Clz),
            0x68 => (input, Instruction::I32Ctz),
            0x69 => (input, Instruction::I32Popcnt),
            0x6A => (input, Instruction::I32Add),
            0x6B => (input, Instruction::I32Sub),
            0x6C => (input, Instruction::I32Mul),
            0x6D => (input, Instruction::I32DivS),
            0x6E => (input, Instruction::I32DivU),
            0x6F => (input, Instruction::I32RemS),
            0x70 => (input, Instruction::I32RemU),
            0x71 => (input, Instruction::I32And),
            0x72 => (input, Instruction::I32Or),
            0x73 => (input, Instruction::I32Xor),
            0x74 => (input, Instruction::I32Shl),
            0x75 => (input, Instruction::I32ShrS),
            0x76 => (input, Instruction::I32ShrU),
            0x77 => (input, Instruction::I32Rotl),
            0x78 => (input, Instruction::I32Rotr),
            0x79 => (input, Instruction::I64Clz),
            0x7A => (input, Instruction::I64Ctz),
            0x7B => (input, Instruction::I64Popcnt),
            0x7C => (input, Instruction::I64Add),
            0x7D => (input, Instruction::I64Sub),
            0x7E => (input, Instruction::I64Mul),
            0x7F => (input, Instruction::I64DivS),
            0x80 => (input, Instruction::I64DivU),
            0x81 => (input, Instruction::I64RemS),
            0x82 => (input, Instruction::I64RemU),
            0x83 => (input, Instruction::I64And),
            0x84 => (input, Instruction::I64Or),
            0x85 => (input, Instruction::I64Xor),
            0x86 => (input, Instruction::I64Shl),
            0x87 => (input, Instruction::I64ShrS),
            0x88 => (input, Instruction::I64ShrU),
            0x89 => (input, Instruction::I64Rotl),
            0x8A => (input, Instruction::I64Rotr),
            0x8B => (input, Instruction::F32Abs),
            0x8C => (input, Instruction::F32Neg),
            0x8D => (input, Instruction::F32Ceil),
            0x8E => (input, Instruction::F32Floor),
            0x8F => (input, Instruction::F32Trunc),
            0x90 => (input, Instruction::F32Nearest),
            0x91 => (input, Instruction::F32Sqrt),
            0x92 => (input, Instruction::F32Add),
            0x93 => (input, Instruction::F32Sub),
            0x94 => (input, Instruction::F32Mul),
            0x95 => (input, Instruction::F32Div),
            0x96 => (input, Instruction::F32Min),
            0x97 => (input, Instruction::F32Max),
            0x98 => (input, Instruction::F32Copysign),
            0x99 => (input, Instruction::F64Abs),
            0x9A => (input, Instruction::F64Neg),
            0x9B => (input, Instruction::F64Ceil),
            0x9C => (input, Instruction::F64Floor),
            0x9D => (input, Instruction::F64Trunc),
            0x9E => (input, Instruction::F64Nearest),
            0x9F => (input, Instruction::F64Sqrt),
            0xA0 => (input, Instruction::F64Add),
            0xA1 => (input, Instruction::F64Sub),
            0xA2 => (input, Instruction::F64Mul),
            0xA3 => (input, Instruction::F64Div),
            0xA4 => (input, Instruction::F64Min),
            0xA5 => (input, Instruction::F64Max),
            0xA6 => (input, Instruction::F64Copysign),
            0xA7 => (input, Instruction::I32WrapI64),
            0xA8 => (input, Instruction::I32TruncF32S),
            0xA9 => (input, Instruction::I32TruncF32U),
            0xAA => (input, Instruction::I32TruncF64S),
            0xAB => (input, Instruction::I32TruncF64U),
            0xAC => (input, Instruction::I64ExtendI32S),
            0xAD => (input, Instruction::I64ExtendI32U),
            0xAE => (input, Instruction::I64TruncF32S),
            0xAF => (input, Instruction::I64TruncF32U),
            0xB0 => (input, Instruction::I64TruncF64S),
            0xB1 => (input, Instruction::I64TruncF64U),
            0xB2 => (input, Instruction::F32ConvertI32S),
            0xB3 => (input, Instruction::F32ConvertI32U),
            0xB4 => (input, Instruction::F32ConvertI64S),
            0xB5 => (input, Instruction::F32ConvertI64U),
            0xB6 => (input, Instruction::F32DemoteF64),
            0xB7 => (input, Instruction::F64ConvertI32S),
            0xB8 => (input, Instruction::F64ConvertI32U),
            0xB9 => (input, Instruction::F64ConvertI64S),
            0xBA => (input, Instruction::F64ConvertI64U),
            0xBB => (input, Instruction::F64PromoteF32),
            0xBC => (input, Instruction::I32ReinterpretF32),
            0xBD => (input, Instruction::I64ReinterpretF64),
            0xBE => (input, Instruction::F32ReinterpretI32),
            0xBF => (input, Instruction::F64ReinterpretI64),
            0xC0 => (input, Instruction::I32Extend8S),
            0xC1 => (input, Instruction::I32Extend16S),
            0xC2 => (input, Instruction::I64Extend8S),
            0xC3 => (input, Instruction::I64Extend16S),
            0xC4 => (input, Instruction::I64Extend32S),
            0xD0 => {
                let (input, ref_type) = RefType::parse(input)?;
                (input, Instruction::PushNullRef(ref_type))
            }
            0xD1 => (input, Instruction::RefIsNull),
            0xD2 => {
                let (input, func_idx) = FuncIdx::parse(input)?;
                (input, Instruction::PushFuncRef(func_idx))
            }
            0xFC => {
                let (input, opcode) = leb128_u32(input)?;
                match opcode {
                    8 => {
                        let (input, data_idx) = DataIdx::parse(input)?;
                        let (input, _) = tag(&[0][..])(input)?;
                        (input, Instruction::MemoryInit(data_idx))
                    }
                    9 => {
                        let (input, data_idx) = DataIdx::parse(input)?;
                        (input, Instruction::DataDrop(data_idx))
                    }
                    10 => {
                        let (input, _) = tag(&[0, 0][..])(input)?;
                        (input, Instruction::Memcpy)
                    }
                    11 => {
                        let (input, _) = tag(&[0][..])(input)?;
                        (input, Instruction::Memfill)
                    }
                    12 => {
                        let (input, element_idx) = ElementIdx::parse(input)?;
                        let (input, table_idx) = TableIdx::parse(input)?;
                        (input, Instruction::TableInit(element_idx, table_idx))
                    }
                    13 => {
                        let (input, element_idx) = ElementIdx::parse(input)?;
                        (input, Instruction::ElementDrop(element_idx))
                    }
                    14 => {
                        let (input, tablie_idx1) = TableIdx::parse(input)?;
                        let (input, tablie_idx2) = TableIdx::parse(input)?;
                        (input, Instruction::TableCopy(tablie_idx1, tablie_idx2))
                    }
                    15 => {
                        let (input, table_idx) = TableIdx::parse(input)?;
                        (input, Instruction::TableGrow(table_idx))
                    }
                    16 => {
                        let (input, table_idx) = TableIdx::parse(input)?;
                        (input, Instruction::TableSize(table_idx))
                    }
                    17 => {
                        let (input, table_idx) = TableIdx::parse(input)?;
                        (input, Instruction::TableFill(table_idx))
                    }
                    _ => panic!("Unknown instruction: 0x{:x} {}", value, opcode),
                }
            }
            _ => panic!("Invalid instruction: 0x{:x}", value),
        };
        Ok((input, instruction))
    }
}
