struct S {
  x: i32,
}

const S: S = S { x: 2 };

fn main() {
  let v = &mut S; // let v = &mut S { x: 2};
  v.x += 1;
  S.x += 1; // S { x: 2 }.x += 1
  println!("{}{}", v.x, S.x); // println!("{}", S { x: 2 }.x);
}
