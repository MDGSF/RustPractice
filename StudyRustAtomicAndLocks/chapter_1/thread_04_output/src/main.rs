use std::thread;

fn main() {
    let numbers = Vec::from_iter(0..=100);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });

    // 获取线程执行结束的返回值
    let average = t.join().unwrap();

    println!("average: {average}");
}
