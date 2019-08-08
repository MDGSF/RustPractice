fn hello() {
    println!("hello function pointer");
}

fn main() {
    let fn_ptr: fn() = hello;
    println!("{:?}", fn_ptr);
    let other_fn = hello;
    hello();
    other_fn();
    fn_ptr();
    (fn_ptr)();
}
