fn main() {
    let mut vec = Vec::with_capacity(10);
    for i in 0..10 {
        vec.push(i);
    }
    vec.truncate(0);
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    for i in 0..10 {
        vec.push(i);
    }
    vec.clear();
    assert!(vec.is_empty());
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    vec.shrink_to_fit();
    assert_eq!(vec.capacity(), 0);

    for i in 0..80 {
        vec.push(i);
        print!("{:?}/", vec.capacity());
    }
}
