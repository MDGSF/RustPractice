fn main() {
    println!("Hello, world!");

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
    vec[0] = 7;
    assert_eq!(vec[0], 7);
    assert_eq!(vec.get(0), Some(&7));
    assert_eq!(vec.get(10), None);

    vec.extend([1, 2, 3].iter().cloned());
    assert_eq!(vec, [7, 1, 2, 3]);
    assert_eq!(vec.get(0..2), Some(&[7, 1][..]));

    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    assert_eq!(vec, [7, 1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);

    vec.swap(1, 3);
    assert_eq!(vec, [7, 3, 2, 1, 4, 5, 6]);

    let slice = [1, 2, 3, 4, 5, 6, 7];
    vec.copy_from_slice(&slice);
    assert_eq!(vec, slice);

    //let slice = [4, 3, 2, 1];
    //vec.clone_from_slice(&slice);
    //assert_eq!(vec, slice);
}
