use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::IResult;

fn parser(s: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("abc"))(s)
}

fn main() {
    assert_eq!(parser("abcabc"), Ok(("", vec!["abc", "abc"])));
    assert_eq!(parser("abc123"), Ok(("123", vec!["abc"])));
    assert_eq!(parser("123123"), Ok(("123123", vec![])));
    assert_eq!(parser(""), Ok(("", vec![])));
}
