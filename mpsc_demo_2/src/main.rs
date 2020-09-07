use std::sync::mpsc::channel;

fn main() {
  let (sender, receiver) = channel();
  let sender_clone = sender.clone();
  let mut handle_vec = vec![];
  let mut results_vec = vec![];

  handle_vec.push(std::thread::spawn(move || {
    sender.send("Send a &str this time").unwrap();
  }));

  handle_vec.push(std::thread::spawn(move || {
    sender_clone.send("And here is another &str").unwrap();
  }));

  for _ in handle_vec {
    results_vec.push(receiver.recv().unwrap());
  }
  println!("{:?}", results_vec);
}
