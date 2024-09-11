use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::IResult;
use std::error::Error;

fn parse_abc_or_def(input: &str) -> IResult<&str, &str> {
    alt((tag("abc"), tag("def"), tag("xyz")))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (leftover_input, output) = parse_abc_or_def("abcWorld")?;
    assert_eq!(leftover_input, "World");
    assert_eq!(output, "abc");

    let (leftover_input, output) = parse_abc_or_def("xyzWorld")?;
    assert_eq!(leftover_input, "World");
    assert_eq!(output, "xyz");

    assert!(parse_abc_or_def("ghiWorld").is_err());

    Ok(())
}
