use turtle::{Color, Point, Turtle};

fn draw(turtle: &mut Turtle, side: f64, depth: usize) {
  for _ in 0..4 {
    let position = turtle.position();
    turtle.set_pen_color(get_color(position));
    turtle.right(90.0);
    turtle.forward(side);
  }

  if depth > 0 {
    let smaller_side = side / f64::sqrt(2.0);

    turtle.pen_up();
    turtle.left(45.0);
    turtle.forward(smaller_side);
    let position = turtle.position();
    turtle.pen_down();

    let heading = turtle.heading();

    draw(turtle, smaller_side, depth - 1);

    turtle.set_heading(heading);

    turtle.pen_up();
    turtle.go_to(position);
    turtle.right(135.0);
    turtle.forward(side);
    turtle.left(45.0);
    turtle.forward(smaller_side);
    turtle.pen_down();

    draw(turtle, smaller_side, depth - 1);
  }
}

fn get_color(position: Point) -> Color {
  Color {
    red: 130.0 + f64::abs(position[0] % 75.0),
    green: 150.0 + f64::abs(position[1] % 55.0),
    blue: 210.0 + f64::abs(position[1] % 25.0),
    alpha: 0.8,
  }
}

fn main() {
  let mut turtle = Turtle::new();
  println!("{:?}", turtle.position());

  let side = 100.0;

  turtle.pen_up();
  turtle.backward(1.5 * side);
  turtle.pen_down();
  println!("{:?}", turtle.position());

  draw(&mut turtle, side, 6);
}
