#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
#[derive(Debug)]
pub struct Positions<I, F> {
    iter: I,
    f: F,
    count: usize,
}

pub fn positions<I, F>(iter: I, f: F) -> Positions<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> bool,
{
    Positions {
        iter: iter,
        f: f,
        count: 0,
    }
}

impl<I, F> Iterator for Positions<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> bool,
{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.iter.next() {
            let i = self.count;
            self.count = i + 1;
            if (self.f)(v) {
                return Some(i);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
}

impl<I, F> DoubleEndedIterator for Positions<I, F>
where
    I: DoubleEndedIterator + ExactSizeIterator,
    F: FnMut(I::Item) -> bool,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.iter.next_back() {
            if (self.f)(v) {
                return Some(self.count + self.iter.len());
            }
        }
        None
    }
}

pub trait Itertools: Iterator {
    fn positions<P>(self, predicate: P) -> Positions<Self, P>
    where
        Self: Sized,
        P: FnMut(Self::Item) -> bool,
    {
        positions(self, predicate)
    }
}

impl<T: ?Sized> Itertools for T where T: Iterator {}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let r = data.iter().positions(|v| v % 3 == 0);
    let rev_r = data.iter().positions(|v| v % 3 == 0).rev();
    for i in r {
        println!("{:?}", i);
    }
    for i in rev_r {
        println!("{:?}", i);
    }
}
