use nom::bytes::complete::tag_no_case;
use nom::IResult;
use std::error::Error;

fn parse_input(input: &str) -> IResult<&str, &str> {
    tag_no_case("abc")(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (leftover_input, output) = parse_input("AbCWorld")?;
    assert_eq!(leftover_input, "World");
    assert_eq!(output, "AbC");

    assert!(parse_input("defWorld").is_err());

    Ok(())
}
