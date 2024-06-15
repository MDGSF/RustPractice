use std::thread;

// 用静态变量在多个线程间共享数据。
static X: [i32; 3] = [1, 2, 3];

fn main() {
    let t1 = thread::spawn(|| dbg!(&X));
    let t2 = thread::spawn(|| dbg!(&X));
    t1.join().unwrap();
    t2.join().unwrap();
}
