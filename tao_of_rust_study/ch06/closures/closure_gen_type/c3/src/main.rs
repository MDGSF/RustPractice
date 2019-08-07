/*
 * 实现 Fn，必然要实现 FnMut 和 FnOnce
 */
#![feature(fn_traits)]
fn main() {
    let s = "hello";
    let mut c = || println!("{:?}", s);
    c();
    c();
    c.call_mut(());
    c.call_once(());
    c(); // 这里没有被转移所有权是因为生成的 FnOnce 自动实现的 Copy
    println!("{:?}", s);
}
