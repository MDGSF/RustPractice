fn swap(a: *mut i32, b: *mut i32) {
    unsafe {
        let t = *a;
        *a = *b;
        *b = t;
    }
}

fn main() {
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;
    let y = Box::new(20);
    let ptr_y = &*y as *const i32;
    unsafe {
        *ptr_x += *ptr_y;
    }
    assert_eq!(x, 30);

    let mut a = 1;
    let mut b = 2;
    let ptr_a = &mut a as *mut i32;
    let ptr_b = &mut b as *mut i32;
    swap(ptr_a, ptr_b);
    println!("a = {}, b = {}", a, b);
}
