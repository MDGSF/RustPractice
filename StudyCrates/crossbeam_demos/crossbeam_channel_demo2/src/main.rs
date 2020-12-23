use crossbeam::crossbeam_channel::*;
use std::thread;
use std::time::Duration;

fn main() {
  println!("Hello, world!");
  let (s1, r1) = unbounded();
  let (s2, r2) = unbounded();

  thread::spawn(move || s1.send(10).unwrap());
  thread::spawn(move || s2.send(20).unwrap());

  select! {
    recv(r1) -> msg => println!("{:?}", msg),
    recv(r2) -> msg => println!("{:?}", msg),
    default(Duration::from_secs(1)) => println!("timed out"),
  }
}
