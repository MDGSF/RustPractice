fn main() {
  println!("Hello, world!");
  test1();
}

fn test1() -> i32 {
  test2()
}

#[track_caller]
fn test2() -> i32 {
  let t = None;
  match t {
    Some(val) => val,
    None => panic!("None"),
  }
}
