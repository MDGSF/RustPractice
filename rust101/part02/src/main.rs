pub enum SomethingOrNothing<T> {
    Something(T),
    Nothing,
}

pub use self::SomethingOrNothing::*;

type NumberOrNothing = SomethingOrNothing<i32>;

impl<T> SomethingOrNothing<T> {
    fn new(o: Option<T>) -> Self {
        match o {
            None => Nothing,
            Some(t) => Something(t),
        }
    }

    fn to_option(self) -> Option<T> {
        match self {
            Nothing => None,
            Something(t) => Some(t),
        }
    }
}

fn call_constructor(x: i32) -> SomethingOrNothing<i32> {
    SomethingOrNothing::new(Some(x))
}

pub trait Minimum: Copy {
    fn min(self, b: Self) -> Self;
}

fn main() {
    println!("Hello, world!");
}
