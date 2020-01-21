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
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn square(bottom_left: Point, slide_length: f32) -> Rectangle {
  Rectangle {
    top_left: Point {
      x: bottom_left.x,
      y: bottom_left.y + slide_length,
    },
    bottom_right: Point {
      x: bottom_left.x + slide_length,
      y: bottom_left.y,
    },
  }
}

fn rect_area(rect: Rectangle) -> f32 {
  let Rectangle {
    top_left,
    bottom_right,
  } = rect;

  let Point {
    x: top_left_x,
    y: top_left_y,
  } = top_left;

  let Point {
    x: bottom_right_x,
    y: bottom_right_y,
  } = bottom_right;

  let width = (top_left_x - bottom_right_x).abs();
  let height = (top_left_y - bottom_right_y).abs();
  width * height
}

fn main() {
  let name = "Peter";
  let age = 27;
  let peter = Person { name, age };
  println!("{:?}", peter);

  let point: Point = Point { x: 10.3, y: 0.4 };
  println!("point coordinates: ({}, {})", point.x, point.y);

  let bottom_right = Point { x: 5.2, ..point };
  println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

  let Point {
    x: top_edge,
    y: left_edge,
  } = point;

  let rectangle = Rectangle {
    top_left: Point {
      x: left_edge,
      y: top_edge,
    },
    bottom_right: bottom_right,
  };
  println!("rectangle = {:?}", rectangle);
  println!("rectangle area = {:?}", rect_area(rectangle));

  let _nil = Nil;

  let pair = Pair(1, 0.1);

  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  let Pair(integer, decimal) = pair;

  println!("pair contains {:?} and {:?}", integer, decimal);

  println!("square = {:?}", square(Point { x: 0.0, y: 0.0 }, 1.0));
}
