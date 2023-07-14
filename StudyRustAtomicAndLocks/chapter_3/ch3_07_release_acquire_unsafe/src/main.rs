use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::thread;
use std::time::Duration;

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        unsafe { DATA = 123 };
        READY.store(true, Release); // Release 保证前面的指令不会重排到这行指令后面
    });

    while !READY.load(Acquire) {
        // Acquire 保证后面的指令不会重排到这行指令前面
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}", unsafe { DATA });
}
