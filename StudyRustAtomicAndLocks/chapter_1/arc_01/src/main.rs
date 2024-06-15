use std::sync::Arc;
use std::thread;

/*
As it turns out, Rc is not thread safe. If multiple threads had an Rc to the same allocation, they might try to modify the reference counter at the same time, which can give unpredictable results.

Instead, we can use std::sync::Arc, which stands for "atomically reference counted." It’s identical to Rc, except it guarantees that modifications to the reference counter are indivisible atomic operations, making it safe to use it with multiple threads.

Because ownership is shared, reference counting pointers (Rc<T> and Arc<T>) have the same restrictions as shared references (&T). They do not give you mutable access to their contained value, since the value might be borrowed by other code at the same time.
*/
fn main() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    let t1 = thread::spawn(move || dbg!(a));
    let t2 = thread::spawn(move || dbg!(b));
    t1.join().unwrap();
    t2.join().unwrap();
}
