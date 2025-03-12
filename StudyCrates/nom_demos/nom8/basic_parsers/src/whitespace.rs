use nom::character::complete::multispace0;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::{Parser, error::ParseError};

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, O, E: ParseError<&'a str>, F>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    delimited(multispace0, inner, multispace0)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes leading whitespace,
pub fn ws_prefix<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    preceded(multispace0, inner)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes trailing whitespace, returning the output of `inner`.
pub fn ws_suffix<'a, O, E: ParseError<&'a str>, F>(
    inner: F,
) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
{
    terminated(inner, multispace0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err;
    use nom::error::ErrorKind;

    #[test]
    fn test_ws() {
        use nom::number::complete::double;
        let parser1 = |s| double(s);
        let mut parser2 = ws(parser1);
        assert_eq!(parser1("12.34"), Ok(("", 12.34)));
        assert_eq!(parser1("-12.34"), Ok(("", -12.34)));
        assert_eq!(parser1("abc"), Err(Err::Error(("abc", ErrorKind::Float))));
        assert_eq!(parser2.parse("12.34"), Ok(("", 12.34)));
        assert_eq!(parser2.parse("-12.34"), Ok(("", -12.34)));
        assert_eq!(parser2.parse("  12.34  "), Ok(("", 12.34)));
        assert_eq!(parser2.parse("  -12.34  "), Ok(("", -12.34)));
        assert_eq!(parser2.parse("12.34  "), Ok(("", 12.34)));
        assert_eq!(parser2.parse("-12.34  "), Ok(("", -12.34)));
        assert_eq!(parser2.parse("  12.34"), Ok(("", 12.34)));
        assert_eq!(parser2.parse("  -12.34"), Ok(("", -12.34)));
    }

    #[test]
    fn test_ws_prefix() {
        use nom::number::complete::double;
        let parser1 = |s| double(s);
        let mut parser2 = ws_prefix(parser1);
        assert_eq!(parser1("12.34"), Ok(("", 12.34)));
        assert_eq!(parser1("-12.34"), Ok(("", -12.34)));
        assert_eq!(parser1("abc"), Err(Err::Error(("abc", ErrorKind::Float))));
        assert_eq!(parser2.parse("12.34"), Ok(("", 12.34)));
        assert_eq!(parser2.parse("-12.34"), Ok(("", -12.34)));
        assert_eq!(parser2.parse("  12.34  "), Ok(("  ", 12.34)));
        assert_eq!(parser2.parse("  -12.34  "), Ok(("  ", -12.34)));
        assert_eq!(parser2.parse("12.34  "), Ok(("  ", 12.34)));
        assert_eq!(parser2.parse("-12.34  "), Ok(("  ", -12.34)));
        assert_eq!(parser2.parse("  12.34"), Ok(("", 12.34)));
        assert_eq!(parser2.parse("  -12.34"), Ok(("", -12.34)));
    }

    #[test]
    fn test_ws_suffix() {
        use nom::number::complete::double;
        let parser1 = |s| double(s);
        let mut parser2 = ws_suffix(parser1);
        assert_eq!(parser1("12.34"), Ok(("", 12.34)));
        assert_eq!(parser1("-12.34"), Ok(("", -12.34)));
        assert_eq!(parser1("abc"), Err(Err::Error(("abc", ErrorKind::Float))));
        assert_eq!(parser2.parse("12.34"), Ok(("", 12.34)));
        assert_eq!(parser2.parse("-12.34"), Ok(("", -12.34)));
        assert_eq!(parser2.parse("12.34  "), Ok(("", 12.34)));
        assert_eq!(parser2.parse("-12.34  "), Ok(("", -12.34)));
        assert_eq!(
            parser2.parse("  12.34"),
            Err(Err::Error(("  12.34", ErrorKind::Float)))
        );
        assert_eq!(
            parser2.parse("  -12.34"),
            Err(Err::Error(("  -12.34", ErrorKind::Float)))
        );
    }
}
