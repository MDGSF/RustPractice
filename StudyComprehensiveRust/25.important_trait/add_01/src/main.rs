#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

/*
Why is Output an associated type? Could it be made a type parameter of the method?
Short answer: Function type parameters are controlled by the caller, 
 but associated types (like Output) are controlled by the implementor of a trait.
*/
impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
}
