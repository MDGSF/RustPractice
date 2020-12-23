use std::collections::HashMap;

pub use fromhashmap_derive_2::*;
pub use value::*;
pub use variableValue::*;

#[derive(Debug, Default, FromValue, ToValue)]
struct Student {
    name: String,
    age: i32,
    sub: Sub,
    sub2: Sub2,
}

#[derive(Debug, Default, FromValue, ToValue)]
struct Sub {
    a: i32,
    b: bool,
}

#[derive(Debug, Default, FromValue, ToValue)]
struct Sub2 {
    sub2_a: i32,
    sub2_b: bool,
    sub2_c: String,
    sub2_d: bool,
    ss21: SubSub2_1,
}

#[derive(Debug, Default, FromValue, ToValue)]
struct SubSub2_1 {
    SubSub1_a: i32,
    SubSub1_b: bool,
}

fn main() {
    println!("Hello, world!");

    let s1 = Student {
        name: "huangjian".to_string(),
        age: 10000,
        sub: Sub {
            a: 100,
            ..Default::default()
        },
        ..Default::default()
    };
    println!("s1 = {:?}", s1);

    let v = s1.toValue();
    println!("v = {}", v);

    let s2: Student = v.fromValue();
    println!("s2 = {:?}", s2);
}
