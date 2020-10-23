#[derive(Debug, Clone)]
pub struct Direction {
  pub row: isize,
  pub col: isize,
  pub name: &'static str,
}

lazy_static! {
  pub static ref DIRECTIONS: Vec<Direction> = {
    let mut m = Vec::new();
    m.push(Direction {
      row: 0,
      col: -1,
      name: "L",
    });
    m.push(Direction {
      row: 0,
      col: 1,
      name: "R",
    });
    m.push(Direction {
      row: -1,
      col: 0,
      name: "U",
    });
    m.push(Direction {
      row: 1,
      col: 0,
      name: "D",
    });
    m
  };
}
