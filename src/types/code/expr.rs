use nom::{bytes::complete::tag, IResult};

use super::Instruction;

#[derive(Debug)]
pub struct Expr(pub Vec<Instruction>);
impl Expr {
    pub fn parse(mut input: &[u8]) -> IResult<&[u8], Expr> {
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
        Ok((input, Expr(instructions)))
    }
}
