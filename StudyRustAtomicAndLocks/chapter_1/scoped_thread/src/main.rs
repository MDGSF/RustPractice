use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    // scope 函数结束的时候，会确保内部 spawn 的线程都结束，
    // 所以内部的线程可以引用外部的变量
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}
