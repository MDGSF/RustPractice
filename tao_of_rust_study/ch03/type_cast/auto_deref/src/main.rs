use std::rc::Rc;

fn foo(s: &[i32]) {
    println!("{:?}", s[0]);
}

fn main() {
    let v = vec![1, 2, 3];
    foo(&v);

    let a = "hello".to_string();
    let b = " world".to_string();
    let c = a + &b;
    println!("{:?}", c);

    let x = Rc::new("hello");
    println!("{:?}", x.chars());
}
