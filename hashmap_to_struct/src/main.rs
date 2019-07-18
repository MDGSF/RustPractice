use std::collections::HashMap;

pub use fromhashmap_derive::*;

pub trait FromHashMap<T>: Default {
    fn from_hashmap(hm: HashMap<String, String>) -> T;
}

#[derive(Debug, Default, FromHashMap)]
struct Student {
    name: String,
    age: i32,
}

fn main() {
    println!("Hello, world!");

    let mut hm = HashMap::new();
    hm.insert(String::from("name"), String::from("huangjian"));
    hm.insert(String::from("age"), String::from("10000"));
    let settings = Student::from_hashmap(hm);
    println!("settings = {:?}", settings);
    //assert_eq!(settings.name, "huangjian");
    //assert_eq!(settings.age, 10000);
}
