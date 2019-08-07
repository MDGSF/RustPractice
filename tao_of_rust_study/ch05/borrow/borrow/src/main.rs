fn foo(mut v: [i32; 3]) -> [i32; 3] {
    v[0] = 3;
    assert_eq!([3, 2, 3], v);
    v
}

fn test1() {
    let v = [1, 2, 3];
    let v2 = foo(v);
    assert_eq!([1, 2, 3], v);
    assert_eq!([3, 2, 3], v2);
}

fn bar(v: &mut [i32; 3]) {
    v[0] = 3;
}

fn test2() {
    let mut v = [1, 2, 3];
    bar(&mut v);
    assert_eq!([3, 2, 3], v);
}

fn main() {
    test1();
    test2();
}
