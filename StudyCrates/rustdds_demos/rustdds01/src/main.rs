use anyhow::Result;
use no_key::{DeserializerAdapter, SerializerAdapter};
use rustdds::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct SomeType {
    a: i32,
    b: String,
    c: f64,
}

fn main() -> Result<()> {
    let obj1 = SomeType {
        a: 1,
        b: "hello".to_string(),
        c: 1.23,
    };

    println!("obj1: {:?}", obj1);

    let data =
        CDRSerializerAdapter::<SomeType, rustdds::serialization::LittleEndian>::to_bytes(&obj1)?;

    println!("data: {:?}", data);

    let obj2 = CDRDeserializerAdapter::<SomeType>::from_bytes(
        &data,
        rustdds::serialization::representation_identifier::RepresentationIdentifier::CDR_LE,
    )?;

    println!("obj2: {:?}", obj2);

    Ok(())
}
