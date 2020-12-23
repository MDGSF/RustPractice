/*
 * 转移语义自动实现 FnOnce
 * 实现 FnOnce 不需要实现 Fn 和 FnMut
 */
fn main() {
    let s = "hello".to_string();
    let c = || s;
    c();
    // c(); // value used here after move
    // println!("{:?}", s);
}
