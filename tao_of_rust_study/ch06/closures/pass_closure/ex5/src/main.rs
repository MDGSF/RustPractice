fn main() {
    let s = "hello";
    let c: Box<dyn Fn() + 'static> = Box::new(move || {
        s;
    });
}
