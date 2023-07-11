use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    // 用 move 把 numbers 的所有权转移到线程内部。
    // 如果没有 move，那就是线程内部引用 numbers 变量，
    // 而线程的生命周期是可能大于 numbers 变量的，所以不合法。
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
}
