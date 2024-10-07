use nom::{
    bytes::complete::{tag, take},
    multi::many0,
    IResult,
};
use nom_leb128::leb128_u32;

use super::{instruction::Instruction, local::LocalTypes};

#[derive(Debug)]
pub struct FunctionCode {
    pub locals: LocalTypes,
    pub instructions: Vec<Instruction>,
}

impl FunctionCode {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FunctionCode> {
        let (input, size) = leb128_u32(input)?;
        let (rest, input) = take(size as usize - 1)(input)?;

        let (input, locals) = LocalTypes::parse(input)?;

        let (input, instructions) = many0(Instruction::parse)(input)?;
        assert!(input.is_empty());

        let (rest, _) = tag([0x0B])(rest)?;
        Ok((
            rest,
            FunctionCode {
                locals,
                instructions,
            },
        ))
    }
}
