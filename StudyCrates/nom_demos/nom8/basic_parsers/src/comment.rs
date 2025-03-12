use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::combinator::value;
use nom::sequence::pair;
use nom::{IResult, Parser, error::ParseError};

pub fn peol_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        pair(char('%'), is_not("\n\r")),
    )
    .parse(i)
}

pub fn pinline_comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (), // Output is thrown away.
        (tag("(*"), take_until("*)"), tag("*)")),
    )
    .parse(i)
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_peol_comment() {}
}
