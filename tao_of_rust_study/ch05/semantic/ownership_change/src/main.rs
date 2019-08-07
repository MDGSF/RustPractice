#[derive(Debug, Copy, Clone)]
struct A {
    a: i32,
    b: u32,
}

fn test1() {
    let a = A { a: 1, b: 2 };
    let b = a;
    println!("{:?}", a);
}

fn main() {
    let a = ("a".to_string(), "b".to_string());
    let b = a;
    // println!("{:?}", a);
    let c = (1, 2, 3);
    let d = c;
    println!("{:?}", c);
}
