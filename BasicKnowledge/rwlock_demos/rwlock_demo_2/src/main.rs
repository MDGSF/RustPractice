use std::mem::drop;
use std::sync::RwLock;

fn main() {
  let my_rwlock = RwLock::new(5);

  let read1 = my_rwlock.read().unwrap();
  let read2 = my_rwlock.read().unwrap();

  if let Ok(mut number) = my_rwlock.try_write() {
    *number += 10;
    println!("Now the number is {}", number);
  } else {
    println!("Couldn't get write access, sorry!");
  };
}
