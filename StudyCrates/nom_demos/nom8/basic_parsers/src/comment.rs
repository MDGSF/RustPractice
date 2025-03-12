use crate::error::DebugError;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::combinator::value;
use nom::sequence::pair;
use nom::{IResult, Parser};

pub fn peol_comment<'a>(i: &'a str) -> IResult<&'a str, (), DebugError> {
    value(
        (), // Output is thrown away.
        pair(char('%'), is_not("\n\r")),
    )
    .parse(i)
}

pub fn pinline_comment<'a>(i: &'a str) -> IResult<&'a str, (), DebugError> {
    value(
        (), // Output is thrown away.
        (tag("(*"), take_until("*)"), tag("*)")),
    )
    .parse(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peol_comment() {
        assert_eq!(peol_comment("%abc"), Ok(("", ())));
        assert_eq!(peol_comment("%abc\n"), Ok(("\n", ())));
        assert_eq!(peol_comment("%abc\r"), Ok(("\r", ())));
        assert_eq!(peol_comment("%  abc  \n"), Ok(("\n", ())));
    }

    #[test]
    fn test_pinline_comment() {
        assert_eq!(pinline_comment("(*abc*)"), Ok(("", ())));
        assert_eq!(pinline_comment("(* hello world *)"), Ok(("", ())));
    }
}
