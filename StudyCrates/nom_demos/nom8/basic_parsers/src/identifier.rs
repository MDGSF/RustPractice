use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1},
    combinator::recognize,
    multi::many0_count,
    sequence::pair,
};

pub fn rust_identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_identifier() {
        assert_eq!(rust_identifier("foo"), Ok(("", "foo")));
        assert_eq!(
            rust_identifier("hello_world123abc"),
            Ok(("", "hello_world123abc"))
        );
    }
}
