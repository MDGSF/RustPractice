use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let (tx, rx) = channel();
    for i in 0..5 {
        let tx = tx.clone();
        thread::spawn(move || {
            tx.send(i).unwrap();
        });
    }
    // drop(tx); // drop 解决死锁
    for j in rx.iter() {
        println!("{:?}", j);
    }
}
