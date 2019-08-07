fn main() {
    let mut x = vec![1];
    x.push(x.pop().unwrap());
}
