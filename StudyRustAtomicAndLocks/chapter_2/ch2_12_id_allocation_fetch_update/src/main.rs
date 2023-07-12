use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID.fetch_update(Relaxed, Relaxed, 
                         |n| n.checked_add(1)).expect("too many IDs!")
}

fn main() {
    dbg!(allocate_new_id());
    dbg!(allocate_new_id());
    dbg!(allocate_new_id());
}
