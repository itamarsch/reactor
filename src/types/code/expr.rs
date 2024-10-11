use std::{
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

use nom::{number::complete::u8, IResult};

use crate::{runtime::function_state::InstructionIndex, types::BlockType};

use super::{instruction::BlockIdx, Instruction};

#[derive(Debug)]
pub struct Expr {
    expr: RefCell<Instructions>,
    blocks: Rc<RefCell<Blocks>>,
}

impl Expr {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let blocks = Rc::new(RefCell::new(Blocks::empty()));
        let (input, instructions) = Instructions::parse(input, blocks.clone())?;
        Ok((
            input,
            Self {
                expr: RefCell::new(instructions),
                blocks,
            },
        ))
    }

    pub fn get_block_type(&self, block_idx: BlockIdx) -> Ref<BlockType> {
        let block_ref = self.blocks.deref().borrow();
        Ref::map(block_ref, |blocks| &blocks.get(block_idx).1)
    }

    pub fn from_raw_instructions(instructions: Vec<Instruction>) -> Self {
        Self {
            expr: RefCell::new(Instructions(instructions)),
            blocks: Rc::new(RefCell::new(Blocks::empty())),
        }
    }

    pub fn get_instruction(&self, state: InstructionIndex) -> Ref<Instruction> {
        match state {
            InstructionIndex::IndexInFunction(i) => {
                let expr = self.expr.borrow();
                Ref::map(expr, |expr| &expr[i])
            }
            InstructionIndex::IndexInBlock(block_idx, i) => {
                let blocks = self.blocks.deref().borrow();
                Ref::map(blocks, |blocks| {
                    let current_block = &blocks.get(block_idx);
                    &current_block.instructions()[i]
                })
            }
        }
    }
    pub fn done(&self, state: InstructionIndex) -> bool {
        match state {
            InstructionIndex::IndexInFunction(i) => self.expr.borrow().0.len() == i,
            InstructionIndex::IndexInBlock(block_idx, i) => {
                self.blocks
                    .deref()
                    .borrow()
                    .get(block_idx)
                    .instructions()
                    .len()
                    == i
            }
        }
    }
}

#[derive(Debug)]
pub struct Instructions(pub Vec<Instruction>);

impl Deref for Instructions {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Instructions {
    pub fn empty() -> Self {
        Self(vec![])
    }

    fn parse_inner(
        mut input: &[u8],
        blocks: Rc<RefCell<Blocks>>,
        reached_end: impl Fn(u8) -> (bool, u8),
    ) -> IResult<&[u8], (Self, u8)> {
        if input.is_empty() {
            return Ok((input, (Self::empty(), 0x0B)));
        }
        let mut instructions = vec![];
        let ending_byte = loop {
            let instruction;
            let end = reached_end(input[0]);
            if end.0 {
                (input, _) = u8(input)?;
                break end.1;
            }
            (input, instruction) = Instruction::parse(input, blocks.clone())?;
            instructions.push(instruction);
        };
        Ok((input, (Self(instructions), ending_byte)))
    }

    pub fn parse(input: &[u8], blocks: Rc<RefCell<Blocks>>) -> IResult<&[u8], Self> {
        let (input, (expr, 0x0B)) = Self::parse_inner(input, blocks, |v| (v == 0x0B, v))? else {
            unreachable!()
        };
        Ok((input, expr))
    }

    pub fn parse_if<'a, 'b>(
        input: &'a [u8],
        blocks: Rc<RefCell<Blocks>>,
    ) -> IResult<&'b [u8], (Self, Self)>
    where
        'a: 'b,
    {
        let (input, (if_expr, end)) =
            Self::parse_inner(input, blocks.clone(), |v| (v == 0x0B || v == 0x05, v))?;

        let (input, else_expr) = if end == 0x05 {
            let (input, (else_expr, 0x0B)) =
                Self::parse_inner(input, blocks.clone(), |v| (v == 0x0B, v))?
            else {
                unreachable!()
            };
            (input, else_expr)
        } else if end == 0x0B {
            (input, Self::empty())
        } else {
            unreachable!()
        };

        Ok((input, (if_expr, else_expr)))
    }
}

#[derive(Debug)]
pub struct Block(Instructions, BlockType);
impl Block {
    pub fn instructions(&self) -> &Instructions {
        &self.0
    }

    pub fn block_type(&self) -> BlockType {
        self.1
    }
}

#[derive(Debug)]
pub struct Blocks(Vec<Block>);
impl Blocks {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn add(&mut self, expr: Instructions, block_type: BlockType) -> BlockIdx {
        let new_idx = BlockIdx(self.0.len());
        self.0.push(Block(expr, block_type));
        new_idx
    }

    pub fn get(&self, BlockIdx(block_idx): BlockIdx) -> &Block {
        &self.0[block_idx]
    }
}
