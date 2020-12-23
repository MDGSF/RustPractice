#[derive(PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let box_point = Box::new(Point { x: 0.0, y: 0.0 });
    let unboxed_point: Point = *box_point;
    assert_eq!(unboxed_point, Point { x: 0.0, y: 0.0 });
    println!("unboxed_point = {:?}", unboxed_point);
}
