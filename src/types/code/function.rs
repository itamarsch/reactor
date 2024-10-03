use nom::{bytes::complete::take, IResult};
use nom_leb128::leb128_u32;

use super::{instruction::Instruction, local::Locals};

#[derive(Debug)]
pub struct FunctionCode {
    pub locals: Locals,
    pub instructions: Vec<Instruction>,
}

impl FunctionCode {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FunctionCode> {
        let (input, size) = leb128_u32(input)?;
        let (rest, input) = take(size as usize)(input)?;

        let (input, locals) = Locals::parse(input)?;

        Ok((
            rest,
            FunctionCode {
                locals,
                instructions: vec![],
            },
        ))
    }
}
