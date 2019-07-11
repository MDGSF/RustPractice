fn main() {
    println!("Hello, world!");

    let vec = vec![1, 2, 3];
    let int_slice = &vec[..];
    let str_slice: &[&str] = &["one", "two", "three"];

    let mut x = [1, 2, 3];
    let x = &mut x[..];
    x[1] = 7;
    assert_eq!(x, &[1, 7, 3]);

    let a = [1, 2, 3];
    assert_eq!(a.len(), 3);
    assert_eq!(a.is_empty(), false);

    let v = [10, 40, 30];
    assert_eq!(Some(&10), v.first());
    let w: &[i32] = &[];
    assert_eq!(None, w.first());

    let x = &mut [0, 1, 2];
    if let Some(first) = x.first_mut() {
        *first = 5;
    }
    assert_eq!(x, &[5, 1, 2]);

    let x = &[0, 1, 2];
    if let Some((first, elements)) = x.split_first() {
        assert_eq!(first, &0);
        assert_eq!(elements, &[1, 2]);
    }

    let x = &mut [0, 1, 2];
    if let Some((first, elements)) = x.split_first_mut() {}

    let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
    a.rotate_left(2);
    assert_eq!(a, ['c', 'd', 'e', 'f', 'a', 'b'])
}
