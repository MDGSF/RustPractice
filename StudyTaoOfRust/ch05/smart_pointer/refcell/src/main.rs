use std::cell::RefCell;

fn main() {
    let x = RefCell::new(vec![1, 2, 3, 4]);
    println!("{:?}", x.borrow());
    x.borrow_mut().push(5);
    println!("{:?}", x.borrow());
}
