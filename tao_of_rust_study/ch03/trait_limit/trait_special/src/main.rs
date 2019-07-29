#![feature(specialization)]
struct Diver<T> {
    inner: T,
}

trait Swimmer {
    fn swim(&self) {
        println!("swimming")
    }
}

impl<T> Swimmer for Diver<T> {}

impl Swimmer for Diver<&'static str> {
    fn swim(&self) {
        println!("drowning, help!")
    }
}

fn main() {
    let x = Diver::<&'static str> { inner: "Bob" };
    x.swim();
    let y = Diver::<String> {
        inner: "Bob".to_string(),
    };
    y.swim();
}
