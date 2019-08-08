fn modify(mut v: Vec<u32>) -> Vec<u32> {
    v.push(42);
    v
}

fn main() {
    let v = vec![1, 2, 3];
    let v = modify(v);
    println!("{:?}", v);
}
