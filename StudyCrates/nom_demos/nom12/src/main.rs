use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::character::complete::space0;
use nom::sequence::tuple;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

fn parse_integer_pair02(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tuple((space0, tag(", "), space0)), i32)(input)
}

fn parse_integer_pair01(input: &str) -> IResult<&str, (i32, i32)> {
    let (remaining, (x, y)) = delimited(space0, parse_integer_pair02, space0)(input)?;
    Ok((remaining, (x, y)))
}

fn parse_coordinate01(input: &str) -> IResult<&str, Coordinate> {
    let (remaining, (x, y)) = delimited(tag("("), parse_integer_pair01, tag(")"))(input)?;
    Ok((remaining, Coordinate { x, y }))
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (remaining, coord) = delimited(space0, parse_coordinate01, space0)(input)?;
    Ok((remaining, coord))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (_, parsed) = parse_coordinate("(3, 5)")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    let (_, parsed) = parse_coordinate("(2, -4)")?;
    assert_eq!(parsed, Coordinate { x: 2, y: -4 });

    let (_, parsed) = parse_coordinate(" (3, 5)")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    let (_, parsed) = parse_coordinate("(3, 5) ")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    let (_, parsed) = parse_coordinate(" (3, 5) ")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    let (_, parsed) = parse_coordinate(" (3 , 5) ")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    let (_, parsed) = parse_coordinate(" ( 3 , 5) ")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    let (_, parsed) = parse_coordinate(" (3 , 5 ) ")?;
    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    let parsing_error = parse_coordinate("(3,)");
    assert!(parsing_error.is_err());

    let parsing_error = parse_coordinate("(,3)");
    assert!(parsing_error.is_err());

    let parsing_error = parse_coordinate("Ferris");
    assert!(parsing_error.is_err());
    Ok(())
}
