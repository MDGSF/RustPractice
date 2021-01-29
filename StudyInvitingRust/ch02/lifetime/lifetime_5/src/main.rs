fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
  if s1.len() > s2.len() {
    s1
  } else {
    s2
  }
}

fn main() {
  let s1 = String::from("Rust");
  let s1_r = &s1;
  {
    let s2 = String::from("C");
    let result = the_longest(s1_r, &s2);
    println!("{} is the longest", result);
  }
}
