#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn rect_area(&self) -> f32 {
        ((self.p1.x - self.p2.x) * (self.p1.y - self.p2.y)).abs()
    }
}

fn square(point: &Point, len: f32) -> Rectangle {
    Rectangle {
        p1: Point { ..*point },
        p2: Point {
            x: point.x + len,
            y: point.y + len,
        },
    }
}

fn main() {
    println!("Hello, world!");

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 0.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    let new_point = Point { x: 0.1, ..point };
    println!("second point: ({}, {})", new_point.x, new_point.y);

    let Point { x: my_x, y: my_y } = point;

    let _rectange = Rectangle {
        p1: Point { x: my_y, y: my_x },
        p2: point,
    };
    println!("_rectange is: {:?}", _rectange);
    println!("The area of _rectange is:{}", _rectange.rect_area());

    let p1 = Point { x: 0.0, y: 0.0 };
    println!("square is: {:?}", square(&p1, 3.0));

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
