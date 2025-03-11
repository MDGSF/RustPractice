use nom::bytes::complete::tag;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::error::Error;

// This is the type we will parse into.
#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

// 1. Nom has an in-built i32 parser.
use nom::character::complete::i32;

// 2. Use the `separated_pair` parser to combine two parsers (in this case,
//    both `i32`), ignoring something in-between.
fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag(", "), i32)(input)
}

// 3. Use the `delimited` parser to apply a parser, ignoring the results
//    of two surrounding parsers.
fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (remaining, (x, y)) = delimited(tag("("), parse_integer_pair, tag(")"))(input)?;

    // Note: we could construct this by implementing `From` on `Coordinate`,
    // We don't, just so it's obvious what's happening.
    Ok((remaining, Coordinate { x, y }))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (_, parsed) = parse_coordinate("(3, 5)")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    let (_, parsed) = parse_coordinate("(2, -4)")?;
    assert_eq!(parsed, Coordinate { x: 2, y: -4 });

    let parsing_error = parse_coordinate("(3,)");
    assert!(parsing_error.is_err());

    let parsing_error = parse_coordinate("(,3)");
    assert!(parsing_error.is_err());

    let parsing_error = parse_coordinate("Ferris");
    assert!(parsing_error.is_err());
    Ok(())
}
