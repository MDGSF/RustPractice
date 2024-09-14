use std::fmt;

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
pub struct DbcNames(pub Vec<String>);

impl fmt::Display for DbcNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "NS_:")?;
        for name in &self.0 {
            writeln!(f, "\t{name}")?;
        }
        Ok(())
    }
}
