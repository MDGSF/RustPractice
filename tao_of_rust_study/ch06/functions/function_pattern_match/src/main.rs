#[derive(Debug)]
struct S {
  i: i32,
}

fn f(ref s: S) {
  println!("{:p}, {:?}", s, s);
  // let _: u8 = s;
}

fn foo(_: i32) {}

fn swap((x, y): (&str, i32)) -> (i32, &str) {
  (y, x)
}

fn main() {
  let s = S { i: 42 };
  println!("main s = {:?}", s);
  f(s);
  println!("main s = {:?}", s);

  foo(3);

  let t = ("Alex", 18);
  let t = swap(t);
  assert_eq!(t, (18, "Alex"));
}
