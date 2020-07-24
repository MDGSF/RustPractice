fn main() {
  let mut created = false;
  test(&mut created);
  println!("created = {}", created);
}

fn test(created: &mut bool) {
  *created = true;
}
