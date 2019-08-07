use std::rc::Rc;
use std::sync::Arc;

fn test1() {
    let x = Box::new("hello");
    let y = x; // 所有权移动

    //println!("{:?}", x);
}

fn test2() {
    let a = Box::new("hello");
    let b = Box::new("Rust".to_string());
    let c = *a;
    let d = *b; // 所有权移动

    println!("{:?}", a);
    //println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
}

fn test3() {
    let a = Rc::new("Rust".to_string());
    let b = Arc::new(vec![1.0, 2.0, 3.0]);
    let c = Rc::new(123);

    // let x = *a; // Rc 和 Arc 内部的东西的所有权无法移动到外面。
    // let y = *b;
    let z = *c; // 123 是值类型，使用 Copy trait
}

fn main() {
    test2();
}
