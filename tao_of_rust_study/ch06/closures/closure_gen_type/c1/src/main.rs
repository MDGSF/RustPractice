/*
 * Fn > FnMut > FnOnce
 *
 * 实现 Fn，必然要实现 FnMut 和 FnOnce
 * 实现 FnMut，必然要实现 FnOnce
 *
 */
fn main() {
    let s = "hello";
    let c = || println!("{:?}", s);
    c();
    c();
    println!("{:?}", s);
}
