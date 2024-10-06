use nom::IResult;
use nom_leb128::leb128_u32;

#[derive(Debug)]
pub struct DataCountSection {
    pub amount_of_datas: u32,
}

impl DataCountSection {
    pub fn parse(input: &[u8]) -> IResult<&[u8], DataCountSection> {
        let (input, amount_of_datas) = leb128_u32(input)?;

        Ok((input, DataCountSection { amount_of_datas }))
    }
}
