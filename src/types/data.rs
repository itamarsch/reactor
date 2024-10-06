use nom::IResult;
use nom_leb128::leb128_u32;

#[derive(Debug)]
pub struct DataIdx(pub u32);
impl DataIdx {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DataIdx> {
        leb128_u32(input).map(|(input, value)| (input, DataIdx(value)))
    }
}
