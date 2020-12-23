use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();
    thread::spawn(move || {
        tx.send(1u8).unwrap();
        tx.send(2u8).unwrap();
        tx.send(3u8).unwrap();
    });
    for x in rx.iter() {
        println!("receive: {}", x);
    }
}
