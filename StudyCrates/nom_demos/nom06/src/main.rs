use nom::character::complete::alpha0;
use nom::IResult;
use std::error::Error;

fn parser(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (remaining, letters) = parser("abc123")?;
    assert_eq!(remaining, "123");
    assert_eq!(letters, "abc");

    Ok(())
}
