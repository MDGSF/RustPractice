use std::fmt;

/// List of all CAN-Nodes, seperated by whitespaces.
/// BU_: ABS DRS_MM5_10
#[derive(PartialEq, Debug, Clone)]
pub struct DbcCanNodes(pub Vec<String>);

impl fmt::Display for DbcCanNodes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BU_:",)?;
        for node in &self.0 {
            write!(f, " {node}")?;
        }
        Ok(())
    }
}
