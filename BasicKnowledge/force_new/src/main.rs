mod foo {
  pub struct Point {
    pub x: i32,
    pub y: i32,
    _secret: (),
  }

  impl Point {
    pub fn new(x: i32, y: i32) -> Point {
      Point { x, y, _secret: () }
    }
  }
}

fn main() {
  let _p = foo::Point::new(1, 2);
}
