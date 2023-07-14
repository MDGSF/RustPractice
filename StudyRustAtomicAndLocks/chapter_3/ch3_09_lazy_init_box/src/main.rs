use std::sync::atomic::AtomicPtr;
use std::sync::atomic::Ordering::{Acquire, Release};

/*
We load a potentially non-null (i.e., initialized) pointer in two places: through the load operation and through the compare_exchange operation when it fails. So, as explained above, we need to use Acquire for both the load memory ordering and the compare_exchange failure memory ordering, to be able to synchronize with the operation that stores the pointer. This store happens when the compare_exchange operation succeeds, so we must use Release as its success ordering. 
*/
fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // Safety: p is not null and points to a properly initialized value.
    unsafe { &*p }
}

struct Data([u8; 100]);

fn generate_data() -> Data {
    Data([123; 100])
}

fn main() {
    println!("{:p}", get_data());
    println!("{:p}", get_data());
}
