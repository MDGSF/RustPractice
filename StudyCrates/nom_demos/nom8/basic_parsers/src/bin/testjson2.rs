use basic_parsers::json2::json;
use nom::Complete;
use nom::Emit;
use nom::OutputM;
use nom::Parser;

fn main() {
    let data = r#"{"a":1,"b":2,"c":3}"#;

    let res = json()
        .process::<OutputM<Emit, Emit, Complete>>(data)
        .unwrap();

    println!("{:?}", res);
}
