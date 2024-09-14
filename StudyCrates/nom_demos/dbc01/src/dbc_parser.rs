use escape8259::unescape;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::bytes::complete::take_while;
use nom::bytes::complete::take_while1;
use nom::character::complete::digit0;
use nom::character::complete::digit1;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::one_of;
use nom::character::complete::satisfy;
use nom::character::complete::space0;
use nom::character::is_alphabetic;
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
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum DbcParseError {
    #[error("bad escape sequence")]
    BadEscape,
    #[error("unknown parser error")]
    Unparseable,
}

impl<I> ParseError<I> for DbcParseError {
    fn from_error_kind(_input: I, _kind: ErrorKind) -> Self {
        DbcParseError::Unparseable
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

/// Version identifier of the DBC file.
/// Format: `VERSION "<VersionIdentifier>"`
#[derive(PartialEq, Debug, Clone)]
pub struct DbcVersion(String);

/// Names used throughout the DBC file.
///
/// Format:
///
/// ```text
/// NS_:
///     BS_
///     CM_
///     ...
/// ```
/// */
#[derive(PartialEq, Debug, Clone)]
pub struct DbcNames(Vec<String>);

/// Bus configuration.
/// Format:: `BS_: <Speed>`
/// Speed in kBit/s
#[derive(PartialEq, Debug, Clone)]
pub struct DbcBusConfiguration(f64);

#[derive(PartialEq, Debug, Clone)]
pub struct OneDbc {
    pub version: DbcVersion,
    pub names: DbcNames,
    pub bus_configuratin: Option<DbcBusConfiguration>,
}

fn spacey<F, I, O, E>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(space0, f, space0)
}

fn multispacey<F, I, O, E>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: FnMut(I) -> IResult<I, O, E>,
    I: nom::InputTakeAtPosition,
    <I as nom::InputTakeAtPosition>::Item: nom::AsChar + Clone,
    E: nom::error::ParseError<I>,
{
    delimited(multispace0, f, multispace0)
}

fn is_nonescaped_string_char(c: char) -> bool {
    let cv = c as u32;
    (cv >= 0x20) && (cv != 0x22) && (cv != 0x5C)
}

// One or more unescaped text characters
fn nonescaped_string(input: &str) -> IResult<&str, &str, DbcParseError> {
    take_while1(is_nonescaped_string_char)(input)
}

fn escape_code(input: &str) -> IResult<&str, &str, DbcParseError> {
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

fn string_body(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(many0(alt((nonescaped_string, escape_code))))(input)
}

fn string_literal(input: &str) -> IResult<&str, String, DbcParseError> {
    let (remain, raw_string) = delimited(tag("\""), string_body, tag("\""))(input)?;

    match unescape(raw_string) {
        Ok(s) => Ok((remain, s)),
        Err(_) => Err(nom::Err::Failure(DbcParseError::BadEscape)),
    }
}

fn dbc_version(input: &str) -> IResult<&str, DbcVersion, DbcParseError> {
    map(preceded(spacey(tag("VERSION")), string_literal), |s| {
        DbcVersion(s)
    })(input)
}

fn parse_uppercase_underscore(input: &str) -> IResult<&str, &str, DbcParseError> {
    take_while1(|c: char| c.is_ascii_uppercase() || c == '_')(input)
}

fn dbc_one_name(input: &str) -> IResult<&str, String, DbcParseError> {
    map(
        tuple((space0, parse_uppercase_underscore, space0, line_ending)),
        |(_, name, _, _)| name.to_owned(),
    )(input)
}

fn dbc_names(input: &str) -> IResult<&str, DbcNames, DbcParseError> {
    map(
        tuple((
            multispacey(tag("NS_:")),
            many0(dbc_one_name),
            many0(line_ending),
        )),
        |(_, names, _)| DbcNames(names),
    )(input)
}

fn dbc_value(input: &str) -> IResult<&str, OneDbc, DbcParseError> {
    map(
        multispacey(tuple((multispacey(dbc_version), multispacey(dbc_names)))),
        |(version, names)| OneDbc {
            version,
            names,
            bus_configuratin: None,
        },
    )(input)
}

pub fn parse_dbc(input: &str) -> Result<OneDbc, DbcParseError> {
    let (_, result) = all_consuming(dbc_value)(input).map_err(|nom_err| match nom_err {
        nom::Err::Incomplete(_) => unreachable!(),
        nom::Err::Error(e) => e,
        nom::Err::Failure(e) => e,
    })?;
    Ok(result)
}

#[test]
fn test_dbc_version() {
    assert_eq!(
        dbc_version("VERSION \"1.0.0\""),
        Ok(("", DbcVersion("1.0.0".into())))
    );

    assert_eq!(
        dbc_version("VERSION  \"3.0.1\""),
        Ok(("", DbcVersion("3.0.1".into())))
    );
}

#[test]
fn test_dbc_one_name() {
    assert_eq!(
        dbc_one_name(
            r#"  BS_
"#
        ),
        Ok(("", "BS_".into())),
    );

    assert_eq!(
        dbc_one_name(
            r#"    CM_
"#
        ),
        Ok(("", "CM_".into())),
    );
}

#[test]
fn test_dbc_names() {
    assert_eq!(
        dbc_names(
            r#"NS_:
    BS_
    CM_


"#
        ),
        Ok(("", DbcNames(vec!["BS_".into(), "CM_".into()]))),
    );
}

#[test]
fn test_dbc_01() {
    assert_eq!(
        parse_dbc(
            r#"VERSION "1.0"


NS_:
    BS_
    CM_


"#
        ),
        Ok(OneDbc {
            version: DbcVersion("1.0".into()),
            names: DbcNames(vec!["BS_".into(), "CM_".into()]),
            bus_configuratin: None,
        }),
    );
}
