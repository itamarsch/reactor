use nom::{
    bytes::complete::{tag, take},
    IResult,
};
use nom_leb128::leb128_u32;

use super::value::ValueType;

#[derive(Debug)]
pub struct FuncTypeIdx(pub u32);
pub struct FuncIdx(pub u32);

#[derive(Debug)]
pub struct FuncType {
    pub params: Vec<ValueType>,
    pub returns: Vec<ValueType>,
}

impl FuncType {
    pub fn parse(input: &[u8]) -> IResult<&[u8], FuncType> {
        let (input, _) = tag([0x60])(input)?;
        let (input, param_length) = leb128_u32(input)?;
        let (input, params) = take(param_length)(input)?;
        let params: Vec<ValueType> = params
            .iter()
            .copied()
            .map(|e| e.try_into().expect("Invalid functype"))
            .collect();

        let (input, returns_length) = leb128_u32(input)?;
        let (input, returns) = take(returns_length)(input)?;
        let returns: Vec<ValueType> = returns
            .iter()
            .copied()
            .map(|e| e.try_into().expect("Invalid functype"))
            .collect();

        Ok((input, FuncType { params, returns }))
    }
}
