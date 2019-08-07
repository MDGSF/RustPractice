fn call<F: FnOnce()>(f: F) {
    f()
}

fn main() {
    let mut x = 0;
    let incr_x = || x += 1;
    call(incr_x);
    // call(incr_x);

    let mut x = 0;
    let incr_x = move || x += 1;
    call(incr_x);
    call(incr_x);

    let mut x = vec![];
    let expand_x = move || x.push(42);
    call(expand_x);
    // call(expand_x);
}
