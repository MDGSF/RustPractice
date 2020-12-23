use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

struct Node {
    next: Option<Rc<RefCell<Node>>>,
    head: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let first = Rc::new(RefCell::new(Node {
        next: None,
        head: None,
    }));
    let second = Rc::new(RefCell::new(Node {
        next: None,
        head: None,
    }));
    let third = Rc::new(RefCell::new(Node {
        next: None,
        head: None,
    }));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(third.clone());
    third.borrow_mut().head = Some(Rc::downgrade(&first));
}

/*
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

struct Node {
    next: Option<Rc<RefCell>>,
    head: Option<Weak<RefCell>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}

fn main() {
    let first = Rc::new(RefCell::new(Node {
        next: None,
        head: None,
    }));
    let second = Rc::new(RefCell::new(Node {
        next: None,
        head: None,
    }));
    let third = Rc::new(RefCell::new(Node {
        next: None,
        head: None,
    }));
    first.borrow_mut().next = Some(second.clone());
    second.borrow_mut().next = Some(third.clone());
    third.borrow_mut().head = Some(Rc::downgrade(&first));
}
*/
