use super::dbc_common_parsers::*;
use super::dbc_error::DbcParseError;
use super::dbc_signal::dbc_signal;
use super::dbc_signal::DbcSignal;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

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

impl fmt::Display for DbcMessageHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BO_ {} {}: {} {}",
            self.can_id, self.name, self.length, self.sending_node
        )
    }
}

impl fmt::Display for DbcMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.header)?;
        for signal in &self.signals {
            writeln!(f, "\t{}", signal)?;
        }
        Ok(())
    }
}

fn dbc_message_name(input: &str) -> IResult<&str, &str, DbcParseError> {
    dbc_object_name(input)
}

fn dbc_message_header(input: &str) -> IResult<&str, DbcMessageHeader, DbcParseError> {
    let res = map(
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
    )(input);

    match res {
        Ok((remain, header)) => {
            log::info!("parse message header: {:?}", header);
            Ok((remain, header))
        }
        Err(e) => {
            log::trace!("parse message header failed, e = {:?}", e);
            Err(nom::Err::Error(DbcParseError::BadMessageHeader))
        }
    }
}

pub fn dbc_message(input: &str) -> IResult<&str, DbcMessage, DbcParseError> {
    map(
        tuple((dbc_message_header, many0(dbc_signal), many0(line_ending))),
        |(header, signals, _)| DbcMessage { header, signals },
    )(input)
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
