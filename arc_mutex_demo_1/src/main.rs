use std::sync::{Arc, Mutex};

fn test1() {
  let my_number = Arc::new(Mutex::new(0));

  let my_number1 = Arc::clone(&my_number);
  let my_number2 = Arc::clone(&my_number);

  let thread1 = std::thread::spawn(move || {
    for _ in 0..10 {
      *my_number1.lock().unwrap() += 1;
    }
  });

  let thread2 = std::thread::spawn(move || {
    for _ in 0..10 {
      *my_number2.lock().unwrap() += 1;
    }
  });

  thread1.join().unwrap();
  thread2.join().unwrap();
  println!("Value is: {:?}", my_number);
}

fn test2() {
  let my_number = Arc::new(Mutex::new(0));
  let mut handle_vec = vec![];

  for _ in 0..2 {
    let my_number_clone = Arc::clone(&my_number);
    let handle = std::thread::spawn(move || {
      for _ in 0..10 {
        *my_number_clone.lock().unwrap() += 1;
      }
    });
    handle_vec.push(handle);
  }

  handle_vec
    .into_iter()
    .for_each(|handle| handle.join().unwrap());
  println!("{:?}", my_number);
}

fn main() {
  test2();
}
