use std::fmt;

struct Point1<T> {
    x: T,
    y: T,
}

impl<T> Point1<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }
}

impl<T: fmt::Display + PartialOrd> Point1<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

impl Point1<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[derive(Debug)]
struct Point2<A, B> {
    x: A,
    y: B,
}

impl<A, B> Point2<A, B> {
    fn mixup<C, D>(self, other: Point2<C, D>) -> Point2<A, D> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    println!("Hello, world!");

    let integer = Point1 { x: 5, y: 10 };
    let float = Point1 { x: 1.0, y: 4.0 };
    println!("interger.x = {}", integer.x());
    println!("float.x = {}", float.x());
    println!(
        "float.distance_from_origin = {}",
        float.distance_from_origin()
    );

    // expected integer, found floating-point number
    // let wont_work = Point1 { x: 5, y: 4.0 };

    let p1 = Point2 { x: 5, y: 4.0 };
    let p2 = Point2 {
        x: "hello",
        y: "world",
    };
    let p3 = p1.mixup(p2);
    println!("p3 = {:?}", p3);
}
