use nom::{bytes::complete::tag, multi::many0, IResult};
use wasmy::{section::Section, VERSION};

fn main() {
    let file = std::fs::read("./test.wasm").unwrap();
    parse_module(&file[..]).unwrap();
}

fn parse_module(input: &[u8]) -> IResult<&[u8], ()> {
    let (input, _) = tag("\0asm")(input)?;

    let (input, _) = tag(VERSION.to_le_bytes())(input)?;

    let (input, sections) = many0(Section::parse)(input)?;
    sections
        .iter()
        .map(|e| e.get_variant())
        .for_each(|e| println!("{e}"));

    Ok((input, ()))
}
