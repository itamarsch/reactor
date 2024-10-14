use std::{
    cell::{OnceCell, Ref, RefCell},
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
        let block_stack = Rc::new(RefCell::new(vec![]));
        let (input, instructions) =
            Instructions::parse(input, blocks.clone(), block_stack.clone())?;
        assert!(block_stack.deref().borrow().is_empty());
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

    pub fn is_block_loop(&self, block_idx: BlockIdx) -> bool {
        self.blocks.deref().borrow().get(block_idx).is_loop()
    }

    pub fn from_raw_instructions(instructions: Vec<Instruction>) -> Self {
        Self {
            expr: RefCell::new(Instructions(instructions)),
            blocks: Rc::new(RefCell::new(Blocks::empty())),
        }
    }

    pub fn amount_of_instructions_in_block(&self, block_idx: BlockIdx) -> usize {
        self.blocks.deref().borrow().get(block_idx).0.len()
    }

    pub fn get_instruction(&self, state: InstructionIndex) -> Ref<Instruction> {
        match state {
            InstructionIndex::IndexInFunction(i) => {
                let expr = self.expr.borrow();
                Ref::map(expr, |expr| &expr[i])
            }
            InstructionIndex::IndexInBlock {
                block_idx,
                index_in_block: i,
                ..
            } => {
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
            InstructionIndex::IndexInBlock {
                block_idx,
                index_in_block: i,
                ..
            } => {
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
        block_stack: Rc<RefCell<Vec<BlockIdx>>>,
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
            (input, instruction) = Instruction::parse(input, blocks.clone(), block_stack.clone())?;
            instructions.push(instruction);
        };
        Ok((input, (Self(instructions), ending_byte)))
    }

    pub fn parse(
        input: &[u8],
        blocks: Rc<RefCell<Blocks>>,
        block_stack: Rc<RefCell<Vec<BlockIdx>>>,
    ) -> IResult<&[u8], Self> {
        let (input, (expr, 0x0B)) =
            Self::parse_inner(input, blocks, block_stack, |v| (v == 0x0B, v))?
        else {
            unreachable!()
        };
        Ok((input, expr))
    }

    pub fn parse_if_block<'a, 'b>(
        input: &'a [u8],
        blocks: Rc<RefCell<Blocks>>,
        block_stack: Rc<RefCell<Vec<BlockIdx>>>,
    ) -> IResult<&'b [u8], (Self, bool)>
    where
        'a: 'b,
    {
        let (input, (if_expr, end)) =
            Self::parse_inner(input, blocks.clone(), block_stack.clone(), |v| {
                (v == 0x0B || v == 0x05, v)
            })?;

        Ok((
            input,
            (
                if_expr,
                if end == 0x05 {
                    true
                } else if end == 0x0B {
                    false
                } else {
                    unreachable!()
                },
            ),
        ))
    }
}

#[derive(Debug)]
pub struct Block(Instructions, BlockType, bool);
impl Block {
    pub fn instructions(&self) -> &Instructions {
        &self.0
    }

    pub fn block_type(&self) -> BlockType {
        self.1
    }

    pub fn is_loop(&self) -> bool {
        self.2
    }
}

#[derive(Debug)]
pub struct Blocks(Vec<OnceCell<Block>>);
impl Blocks {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn new_block(&mut self) -> BlockIdx {
        let new_idx = BlockIdx(self.0.len());
        self.0.push(OnceCell::new());
        new_idx
    }

    pub fn set_new_block(
        &mut self,
        expr: Instructions,
        block_type: BlockType,
        is_loop: bool,
        BlockIdx(block_idx): BlockIdx,
    ) {
        self.0[block_idx]
            .set(Block(expr, block_type, is_loop))
            .unwrap();
    }

    pub fn get(&self, BlockIdx(block_idx): BlockIdx) -> &Block {
        self.0[block_idx].get().unwrap()
    }
}
