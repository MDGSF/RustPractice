extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use rmps::encode::{Ext, UnderlyingWrite};
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Serialize)]
struct Dog {
    name: String,
    age: u16,
    sub: SubDog,
}

#[derive(Debug, Deserialize, Serialize)]
struct SubDog {
    id: u32,
}

#[derive(Debug, Deserialize)]
struct Dog2 {
    #[serde(default)]
    id: u32,

    #[serde(default, rename = "age")]
    my_age: u16,

    #[serde(default)]
    name: String,

    #[serde(default)]
    addr: String,

    sub: SubDog,
}

fn main() {
    let dog = Dog {
        name: "Bobby".to_string(),
        age: 8,
        sub: SubDog { id: 111 },
    };

    let mut buf = Vec::new();
    let mut se = Serializer::new(&mut buf).with_struct_map();
    dog.serialize(&mut se).unwrap();

    println!("se = {:?}", se.into_inner());
    println!("buf = {:?}", buf);

    let cur = Cursor::new(&buf[..]);
    let mut de = Deserializer::new(cur);
    let out: Dog2 = Deserialize::deserialize(&mut de).unwrap();
    println!("out = {:?}", out);
}
