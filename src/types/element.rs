use nom::IResult;
use nom_leb128::leb128_u32;

#[derive(Debug)]
pub struct ElementIdx(pub u32);
impl ElementIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], ElementIdx> {
        leb128_u32(input).map(|(input, value)| (input, ElementIdx(value)))
    }
}
