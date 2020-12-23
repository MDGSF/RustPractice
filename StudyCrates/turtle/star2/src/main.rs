use turtle::Turtle;

const WINDING: f64 = 3.0;

const POINTS: f64 = 5.0;

const TURN: f64 = 360.0 * WINDING / POINTS;

fn main() {
  let mut turtle = Turtle::new();

  turtle.right(90.0);
  turtle.set_pen_size(2.0);
  turtle.set_pen_color("yellow");

  for _ in 0..POINTS as usize {
    turtle.forward(300.0);
    turtle.right(TURN);
  }
}
