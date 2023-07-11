use std::thread;

fn main() {
    thread::spawn(f);
    thread::spawn(f);

    println!("Hello from the main thread.");
}

fn f() {
    println!("Hello from another thread!");

    // 可以拷贝，可以比较是否相等，保证每个线程都不一样。
    // 不保证是连续的。
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
