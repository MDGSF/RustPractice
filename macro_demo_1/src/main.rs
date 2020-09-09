macro_rules! give_size {
  () => {
    6
  };
}

fn main() {
  let six = give_size!();
  println!("{}", six);
}
