use anyhow::Result;
use dbc01::dbc_parser::*;

fn main() -> Result<()> {
    let data = std::fs::read_to_string("dbc/mytest/a.dbc")?;
    let (remain, onedbc) = dbc_value(&data)?;
    println!("remain: {}", remain);
    println!("onedbc: {}", onedbc);
    Ok(())
}
