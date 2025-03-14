#[cfg(test)]
mod tests {

    use nom::Err;
    use nom::IResult;
    use nom::error::Error;
    use nom::error::ErrorKind;

    #[test]
    fn test_double() {
        use nom::number::complete::double;

        let parser = |s| double(s);

        assert_eq!(parser("11e-1"), Ok(("", 1.1)));
        assert_eq!(parser("123E-02"), Ok(("", 1.23)));
        assert_eq!(parser("123K-01"), Ok(("K-01", 123.0)));
        assert_eq!(parser("abc"), Err(Err::Error(("abc", ErrorKind::Float))));
        assert_eq!(parser("-12.34"), Ok(("", -12.34)));
    }

    #[test]
    fn test_not_space() {
        use nom::bytes::complete::is_not;

        fn not_space(s: &str) -> IResult<&str, &str> {
            is_not(" \t\r\n")(s)
        }

        assert_eq!(not_space("Hello, World!"), Ok((" World!", "Hello,")));
        assert_eq!(not_space("Sometimes\t"), Ok(("\t", "Sometimes")));
        assert_eq!(not_space("Nospace"), Ok(("", "Nospace")));
        assert_eq!(
            not_space(""),
            Err(Err::Error(Error::new("", ErrorKind::IsNot)))
        );
    }

    #[test]
    fn test_pair() {
        use nom::bytes::complete::tag;
        use nom::sequence::pair;
        use nom::{Err, Parser, error::ErrorKind};

        let mut parser = pair(tag("abc"), tag("efg"));

        assert_eq!(parser.parse("abcefg"), Ok(("", ("abc", "efg"))));
        assert_eq!(parser.parse("abcefghij"), Ok(("hij", ("abc", "efg"))));
        assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
        assert_eq!(
            parser.parse("123"),
            Err(Err::Error(("123", ErrorKind::Tag)))
        );

        assert_eq!(
            parser.parse("abcxyz"),
            Err(Err::Error(("xyz", ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_char() {
        use nom::character::complete::char;

        fn parser(i: &str) -> IResult<&str, char> {
            char('a')(i)
        }

        assert_eq!(parser("abc"), Ok(("bc", 'a')));
        assert_eq!(
            parser(" abc"),
            Err(Err::Error(Error::new(" abc", ErrorKind::Char)))
        );
        assert_eq!(
            parser("bc"),
            Err(Err::Error(Error::new("bc", ErrorKind::Char)))
        );
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Char))));
    }

    #[test]
    fn test_is_a() {
        use nom::bytes::complete::is_a;

        fn hex(s: &str) -> IResult<&str, &str> {
            is_a("1234567890ABCDEF")(s)
        }

        assert_eq!(hex("123 and voila"), Ok((" and voila", "123")));
        assert_eq!(hex("DEADBEEF and others"), Ok((" and others", "DEADBEEF")));
        assert_eq!(hex("BADBABEsomething"), Ok(("something", "BADBABE")));
        assert_eq!(hex("D15EA5E"), Ok(("", "D15EA5E")));
        assert_eq!(hex(""), Err(Err::Error(Error::new("", ErrorKind::IsA))));
    }
}
