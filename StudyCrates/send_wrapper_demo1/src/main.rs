use send_wrapper::SendWrapper;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    println!("Hello, world!");

    let value = Rc::new(42);
    let wrapped_value = SendWrapper::new(value);

    let (sender, receiver) = channel();

    thread::spawn(move || {
        sender.send(wrapped_value).unwrap();
    });

    let wrapped_value = receiver.recv().unwrap();

    let value = wrapped_value.deref();

    println!("value = {}", value);
}
