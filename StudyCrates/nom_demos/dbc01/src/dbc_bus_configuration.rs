use std::fmt;

/// Bus configuration.
/// Format:: `BS_: <Speed>`
/// Speed in kBit/s
#[derive(PartialEq, Debug, Clone)]
pub struct DbcBusConfiguration(pub Option<f64>);

impl fmt::Display for DbcBusConfiguration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(speed) => write!(f, "BS_: {}", speed),
            None => write!(f, "BS_:"),
        }
    }
}
