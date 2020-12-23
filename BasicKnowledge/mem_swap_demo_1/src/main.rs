use std::{fmt, mem};

struct Ring {
  owner: String,
  former_owner: String,
  seeker: String,
}

impl Ring {
  fn new(owner: &str, former_owner: &str, seeker: &str) -> Self {
    Self {
      owner: owner.to_string(),
      former_owner: former_owner.to_string(),
      seeker: seeker.to_string(),
    }
  }
}

impl fmt::Display for Ring {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} has the ring, {} used to have it, and {} wants it",
      self.owner, self.former_owner, self.seeker
    )
  }
}

fn main() {
  let mut one_ring = Ring::new("Frodo", "Gollum", "Sauron");
  println!("{}", one_ring);

  // let t = one_ring.owner;
  // one_ring.owner = one_ring.former_owner;
  // one_ring.former_owner = t;
  mem::swap(&mut one_ring.owner, &mut one_ring.former_owner);
  println!("{}", one_ring);
}
