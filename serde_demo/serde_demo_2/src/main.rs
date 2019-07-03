#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Point1 {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point2 {
    #[serde(default)]
    z: i32,
    y: i32,
    x: i32,
}

fn main() {
    let point = Point1 { x: 1, y: 2 };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("point1 = {}", serialized);

    let deserialized: Point2 = serde_json::from_str(&serialized).unwrap();
    println!("point2 = {:?}", deserialized);
}
