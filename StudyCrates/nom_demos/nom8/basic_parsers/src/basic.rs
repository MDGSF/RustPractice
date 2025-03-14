#[cfg(test)]
mod tests {

    use nom::Err;
    use nom::IResult;
    use nom::error::Error;
    use nom::error::ErrorKind;

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
    fn test_one_of() {
        use nom::character::complete::one_of;

        assert_eq!(one_of::<_, _, (&str, ErrorKind)>("abc")("b"), Ok(("", 'b')));
        assert_eq!(
            one_of::<_, _, (&str, ErrorKind)>("a")("bc"),
            Err(Err::Error(("bc", ErrorKind::OneOf)))
        );
        assert_eq!(
            one_of::<_, _, (&str, ErrorKind)>("a")(""),
            Err(Err::Error(("", ErrorKind::OneOf)))
        );
    }

    #[test]
    fn test_none_of() {
        use nom::character::complete::none_of;

        assert_eq!(
            none_of::<_, _, (&str, ErrorKind)>("abc")("z"),
            Ok(("", 'z'))
        );
        assert_eq!(
            none_of::<_, _, (&str, ErrorKind)>("ab")("a"),
            Err(Err::Error(("a", ErrorKind::NoneOf)))
        );
        assert_eq!(
            none_of::<_, _, (&str, ErrorKind)>("a")(""),
            Err(Err::Error(("", ErrorKind::NoneOf)))
        );
    }

    #[test]
    fn test_tag() {
        use nom::bytes::complete::tag;

        fn parser(s: &str) -> IResult<&str, &str> {
            tag("Hello")(s)
        }

        assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
        assert_eq!(
            parser("Something"),
            Err(Err::Error(Error::new("Something", ErrorKind::Tag)))
        );
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
    }

    #[test]
    fn test_tag_no_case() {
        use nom::bytes::complete::tag_no_case;

        fn parser(s: &str) -> IResult<&str, &str> {
            tag_no_case("hello")(s)
        }

        assert_eq!(parser("Hello, World!"), Ok((", World!", "Hello")));
        assert_eq!(parser("hello, World!"), Ok((", World!", "hello")));
        assert_eq!(parser("HeLlO, World!"), Ok((", World!", "HeLlO")));
        assert_eq!(
            parser("Something"),
            Err(Err::Error(Error::new("Something", ErrorKind::Tag)))
        );
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
    }

    #[test]
    fn test_take() {
        use nom::bytes::complete::take;

        fn take6(s: &str) -> IResult<&str, &str> {
            take(6usize)(s)
        }

        assert_eq!(take6("1234567"), Ok(("7", "123456")));
        assert_eq!(take6("things"), Ok(("", "things")));
        assert_eq!(
            take6("short"),
            Err(Err::Error(Error::new("short", ErrorKind::Eof)))
        );
        assert_eq!(take6(""), Err(Err::Error(Error::new("", ErrorKind::Eof))));
    }

    #[test]
    fn test_take_while() {
        use nom::AsChar;
        use nom::bytes::complete::take_while;

        fn alpha(s: &[u8]) -> IResult<&[u8], &[u8]> {
            take_while(AsChar::is_alpha)(s)
        }

        assert_eq!(alpha(b"latin123"), Ok((&b"123"[..], &b"latin"[..])));
        assert_eq!(alpha(b"12345"), Ok((&b"12345"[..], &b""[..])));
        assert_eq!(alpha(b"latin"), Ok((&b""[..], &b"latin"[..])));
        assert_eq!(alpha(b""), Ok((&b""[..], &b""[..])));
    }

    #[test]
    fn test_take_till() {
        use nom::bytes::complete::take_till;

        fn till_colon(s: &str) -> IResult<&str, &str> {
            take_till(|c| c == ':')(s)
        }

        assert_eq!(till_colon("latin:123"), Ok((":123", "latin")));
        assert_eq!(till_colon(":empty matched"), Ok((":empty matched", ""))); //allowed
        assert_eq!(till_colon("12345"), Ok(("", "12345")));
        assert_eq!(till_colon(""), Ok(("", "")));
    }

    #[test]
    fn test_take_until() {
        use nom::bytes::complete::take_until;

        fn until_eof(s: &str) -> IResult<&str, &str> {
            take_until("eof")(s)
        }

        assert_eq!(until_eof("hello, worldeof"), Ok(("eof", "hello, world")));
        assert_eq!(
            until_eof("hello, world"),
            Err(Err::Error(Error::new("hello, world", ErrorKind::TakeUntil)))
        );
        assert_eq!(
            until_eof(""),
            Err(Err::Error(Error::new("", ErrorKind::TakeUntil)))
        );
        assert_eq!(until_eof("1eof2eof"), Ok(("eof2eof", "1")));
    }

    #[test]
    fn test_alt() {
        use nom::Parser;
        use nom::branch::alt;
        use nom::character::complete::{alpha1, digit1};
        fn parser(input: &str) -> IResult<&str, &str> {
            alt((alpha1, digit1)).parse(input)
        }

        // the first parser, alpha1, recognizes the input
        assert_eq!(parser("abc"), Ok(("", "abc")));

        // the first parser returns an error, so alt tries the second one
        assert_eq!(parser("123456"), Ok(("", "123456")));

        // both parsers failed, and with the default error type, alt will return the last error
        assert_eq!(
            parser(" "),
            Err(Err::Error(Error::new(" ", ErrorKind::Digit)))
        );
    }

    #[test]
    fn test_permutation_01() {
        use nom::Parser;
        use nom::branch::permutation;
        use nom::character::complete::{alpha1, digit1};
        fn parser(input: &str) -> IResult<&str, (&str, &str)> {
            permutation((alpha1, digit1)).parse(input)
        }

        // permutation recognizes alphabetic characters then digit
        assert_eq!(parser("abc123"), Ok(("", ("abc", "123"))));

        // but also in inverse order
        assert_eq!(parser("123abc"), Ok(("", ("abc", "123"))));

        // it will fail if one of the parsers failed
        assert_eq!(
            parser("abc;"),
            Err(Err::Error(Error::new(";", ErrorKind::Digit)))
        );
    }

    #[test]
    fn test_permutation_02() {
        use nom::Parser;
        use nom::branch::permutation;
        use nom::character::complete::{anychar, char};

        fn parser(input: &str) -> IResult<&str, (char, char)> {
            permutation((anychar, char('a'))).parse(input)
        }

        // anychar parses 'b', then char('a') parses 'a'
        assert_eq!(parser("ba"), Ok(("", ('b', 'a'))));

        // anychar parses 'a', then char('a') fails on 'b',
        // even though char('a') followed by anychar would succeed
        assert_eq!(
            parser("ab"),
            Err(Err::Error(Error::new("b", ErrorKind::Char)))
        );
    }

    #[test]
    fn test_delimited() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::sequence::delimited;

        let mut parser = delimited(tag("("), tag("abc"), tag(")"));

        assert_eq!(parser.parse("(abc)"), Ok(("", "abc")));
        assert_eq!(parser.parse("(abc)def"), Ok(("def", "abc")));
        assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
        assert_eq!(
            parser.parse("123"),
            Err(Err::Error(("123", ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_preceded() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::sequence::preceded;

        let mut parser = preceded(tag("abc"), tag("efg"));

        assert_eq!(parser.parse("abcefg"), Ok(("", "efg")));
        assert_eq!(parser.parse("abcefghij"), Ok(("hij", "efg")));
        assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
        assert_eq!(
            parser.parse("123"),
            Err(Err::Error(("123", ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_terminated() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::sequence::terminated;

        let mut parser = terminated(tag("abc"), tag("efg"));

        assert_eq!(parser.parse("abcefg"), Ok(("", "abc")));
        assert_eq!(parser.parse("abcefghij"), Ok(("hij", "abc")));
        assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
        assert_eq!(
            parser.parse("123"),
            Err(Err::Error(("123", ErrorKind::Tag)))
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
    fn test_separated_pair() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::sequence::separated_pair;

        let mut parser = separated_pair(tag("abc"), tag("|"), tag("efg"));

        assert_eq!(parser.parse("abc|efg"), Ok(("", ("abc", "efg"))));
        assert_eq!(parser.parse("abc|efghij"), Ok(("hij", ("abc", "efg"))));
        assert_eq!(parser.parse(""), Err(Err::Error(("", ErrorKind::Tag))));
        assert_eq!(
            parser.parse("123"),
            Err(Err::Error(("123", ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_count() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::multi::count;

        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            count(tag("abc"), 2).parse(s)
        }

        assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(
            parser("abc123"),
            Err(Err::Error(Error::new("123", ErrorKind::Tag)))
        );
        assert_eq!(
            parser("123123"),
            Err(Err::Error(Error::new("123123", ErrorKind::Tag)))
        );
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
        assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
    }

    #[test]
    fn test_many0() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::multi::many0;

        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many0(tag("abc")).parse(s)
        }

        assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
        assert_eq!(parser("123123"), Ok(("123123", vec![])));
        assert_eq!(parser(""), Ok(("", vec![])));
    }

    #[test]
    fn test_many0_count() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::multi::many0_count;

        fn parser(s: &str) -> IResult<&str, usize> {
            many0_count(tag("abc")).parse(s)
        }

        assert_eq!(parser("abcabc"), Ok(("", 2)));
        assert_eq!(parser("abc123"), Ok(("123", 1)));
        assert_eq!(parser("123123"), Ok(("123123", 0)));
        assert_eq!(parser(""), Ok(("", 0)));
    }

    #[test]
    fn test_many_m_n() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::multi::many_m_n;

        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            many_m_n(0, 2, tag("abc")).parse(s)
        }

        assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
        assert_eq!(parser("123123"), Ok(("123123", vec![])));
        assert_eq!(parser(""), Ok(("", vec![])));
        assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
    }

    #[test]
    fn test_many_till() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::multi::many_till;

        fn parser(s: &str) -> IResult<&str, (Vec<&str>, &str)> {
            many_till(tag("abc"), tag("end")).parse(s)
        }

        assert_eq!(parser("abcabcend"), Ok(("", (vec!["abc", "abc"], "end"))));
        assert_eq!(
            parser("abc123end"),
            Err(Err::Error(Error::new("123end", ErrorKind::Tag)))
        );
        assert_eq!(
            parser("123123end"),
            Err(Err::Error(Error::new("123123end", ErrorKind::Tag)))
        );
        assert_eq!(parser(""), Err(Err::Error(Error::new("", ErrorKind::Tag))));
        assert_eq!(parser("abcendefg"), Ok(("efg", (vec!["abc"], "end"))));
    }

    #[test]
    fn test_separated_list0() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::multi::separated_list0;

        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            separated_list0(tag("|"), tag("abc")).parse(s)
        }

        assert_eq!(parser("abc|abc|abc"), Ok(("", vec!["abc", "abc", "abc"])));
        assert_eq!(parser("abc123abc"), Ok(("123abc", vec!["abc"])));
        assert_eq!(parser("abc|def"), Ok(("|def", vec!["abc"])));
        assert_eq!(parser(""), Ok(("", vec![])));
        assert_eq!(parser("def|abc"), Ok(("def|abc", vec![])));
    }

    #[test]
    fn test_fold_many0() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::multi::fold_many0;

        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many0(tag("abc"), Vec::new, |mut acc: Vec<_>, item| {
                acc.push(item);
                acc
            })
            .parse(s)
        }

        assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
        assert_eq!(parser("123123"), Ok(("123123", vec![])));
        assert_eq!(parser(""), Ok(("", vec![])));
    }

    #[test]
    fn test_fold_many_m_n() {
        use nom::Parser;
        use nom::bytes::complete::tag;
        use nom::multi::fold_many_m_n;

        fn parser(s: &str) -> IResult<&str, Vec<&str>> {
            fold_many_m_n(0, 2, tag("abc"), Vec::new, |mut acc: Vec<_>, item| {
                acc.push(item);
                acc
            })
            .parse(s)
        }

        assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
        assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
        assert_eq!(parser("123123"), Ok(("123123", vec![])));
        assert_eq!(parser(""), Ok(("", vec![])));
        assert_eq!(parser("abcabcabc"), Ok(("abc", vec!["abc", "abc"])));
    }

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
}
