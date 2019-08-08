use std::iter::FromIterator;

#[derive(Debug)]
struct MyVec(Vec<i32>);

impl MyVec {
    fn new() -> MyVec {
        MyVec(Vec::new())
    }
    fn add(&mut self, elem: i32) {
        self.0.push(elem);
    }
}

impl FromIterator<i32> for MyVec {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut c = MyVec::new();
        for i in iter {
            c.add(i);
        }
        c
    }
}

fn main() {
    let iter = (0..5).into_iter();
    let c = MyVec::from_iter(iter);
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);

    let iter = (0..5).into_iter();
    let c: MyVec = iter.collect();
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);

    let iter = (0..5).into_iter();
    let c = iter.collect::<MyVec>();
    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
}
