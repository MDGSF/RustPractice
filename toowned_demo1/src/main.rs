fn main() {
  println!("Hello, world!");
  let s1: &str = "hello";
  let s2: String = s1.to_owned();
  println!("s2 = {}", s2);

  let v1: &[i32] = &[1, 2];
  let v2: Vec<i32> = v1.to_owned();
  println!("v2 = {:?}", v2);
}
