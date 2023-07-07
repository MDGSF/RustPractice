fn main() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());
    
    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    v3.dedup();
    println!("{v3:?}");
}
