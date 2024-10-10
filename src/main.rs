use std::collections::HashMap;

use nom::{bytes::complete::tag, multi::many0, IResult, Parser};
use wasmy::{
    module::Module,
    runtime::Runtime,
    section::{Section, SectionType},
    VERSION,
};

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let file = std::fs::read(file).unwrap();
    let (_, sections) = parse_module(&file[..]).unwrap();

    let module = Module::new(sections);
    // println!("{:#?}", module);
    let runtime = Runtime::new(module);
    runtime.execute();
}

fn parse_module(input: &[u8]) -> IResult<&[u8], HashMap<SectionType, Section>> {
    let (input, _) = tag("\0asm")(input)?;

    let (input, _) = tag(VERSION.to_le_bytes())(input)?;

    let (input, sections) = many0(Section::parse.map(|s| (s.get_variant(), s)))(input)?;

    let mut sections_hashmap = HashMap::with_capacity(sections.len());

    for (t, s) in sections.into_iter() {
        if sections_hashmap.insert(t, s).is_some() {
            panic!("Duplicate sections")
        }
    }
    assert!(input.is_empty(), "{:?}", input);

    Ok((input, sections_hashmap))
}
