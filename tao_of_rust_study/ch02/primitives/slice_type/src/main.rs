fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    // assert_eq!(&arr, [1, 2, 3, 4, 5]);
    assert_eq!(&arr[1..], &[2, 3, 4, 5]);
    assert_eq!(&arr[1..], [2, 3, 4, 5]);
    assert_eq!(&arr.len(), &5);
    assert_eq!(arr.len(), 5);
    assert_eq!(&arr.is_empty(), &false);
    assert_eq!(arr.is_empty(), false);

    let arr = &mut [1, 2, 3];
    arr[1] = 7;
    assert_eq!(arr, &[1, 7, 3]);

    let vec = vec![1, 2, 3];
    assert_eq!(&vec[..], &[1, 2, 3]);
    assert_eq!(&vec[..], [1, 2, 3]);

    let str_slice: &[&str] = &["one", "two", "three"];
    assert_eq!(str_slice, &["one", "two", "three"]);
    assert_eq!(str_slice, ["one", "two", "three"]);
}
