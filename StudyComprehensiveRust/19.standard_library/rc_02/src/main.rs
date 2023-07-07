use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i64,
    parent: Option<Weak<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let root = Rc::new(RefCell::new(Node {
        value: 42,
        parent: None,
        children: vec![],
    }));
    let child = Rc::new(RefCell::new(Node {
        value: 43,
        parent: Some(Rc::downgrade(&root)),
        children: vec![],
    }));
    root.borrow_mut().children.push(child);
    println!("graph: {root:?}");
}
