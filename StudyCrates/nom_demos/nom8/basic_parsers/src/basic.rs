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
}
