use nom::{bytes::complete::tag, IResult};

use crate::types::code::instruction;

use super::Instruction;

#[derive(Debug)]
pub struct Expr(pub Vec<Instruction>);
impl Expr {
    pub fn empty() -> Self {
        Self(vec![])
    }

    fn parse_inner(
        mut input: &[u8],
        reached_end: impl Fn(u8) -> (bool, u8),
    ) -> IResult<&[u8], (Expr, u8)> {
        if input.is_empty() {
            return Ok((input, (Expr(vec![]), 0x0B)));
        }
        let mut instructions = vec![];
        let ending_byte = loop {
            let instruction;
            let end = reached_end(input[0]);
            if end.0 {
                (input, _) = tag(&[0x0B][..])(input)?;
                break end.1;
            }
            (input, instruction) = Instruction::parse(input)?;
            instructions.push(instruction);
        };
        Ok((input, (Expr(instructions), ending_byte)))
    }

    pub fn parse(input: &[u8]) -> IResult<&[u8], Expr> {
        let (input, (expr, 0x0B)) = Self::parse_inner(input, |v| (v == 0x0B, v))? else {
            unreachable!()
        };
        Ok((input, expr))
    }

    pub fn parse_if(input: &[u8]) -> IResult<&[u8], (Expr, Expr)> {
        let (input, (if_expr, end)) = Self::parse_inner(input, |v| (v == 0x0B || v == 0x05, v))?;

        let (input, else_expr) = if end == 0x05 {
            let (input, (else_expr, 0x0B)) = Self::parse_inner(input, |v| (v == 0x0B, v))? else {
                unreachable!()
            };
            (input, else_expr)
        } else if end == 0x0B {
            (input, Expr::empty())
        } else {
            unreachable!()
        };

        Ok((input, (if_expr, else_expr)))
    }
}
