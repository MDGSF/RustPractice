use rand;

// 只要 t 中的所有数据都拥有所有权，那 t 就是满足 'static 的
fn drop_static<T: 'static>(t: T) {
  std::mem::drop(t);
}

fn main() {
  let mut strings: Vec<String> = Vec::new();
  for _ in 0..10 {
    if rand::random() {
      let string = rand::random::<u64>().to_string();
      strings.push(string);
    }
  }

  for mut string in strings {
    string.push_str("a mutation");
    drop_static(string);
  }

  println!("I am the end of the program");
}
