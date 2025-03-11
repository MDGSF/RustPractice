use escape8259::unescape;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::character::complete::digit0;
use nom::character::complete::digit1;
use nom::character::complete::multispace0;
use nom::character::complete::one_of;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::error::{ErrorKind, ParseError};
use nom::multi::many0;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::separated_pair;
use nom::sequence::tuple;
use nom::IResult;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum JSONParseError {
    #[error("bad integer")]
    BadInt,
    #[error("bad float")]
    BadFloat,
    #[error("bad escape sequence")]
    BadEscape,
    #[error("unknown parser error")]
    Unparseable,
}

impl<I> ParseError<I> for JSONParseError {
    fn from_error_kind(_input: I, _kind: ErrorKind) -> Self {
        JSONParseError::Unparseable
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Str(String),
    Array(Vec<Node>),
    Object(Vec<(String, Node)>),
}

fn json_bool(input: &str) -> IResult<&str, Node, JSONParseError> {
    alt((
        value(Node::Bool(false), tag("false")),
        value(Node::Bool(true), tag("true")),
    ))(input)
}

fn json_null(input: &str) -> IResult<&str, Node, JSONParseError> {
    value(Node::Null, tag("null"))(input)
}

fn digit1to9(input: &str) -> IResult<&str, char, JSONParseError> {
    one_of("123456789")(input)
}
fn uint(input: &str) -> IResult<&str, &str, JSONParseError> {
    alt((tag("0"), recognize(pair(digit1to9, digit0))))(input)
}

fn integer_body(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(pair(opt(tag("-")), uint))(input)
}

fn json_integer(input: &str) -> IResult<&str, Node, JSONParseError> {
    let (remain, raw_int) = integer_body(input)?;
    match raw_int.parse::<i64>() {
        Ok(i) => Ok((remain, Node::Integer(i))),
        Err(_) => Err(nom::Err::Failure(JSONParseError::BadInt)),
    }
}

fn frac(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(pair(tag("."), digit1))(input)
}

fn exp(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(tuple((tag("e"), opt(alt((tag("-"), tag("+")))), digit1)))(input)
}

fn float_body(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(tuple((
        opt(tag("-")),
        uint,
        alt((recognize(pair(frac, opt(exp))), exp)),
    )))(input)
}

fn json_float(input: &str) -> IResult<&str, Node, JSONParseError> {
    let (remain, raw_float) = float_body(input)?;
    match raw_float.parse::<f64>() {
        Ok(f) => Ok((remain, Node::Float(f))),
        Err(_) => Err(nom::Err::Failure(JSONParseError::BadFloat)),
    }
}

fn is_nonescaped_string_char(c: char) -> bool {
    let cv = c as u32;
    (cv >= 0x20) && (cv != 0x22) && (cv != 0x5C)
}

// One or more unescaped text characters
fn nonescaped_string(input: &str) -> IResult<&str, &str, JSONParseError> {
    take_while1(is_nonescaped_string_char)(input)
}

fn escape_code(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(pair(
        tag("\\"),
        alt((
            tag("\""),
            tag("\\"),
            tag("/"),
            tag("b"),
            tag("f"),
            tag("n"),
            tag("r"),
            tag("t"),
            tag("u"),
        )),
    ))(input)
}

fn string_body(input: &str) -> IResult<&str, &str, JSONParseError> {
    recognize(many0(alt((nonescaped_string, escape_code))))(input)
}

fn string_literal(input: &str) -> IResult<&str, String, JSONParseError> {
    let (remain, raw_string) = delimited(tag("\""), string_body, tag("\""))(input)?;

    match unescape(raw_string) {
        Ok(s) => Ok((remain, s)),
        Err(_) => Err(nom::Err::Failure(JSONParseError::BadEscape)),
    }
}

fn json_string(input: &str) -> IResult<&str, Node, JSONParseError> {
    map(string_literal, |s| Node::Str(s))(input)
}

fn json_array(input: &str) -> IResult<&str, Node, JSONParseError> {
    let parser = delimited(
        spacey(tag("[")),
        separated_list0(spacey(tag(",")), json_value),
        spacey(tag("]")),
    );
    map(parser, |v| Node::Array(v))(input)
}

// "key: value", where key and value are any JSON type.
fn object_member(input: &str) -> IResult<&str, (String, Node), JSONParseError> {
    separated_pair(string_literal, spacey(tag(":")), json_value)(input)
}

fn json_object(input: &str) -> IResult<&str, Node, JSONParseError> {
    let parser = delimited(
        spacey(tag("{")),
        separated_list0(spacey(tag(",")), object_member),
        spacey(tag("}")),
    );
    map(parser, |v| Node::Object(v))(input)
}

fn spacey<F, I, O, E>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(multispace0, f, multispace0)
}

fn json_value(input: &str) -> IResult<&str, Node, JSONParseError> {
    spacey(alt((
        json_array,
        json_object,
        json_string,
        json_float,
        json_integer,
        json_bool,
        json_null,
    )))(input)
}

pub fn parse_json(input: &str) -> Result<Node, JSONParseError> {
    let (_, result) = all_consuming(json_value)(input).map_err(|nom_err| match nom_err {
        nom::Err::Incomplete(_) => unreachable!(),
        nom::Err::Error(e) => e,
        nom::Err::Failure(e) => e,
    })?;
    Ok(result)
}

#[test]
fn test_bool() {
    assert_eq!(json_bool("false"), Ok(("", Node::Bool(false))));
    assert_eq!(json_bool("true"), Ok(("", Node::Bool(true))));
    assert!(json_bool("foo").is_err());
}

#[test]
fn test_null() {
    assert_eq!(json_null("null"), Ok(("", Node::Null)));
}

#[test]
fn test_uint() {
    assert_eq!(uint("123abc"), Ok(("abc", "123")));
}

#[test]
fn test_integer() {
    assert_eq!(json_integer("42"), Ok(("", Node::Integer(42))));
    assert_eq!(json_integer("-123"), Ok(("", Node::Integer(-123))));
    assert_eq!(json_integer("0"), Ok(("", Node::Integer(0))));
    assert_eq!(json_integer("01"), Ok(("1", Node::Integer(0))));
}

#[test]
fn test_literal() {
    assert_eq!(json_value("56"), Ok(("", Node::Integer(56))));
    assert_eq!(json_value("78.0"), Ok(("", Node::Float(78.0))));
}

#[test]
fn test_string() {
    // Plain Unicode strings with no escaping
    assert_eq!(json_string(r#""""#), Ok(("", Node::Str("".into()))));
    assert_eq!(
        json_string(r#""Hello""#),
        Ok(("", Node::Str("Hello".into())))
    );
    assert_eq!(json_string(r#""の""#), Ok(("", Node::Str("の".into()))));
    assert_eq!(json_string(r#""𝄞""#), Ok(("", Node::Str("𝄞".into()))));

    // valid 2-character escapes
    assert_eq!(
        json_string(r#""  \\  ""#),
        Ok(("", Node::Str("  \\  ".into())))
    );
    assert_eq!(
        json_string(r#""  \"  ""#),
        Ok(("", Node::Str("  \"  ".into())))
    );

    // valid 6-character escapes
    assert_eq!(
        json_string(r#""\u0000""#),
        Ok(("", Node::Str("\x00".into())))
    );
    assert_eq!(json_string(r#""\u00DF""#), Ok(("", Node::Str("ß".into()))));
    assert_eq!(
        json_string(r#""\uD834\uDD1E""#),
        Ok(("", Node::Str("𝄞".into())))
    );

    // Invalid because surrogate characters must come in pairs
    assert!(json_string(r#""\ud800""#).is_err());
    // Unknown 2-character escape
    assert!(json_string(r#""\x""#).is_err());
    // Not enough hex digits
    assert!(json_string(r#""\u""#).is_err());
    assert!(json_string(r#""\u001""#).is_err());
    // Naked control character
    assert!(json_string(r#""\x0a""#).is_err());
    // Not a JSON string because it's not wrapped in quotes
    assert!(json_string("abc").is_err());
}

#[test]
fn test_array() {
    assert_eq!(json_array("[]"), Ok(("", Node::Array(vec![]))));
    assert_eq!(
        json_array("[1]"),
        Ok(("", Node::Array(vec![Node::Integer(1)])))
    );

    let expected = Node::Array(vec![Node::Integer(1), Node::Integer(2)]);
    assert_eq!(json_array("[1,2]"), Ok(("", expected)));
}
