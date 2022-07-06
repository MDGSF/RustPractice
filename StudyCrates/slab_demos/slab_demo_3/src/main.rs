extern crate slab;

use slab::Slab;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let slab = Arc::new(Mutex::new(Slab::new()));

    let hello = slab.lock().unwrap().insert("hello");
    let world = slab.lock().unwrap().insert("world");

    let slab_thread = Arc::clone(&slab);
    let world_thread = world;
    let thread_join_handle = thread::spawn(move || {
        println!("{:?}", slab_thread.lock().unwrap()[world_thread]);
    });
    println!("{:?}", slab.lock().unwrap()[hello]);

    let _res = thread_join_handle.join();
}
