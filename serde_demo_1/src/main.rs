#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Sub1 {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point1 {
    x: i32,
    y: i32,
    sub: Sub1,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point2 {
    #[serde(rename = "x")]
    #[serde(default)]
    iix: i32,

    #[serde(default)]
    y: i32,

    #[serde(default)]
    z: i32,

    #[serde(rename = "sub")]
    xx: Sub1,
}

fn main() {
    let point = Point1 {
        x: 1,
        y: 2,
        sub: Sub1 {
            name: "huangjian".to_string(),
        },
    };

    let serialized = serde_json::to_string(&point).unwrap();

    println!("serialized = {}", serialized);

    let deserialized: Point2 = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);

    let data = serde_json::to_string(&deserialized).unwrap();

    println!("data = {}", data);
}
