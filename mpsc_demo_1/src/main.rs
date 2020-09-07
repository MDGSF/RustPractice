use std::sync::mpsc::channel;

fn main() {
  let (sender, receiver) = channel();
  sender.send(5);
  println!("{:?}", receiver.recv());
}
