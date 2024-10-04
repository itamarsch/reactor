use nom::IResult;
use nom_leb128::leb128_u32;

#[derive(Debug)]
pub struct LabelIdx(pub u32);
impl LabelIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], LabelIdx> {
        leb128_u32(input).map(|(input, value)| (input, LabelIdx(value)))
    }
}
