use turtle::Turtle;

fn main() {
  let mut turtle = Turtle::new();

  turtle.set_fill_color("black");
  turtle.begin_fill();
  turtle.pen_up();
  turtle.forward(130.0);
  turtle.pen_down();

  gear(&mut turtle);
  turtle.end_fill();

  turtle.pen_up();
  turtle.forward(45.0);
  turtle.left(90.0);
  turtle.pen_down();

  inner_gear(&mut turtle);

  letter(&mut turtle);

  // turtle.hide();
}

fn gear(turtle: &mut Turtle) {
  let teeth = 32.0;
  let tooth_size = 20.0;
  let angle = 360.0 / teeth;
  turtle.right(180.0 - angle * 6.0 / 2.0);
  for _ in 0..teeth as u64 {
    turtle.forward(tooth_size);
    turtle.left(180.0 - angle * 7.0);
    turtle.forward(tooth_size);
    turtle.right(180.0 - angle * 6.0);
  }
  turtle.right(angle * 6.0 / 2.0);
}

fn inner_gear(turtle: &mut Turtle) {
  inner_circle(turtle);
  inner_teeth(turtle);
}

fn inner_circle(turtle: &mut Turtle) {
  turtle.set_fill_color("white");
  turtle.begin_fill();
  for _ in 0..360 {
    turtle.forward(1.5);
    turtle.right(1.0);
  }
  turtle.end_fill();
  turtle.set_fill_color("black");
}

fn inner_teeth(turtle: &mut Turtle) {
  let tooth_size = 40.0;
  turtle.pen_up();
  turtle.left(90.0);
  turtle.forward(10.0);
  turtle.right(90.0);
  turtle.pen_down();

  for _ in 0..5 {
    turtle.pen_up();
    turtle.forward(tooth_size / 2.0);
    turtle.pen_down();

    turtle.begin_fill();
    turtle.right(120.0);
    turtle.forward(tooth_size);
    turtle.right(120.0);
    turtle.forward(tooth_size);
    turtle.right(120.0);
    turtle.end_fill();

    turtle.pen_up();
    turtle.forward(tooth_size / 2.0);
    turtle.right(90.0);
    turtle.forward(5.0);
    turtle.left(90.0);
    turtle.pen_down();

    inner_tooth_circle(turtle);

    turtle.pen_up();
    turtle.left(90.0);
    turtle.forward(5.0);
    turtle.right(90.0);
    for _ in 0..360 / 5 / 2 {
      turtle.forward(3.41);
      turtle.right(2.0);
    }
    turtle.pen_down();
  }
}

fn inner_tooth_circle(turtle: &mut Turtle) {
  turtle.set_fill_color("white");
  turtle.begin_fill();
  for _ in 0..360 / 8 {
    turtle.forward(1.0);
    turtle.right(8.0);
  }
  turtle.end_fill();
  turtle.set_fill_color("black");
}

fn letter(turtle: &mut Turtle) {}
