use super::dbc_signal::DbcSignal;
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
