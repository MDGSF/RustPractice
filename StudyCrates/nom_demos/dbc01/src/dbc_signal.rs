use std::fmt;

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

impl fmt::Display for DbcSignalMultiplexer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbcSignalMultiplexer::M => write!(f, "M"),
            DbcSignalMultiplexer::MultiplexerIdentifier(id) => write!(f, "m{id}"),
        }
    }
}

/// Endianness: 1 = little-endian, Intel; 0 = big-endian, Motorola
#[derive(PartialEq, Debug, Clone)]
pub enum DbcSignalEndianness {
    LittleEndian,
    BigEndian,
}

impl fmt::Display for DbcSignalEndianness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbcSignalEndianness::LittleEndian => write!(f, "1"),
            DbcSignalEndianness::BigEndian => write!(f, "0"),
        }
    }
}

/// Signed: + = unsigned; - = signed
#[derive(PartialEq, Debug, Clone)]
pub enum DbcSignalSigned {
    Signed,
    Unsigned,
}

impl fmt::Display for DbcSignalSigned {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbcSignalSigned::Signed => write!(f, "-"),
            DbcSignalSigned::Unsigned => write!(f, "+"),
        }
    }
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
    pub receiving_nodes: Option<Vec<String>>,
}

impl fmt::Display for DbcSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let multiplexer = match &self.multiplexer {
            Some(m) => format!("{m} "),
            None => "".to_string(),
        };
        let signed = match &self.signed {
            DbcSignalSigned::Signed => "-",
            DbcSignalSigned::Unsigned => "+",
        };
        let endianness = &self.endianness.to_string();
        let min_max = match (&self.min, &self.max) {
            (Some(min), Some(max)) => format!("[{min}|{max}]"),
            _ => "".to_string(),
        };
        let unit = match &self.unit {
            Some(u) => format!("\"{u}\""),
            None => "".to_string(),
        };
        let mut receiving_nodes_str = String::new();
        if let Some(nodes) = &self.receiving_nodes {
            receiving_nodes_str = nodes.join(",");
        };

        write!(
            f,
            "SG_ {} {}: {}|{}@{}{} ({},{}) {} {} {}",
            self.name,
            multiplexer,
            self.start_bit,
            self.length,
            endianness,
            signed,
            self.factor,
            self.offset,
            min_max,
            unit,
            receiving_nodes_str
        )
    }
}
