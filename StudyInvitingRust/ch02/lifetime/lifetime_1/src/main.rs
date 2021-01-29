fn main() {
  let mut v = vec![];
  v.push(1);
  {
    println!("{:?}", v[0]);
    v.push(2);
  }
}
