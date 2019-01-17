extern crate rmp_serde as rmps;
extern crate serde;

use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
    test1();
}

fn test1() {
    let mut buf = Vec::new();
    let val = (42u8, "the Answer");
    val.serialize(&mut Serializer::new(&mut buf)).unwrap();

    let mut de = Deserializer::new(&buf[..]);
    println!("{}", Deserialize::deserialize(&mut de).unwrap());
}
