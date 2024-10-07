use nom::{bytes::complete::tag, multi::many0, IResult};
use wasmy::{module::take_functions, section::Section, VERSION};

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let file = std::fs::read(file).unwrap();
    parse_module(&file[..]).unwrap();
}

fn parse_module(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("\0asm")(input)?;

    let (input, _) = tag(VERSION.to_le_bytes())(input)?;

    let (input, sections) = many0(Section::parse)(input)?;
    sections.iter().for_each(|e| println!("{:?}", e));
    take_functions(sections);

    Ok((input, ()))
}
