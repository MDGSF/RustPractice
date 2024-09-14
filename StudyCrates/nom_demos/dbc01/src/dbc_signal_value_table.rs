/// Value table definition for signals.
/// Format: `VAL_TABLE_ <ValueTableName> <ValueTableDefinition>;`
/// ValueTableDefinition: List of `IntValue "StringValue"` Pairs, seperated by whitespaces
pub struct DbcSignalValueTable {
    pub name: String,
    pub values: Vec<(i64, String)>,
}

pub struct DbcSignalValueTables(Vec<DbcSignalValueTable>);
