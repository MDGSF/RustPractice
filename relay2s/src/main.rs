use crossbeam_channel::bounded;
use futures::io;
use futures::prelude::*;
use smol::{Async, Task};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {}

fn test() {
  let (c1s, c1r) = bounded(128);
  let (c2s, c2r) = bounded(128);

  let t1 = thread::spawn(move || {
    for i in 0..10 {
      c1s.send(i).unwrap();
      let ret = c2r.recv().unwrap();
      println!("ret = {}", ret);
    }
  });

  let t2 = thread::spawn(move || loop {
    if let Ok(data) = c1r.recv() {
      c2s.send(data).unwrap();
    } else {
      break;
    }
  });

  t1.join().unwrap();
  t2.join().unwrap();
}
