fn main() {
    let s = String::from("hello");
    let a = &mut s[..];
    a[0] = 'b';
}
