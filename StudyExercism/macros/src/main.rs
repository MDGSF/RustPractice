use macros::hashmap;

fn main() {
  let m = hashmap!(
      'h' => 89,
      'a' => 1,
      's' => 19,
      'h' => 8,
  );

  let m = macros::hashmap!(23 => 623, 34 => 21,);

  let m: ::std::collections::HashMap<(), ()> = hashmap!();

  println!("{:?}", m);
}
