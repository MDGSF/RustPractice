#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]

pub struct Step<I> {
    iter: I,
    skip: usize,
}

impl<I> Iterator for Step<I>
where
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        let elt = self.iter.next();
        if self.skip > 0 {
            self.iter.nth(self.skip - 1);
        }
        elt
    }
}

pub fn step<I>(iter: I, step: usize) -> Step<I>
where
    I: Iterator,
{
    assert!(step != 0);
    Step {
        iter: iter,
        skip: step - 1,
    }
}

pub trait IterExt: Iterator {
    fn step(self, n: usize) -> Step<Self>
    where
        Self: Sized,
    {
        step(self, n)
    }
}

impl<T: ?Sized> IterExt for T where T: Iterator {}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    let sum = arr.iter().step(2).fold(0, |acc, x| acc + x);
    assert_eq!(sum, 9);
}
