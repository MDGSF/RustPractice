use std::sync::Arc;
use std::thread;

fn main() {
    test01();
    test02();
}

// bad
fn test01() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();
    let t = thread::spawn(move || {
        dbg!(b);
    });
    dbg!(a);
    t.join().unwrap();
}

// good
// 不需要再起一个新的变量名字
fn test02() {
    let a = Arc::new([4, 5, 6]);
    let t = thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    });
    dbg!(a);
    t.join().unwrap();
}
