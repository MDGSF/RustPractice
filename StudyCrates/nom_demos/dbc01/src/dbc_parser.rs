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
    #[error("bad integer")]
    BadInt,
    #[error("bad float")]
    BadFloat,
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

/// List of all CAN-Nodes, seperated by whitespaces.
/// BU_: ABS DRS_MM5_10
#[derive(PartialEq, Debug, Clone)]
pub struct DbcCanNodes(Vec<String>);

/*
 SG_ S7 m1 : 40|24@1- (1,0) [0|0] "" Vector__XXX
 SG_ S8 m2 : 40|8@1- (1,0) [0|0] "" Vector__XXX
 SG_ S6 M : 32|8@1- (1,0) [0|0] "" Vector__XXX
 SG_ S3 m0 : 16|16@1- (1,0) [0|0] "" Vector__XXX
 SG_ S2 m0 : 8|8@1- (1,0) [0|0] "" Vector__XXX
*/
#[derive(PartialEq, Debug, Clone)]
pub enum DbcSignalMultiplexer {
    M,
    MultiplexerIdentifier(i64),
}

#[derive(PartialEq, Debug, Clone)]
pub enum DbcSignalEndianness {
    LittleEndian,
    BigEndian,
}

#[derive(PartialEq, Debug, Clone)]
pub enum DbcSignalSigned {
    Signed,
    Unsigned,
}

/// Signal definition.
/// Format: `SG_ <SignalName> [M|m<MultiplexerIdentifier>] : <StartBit>|<Length>@<Endianness><Signed> (<Factor>,<Offset>) [<Min>|<Max>] "[Unit]" [ReceivingNodes]`
/// Length in bits.
/// Signed: + = unsigned; - = signed
/// Endianness: 1 = little-endian, Intel; 0 = big-endian, Motorola
/// M: If M than this signals contains a multiplexer identifier.
/// MultiplexerIdentifier: Signal definition is only used if the value of the multiplexer signal equals to this value.
#[derive(PartialEq, Debug, Clone)]
pub struct DbcSignal {
    pub name: String,
    pub multiplexer: Option<DbcSignalMultiplexer>,
    pub start_bit: i64,
    pub length: i64,
    pub endianness: DbcSignalEndianness,
    pub signed: DbcSignalSigned,
    pub factor: f64,
    pub offset: f64,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub unit: Option<String>,
    pub receiving_node: Option<String>,
}

/// Message definition.
/// Format: `BO_ <CAN-ID> <MessageName>: <MessageLength> <SendingNode>`
/// MessageLength in bytes.
#[derive(PartialEq, Debug, Clone)]
pub struct DbcMessageHeader {
    pub can_id: i64,
    pub name: String,
    pub length: i64,
    pub sending_node: String,
}
#[derive(PartialEq, Debug, Clone)]
pub struct DbcMessage {
    pub header: DbcMessageHeader,
    pub signals: Vec<DbcSignal>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct OneDbc {
    pub version: DbcVersion,
    pub names: DbcNames,
    pub bus_configuration: Option<DbcBusConfiguration>,
    pub can_nodes: DbcCanNodes,
    pub messages: Vec<DbcMessage>,
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

fn digit1to9(input: &str) -> IResult<&str, char, DbcParseError> {
    one_of("123456789")(input)
}
fn uint(input: &str) -> IResult<&str, &str, DbcParseError> {
    alt((tag("0"), recognize(pair(digit1to9, digit0))))(input)
}

fn integer_body(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(pair(opt(tag("-")), uint))(input)
}

fn integer_value(input: &str) -> IResult<&str, i64, DbcParseError> {
    let (remain, raw_int) = integer_body(input)?;
    match raw_int.parse::<i64>() {
        Ok(i) => Ok((remain, i)),
        Err(_) => Err(nom::Err::Failure(DbcParseError::BadInt)),
    }
}

fn frac(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(pair(tag("."), digit1))(input)
}

fn exp(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(tuple((tag("e"), opt(alt((tag("-"), tag("+")))), digit1)))(input)
}

fn float_body(input: &str) -> IResult<&str, &str, DbcParseError> {
    recognize(tuple((
        opt(tag("-")),
        uint,
        alt((recognize(pair(frac, opt(exp))), exp)),
    )))(input)
}

fn float_value(input: &str) -> IResult<&str, f64, DbcParseError> {
    let (remain, raw_float) = float_body(input)?;
    match raw_float.parse::<f64>() {
        Ok(f) => Ok((remain, f)),
        Err(_) => Err(nom::Err::Failure(DbcParseError::BadFloat)),
    }
}

fn number_value(input: &str) -> IResult<&str, f64, DbcParseError> {
    alt((
        map(float_value, |f| f.into()),
        map(integer_value, |i| i as f64),
    ))(input)
}

fn dbc_object_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(input)
}

fn dbc_node_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_object_name(input)
}

fn dbc_signal_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_object_name(input)
}

fn dbc_message_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_object_name(input)
}

fn dbc_version(input: &str) -> IResult<&str, DbcVersion, DbcParseError> {
    map(preceded(spacey(tag("VERSION")), string_literal), |s| {
        DbcVersion(s)
    })(input)
}

fn dbc_one_line_name(input: &str) -> IResult<&str, String, DbcParseError> {
    map(
        tuple((space0, dbc_object_name, space0, line_ending)),
        |(_, name, _, _)| name.to_owned(),
    )(input)
}

fn dbc_names(input: &str) -> IResult<&str, DbcNames, DbcParseError> {
    map(
        tuple((
            multispacey(tag("NS_:")),
            many0(dbc_one_line_name),
            many0(line_ending),
        )),
        |(_, names, _)| DbcNames(names),
    )(input)
}

fn dbc_bus_configuration(input: &str) -> IResult<&str, Option<DbcBusConfiguration>, DbcParseError> {
    map(
        tuple((
            multispacey(tag("BS_")),
            spacey(tag(":")),
            opt(float_value),
            many0(line_ending),
        )),
        |(_, _, speed, _)| match speed {
            None => None,
            Some(speed) => Some(DbcBusConfiguration(speed)),
        },
    )(input)
}

fn dbc_can_nodes(input: &str) -> IResult<&str, DbcCanNodes, DbcParseError> {
    map(
        tuple((
            multispacey(tag("BU_")),
            spacey(tag(":")),
            many0(spacey(dbc_node_name)),
            many0(line_ending),
        )),
        |(_, _, names, _)| DbcCanNodes(names.into_iter().map(String::from).collect()),
    )(input)
}

fn dbc_signal_multiplexer(input: &str) -> IResult<&str, DbcSignalMultiplexer, DbcParseError> {
    alt((
        map(tag("M"), |_| DbcSignalMultiplexer::M),
        map(tuple((tag("m"), integer_value)), |(_, num)| {
            DbcSignalMultiplexer::MultiplexerIdentifier(num)
        }),
    ))(input)
}

fn dbc_signal_endianness(input: &str) -> IResult<&str, DbcSignalEndianness, DbcParseError> {
    alt((
        map(tag("1"), |_| DbcSignalEndianness::LittleEndian),
        map(tag("0"), |_| DbcSignalEndianness::BigEndian),
    ))(input)
}

fn dbc_signal_signed(input: &str) -> IResult<&str, DbcSignalSigned, DbcParseError> {
    alt((
        map(tag("+"), |_| DbcSignalSigned::Unsigned),
        map(tag("-"), |_| DbcSignalSigned::Signed),
    ))(input)
}

fn dbc_signal_factor_offset(input: &str) -> IResult<&str, (f64, f64), DbcParseError> {
    let (remaining, (factor, offset)) = delimited(
        spacey(tag("(")),
        separated_pair(number_value, spacey(tag(",")), number_value),
        spacey(tag(")")),
    )(input)?;

    Ok((remaining, (factor, offset)))
}

fn dbc_signal_min_max(input: &str) -> IResult<&str, (f64, f64), DbcParseError> {
    let (remaining, (min_value, max_value)) = delimited(
        spacey(tag("[")),
        separated_pair(number_value, spacey(tag("|")), number_value),
        spacey(tag("]")),
    )(input)?;

    Ok((remaining, (min_value, max_value)))
}

fn dbc_signal(input: &str) -> IResult<&str, DbcSignal, DbcParseError> {
    map(
        tuple((
            multispacey(tag("SG_")),
            spacey(dbc_signal_name),
            spacey(opt(dbc_signal_multiplexer)),
            spacey(tag(":")),
            spacey(integer_value), // start bit
            tag("|"),
            integer_value, // length
            tag("@"),
            dbc_signal_endianness,
            spacey(dbc_signal_signed),
            spacey(dbc_signal_factor_offset),
            spacey(opt(dbc_signal_min_max)),
            spacey(opt(string_literal)), // "[Unit]"
            spacey(opt(dbc_node_name)),
            many0(line_ending),
        )),
        |(
            _,
            name,
            multiplexer,
            _,
            start_bit,
            _,
            length,
            _,
            endianness,
            signed,
            factor_offset,
            min_max,
            unit,
            receiving_node,
            _,
        )| DbcSignal {
            name: String::from(name),
            multiplexer,
            start_bit,
            length,
            endianness,
            signed,
            factor: factor_offset.0,
            offset: factor_offset.1,
            min: min_max.map(|(min, _)| min),
            max: min_max.map(|(_, max)| max),
            unit,
            receiving_node: receiving_node.map(String::from),
        },
    )(input)
}

fn dbc_message_header(input: &str) -> IResult<&str, DbcMessageHeader, DbcParseError> {
    map(
        tuple((
            multispacey(tag("BO_")),
            spacey(integer_value), // can id
            spacey(dbc_message_name),
            spacey(tag(":")),
            spacey(integer_value), // length
            spacey(dbc_node_name),
        )),
        |(_, can_id, message_name, _, length, sending_node_name)| DbcMessageHeader {
            can_id,
            name: String::from(message_name),
            length,
            sending_node: String::from(sending_node_name),
        },
    )(input)
}

fn dbc_message(input: &str) -> IResult<&str, DbcMessage, DbcParseError> {
    map(
        tuple((dbc_message_header, many0(dbc_signal), many0(line_ending))),
        |(header, signals, _)| DbcMessage { header, signals },
    )(input)
}

fn dbc_value(input: &str) -> IResult<&str, OneDbc, DbcParseError> {
    map(
        multispacey(tuple((
            multispacey(dbc_version),
            multispacey(dbc_names),
            multispacey(dbc_bus_configuration),
            multispacey(dbc_can_nodes),
            multispacey(many0(dbc_message)),
        ))),
        |(version, names, bus_configuration, can_nodes, messages)| OneDbc {
            version,
            names,
            bus_configuration,
            can_nodes,
            messages,
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
fn test_dbc_one_line_name() {
    assert_eq!(
        dbc_one_line_name(
            r#"  BS_
"#
        ),
        Ok(("", "BS_".into())),
    );

    assert_eq!(
        dbc_one_line_name(
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
fn test_dbc_bus_configuration() {
    assert_eq!(
        dbc_bus_configuration(
            r#"BS_: 12.34

"#
        ),
        Ok(("", Some(DbcBusConfiguration(12.34)))),
    );

    assert_eq!(
        dbc_bus_configuration(
            r#"BS_:

"#
        ),
        Ok(("", None)),
    );
}

#[test]
fn test_dbc_can_nodes() {
    assert_eq!(
        dbc_can_nodes(
            r#"BU_: ABS DRS_MM5_10

"#
        ),
        Ok(("", DbcCanNodes(vec!["ABS".into(), "DRS_MM5_10".into()]))),
    );

    assert_eq!(
        dbc_can_nodes(r#"BU_:Matrix"#),
        Ok(("", DbcCanNodes(vec!["Matrix".into()]))),
    );

    assert_eq!(
        dbc_can_nodes(r#"BU_: Node2 Node1 Node0"#),
        Ok((
            "",
            DbcCanNodes(vec!["Node2".into(), "Node1".into(), "Node0".into()])
        )),
    );
}

#[test]
fn test_dbc_signal_multiplexer_01() {
    assert_eq!(
        dbc_signal_multiplexer(r#"M"#),
        Ok(("", DbcSignalMultiplexer::M)),
    );
}

#[test]
fn test_dbc_signal_multiplexer_02() {
    assert_eq!(
        dbc_signal_multiplexer(r#"m0"#),
        Ok(("", DbcSignalMultiplexer::MultiplexerIdentifier(0))),
    );
}

#[test]
fn test_dbc_signal_multiplexer_03() {
    assert_eq!(
        dbc_signal_multiplexer(r#"m123"#),
        Ok(("", DbcSignalMultiplexer::MultiplexerIdentifier(123))),
    );
}

#[test]
fn test_dbc_signal_01() {
    assert_eq!(
        dbc_signal(
            r#" SG_ AY1 : 32|16@1+ (0.000127465,-4.1768) [-4.1768|4.1765] "g"  ABS

"#
        ),
        Ok((
            "",
            DbcSignal {
                name: "AY1".into(),
                multiplexer: None,
                start_bit: 32,
                length: 16,
                endianness: DbcSignalEndianness::LittleEndian,
                signed: DbcSignalSigned::Unsigned,
                factor: 0.000127465,
                offset: -4.1768,
                min: Some(-4.1768),
                max: Some(4.1765),
                unit: Some("g".into()),
                receiving_node: Some("ABS".into()),
            }
        )),
    );
}

#[test]
fn test_dbc_signal_02() {
    assert_eq!(
        dbc_signal(
            r#" SG_ S2 m0 : 8|8@1- (1.0,0.0) [0.0|0.0] "" Vector__XXX

"#
        ),
        Ok((
            "",
            DbcSignal {
                name: "S2".into(),
                multiplexer: Some(DbcSignalMultiplexer::MultiplexerIdentifier(0)),
                start_bit: 8,
                length: 8,
                endianness: DbcSignalEndianness::LittleEndian,
                signed: DbcSignalSigned::Signed,
                factor: 1.0,
                offset: 0.0,
                min: Some(0.0),
                max: Some(0.0),
                unit: Some("".into()),
                receiving_node: Some("Vector__XXX".into()),
            }
        )),
    );
}

#[test]
fn test_dbc_signal_03() {
    assert_eq!(
        dbc_signal(
            r#" SG_ S2 m0 : 8|8@1- (1,0) [0|0] "" Vector__XXX

"#
        ),
        Ok((
            "",
            DbcSignal {
                name: "S2".into(),
                multiplexer: Some(DbcSignalMultiplexer::MultiplexerIdentifier(0)),
                start_bit: 8,
                length: 8,
                endianness: DbcSignalEndianness::LittleEndian,
                signed: DbcSignalSigned::Signed,
                factor: 1.0,
                offset: 0.0,
                min: Some(0.0),
                max: Some(0.0),
                unit: Some("".into()),
                receiving_node: Some("Vector__XXX".into()),
            }
        )),
    );
}

#[test]
fn test_dbc_message_header() {
    assert_eq!(
        dbc_message_header(r#"BO_ 2348941054 Normal: 8 Vector__XXX"#),
        Ok((
            "",
            DbcMessageHeader {
                can_id: 2348941054,
                name: "Normal".into(),
                length: 8,
                sending_node: "Vector__XXX".into(),
            }
        )),
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

BS_:
BU_: ABS DRS_MM5_10

BO_ 117 DRS_RX_ID0: 8 ABS

BO_ 112 MM5_10_TX1: 8 DRS_MM5_10
 SG_ Yaw_Rate : 0|16@1+ (0.005,-163.84) [-163.84|163.83] "°/s"  ABS
 SG_ AY1 : 32|16@1+ (0.000127465,-4.1768) [-4.1768|4.1765] "g"  ABS

"#
        ),
        Ok(OneDbc {
            version: DbcVersion("1.0".into()),
            names: DbcNames(vec!["BS_".into(), "CM_".into()]),
            bus_configuration: None,
            can_nodes: DbcCanNodes(vec!["ABS".into(), "DRS_MM5_10".into()]),
            messages: vec![
                DbcMessage {
                    header: DbcMessageHeader {
                        can_id: 117,
                        name: "DRS_RX_ID0".into(),
                        length: 8,
                        sending_node: "ABS".into(),
                    },
                    signals: vec![],
                },
                DbcMessage {
                    header: DbcMessageHeader {
                        can_id: 112,
                        name: "MM5_10_TX1".into(),
                        length: 8,
                        sending_node: "DRS_MM5_10".into(),
                    },
                    signals: vec![
                        DbcSignal {
                            name: "Yaw_Rate".into(),
                            multiplexer: None,
                            start_bit: 0,
                            length: 16,
                            endianness: DbcSignalEndianness::LittleEndian,
                            signed: DbcSignalSigned::Unsigned,
                            factor: 0.005,
                            offset: -163.84,
                            min: Some(-163.84),
                            max: Some(163.83),
                            unit: Some("°/s".into()),
                            receiving_node: Some("ABS".into()),
                        },
                        DbcSignal {
                            name: "AY1".into(),
                            multiplexer: None,
                            start_bit: 32,
                            length: 16,
                            endianness: DbcSignalEndianness::LittleEndian,
                            signed: DbcSignalSigned::Unsigned,
                            factor: 0.000127465,
                            offset: -4.1768,
                            min: Some(-4.1768),
                            max: Some(4.1765),
                            unit: Some("g".into()),
                            receiving_node: Some("ABS".into()),
                        }
                    ],
                },
            ],
        }),
    );
}
