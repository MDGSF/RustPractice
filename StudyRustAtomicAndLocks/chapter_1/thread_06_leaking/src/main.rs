use std::thread;

/*
Another way to share ownership is by leaking an allocation. Using Box::leak, one can release ownership of a Box, promising to never drop it. From that point on, the Box will live forever, without an owner, allowing it to be borrowed by any thread for as long as the program runs.

The downside of leaking a Box is that we’re leaking memory. We allocate something, but never drop and deallocate it. This can be fine if it happens only a limited number of times. But if we keep doing this, the program will slowly run out of memory.
*/
fn main() {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    let t1 = thread::spawn(move || dbg!(x));
    let t2 = thread::spawn(move || dbg!(x));
    t1.join().unwrap();
    t2.join().unwrap();
}
