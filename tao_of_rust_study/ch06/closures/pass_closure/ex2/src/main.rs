use std::ops::Fn;

trait Any {
    fn any<F>(&self, f: F) -> bool
    where
        Self: Sized,
        F: Fn(u32) -> bool;
}

impl Any for Vec<u32> {
    fn any<F>(&self, f: F) -> bool
    where
        Self: Sized,
        F: Fn(u32) -> bool,
    {
        for &x in self {
            if f(x) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let v = vec![1, 2, 3];
    let b = v.any(|x| x == 3);
    println!("{:?}", b);
}
