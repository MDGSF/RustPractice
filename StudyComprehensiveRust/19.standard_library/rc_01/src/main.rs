use std::rc::Rc;

fn main() {
    let a = Rc::new(0);
    let b = Rc::clone(&a);
    println!("a: {a}");
    println!("b: {b}");
}
