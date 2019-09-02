use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn main() {
    println!("Hello, world!");
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = spinlock.clone();
    let thread = thread::spawn(move || spinlock_clone.store(0, Ordering::SeqCst));
    let mut i = 0;
    while spinlock.load(Ordering::SeqCst) != 0 {
        println!("while i = {}", i);
        i += 1;
    }
    if let Err(panic) = thread.join() {
        println!("Thread had an error: {:?}", panic);
    }
}
