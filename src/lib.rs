pub mod section;
pub mod types;
pub const VERSION: u32 = 1;

pub mod module;
pub mod runtime;
pub mod wasi;

use crate::section::{Section, SectionType};
use nom::{
    bytes::complete::tag,
    combinator::cut,
    error::{ContextError, ParseError},
    IResult, Parser,
};
use std::collections::HashMap;

pub fn repeat_until_empty<'a, T, F, E>(
    mut parse: F,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<T>, E>
where
    T: 'a,
    F: Parser<&'a [u8], T, E>,
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
{
    move |mut input| {
        let mut results = vec![];
        while !input.is_empty() {
            let result;
            (input, result) = parse.parse(input)?;
            results.push(result);
        }
        Ok((input, results))
    }
}

pub fn parse_sections(input: &[u8]) -> IResult<&[u8], HashMap<SectionType, Section>> {
    let (input, _) = tag("\0asm")(input)?;

    let (input, _) = tag(VERSION.to_le_bytes())(input)?;

    let (input, sections) =
        repeat_until_empty(cut(Section::parse.map(|s| (s.get_variant(), s))))(input)?;

    let mut sections_hashmap = HashMap::with_capacity(sections.len());

    for (t, s) in sections.into_iter() {
        if sections_hashmap.insert(t, s).is_some() {
            panic!("Duplicate sections")
        }
    }
    assert!(input.is_empty(), "{:?}", input);

    Ok((input, sections_hashmap))
}
