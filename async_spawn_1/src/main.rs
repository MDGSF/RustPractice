use futures::{channel::mpsc, select, FutureExt, SinkExt};

use async_std::{
  io::BufReader,
  net::{TcpListener, TcpStream, ToSocketAddrs},
  prelude::*,
  task,
};

use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
  task::block_on(runtime());
}

async fn runtime() {
  task::spawn(loop1());

  for i in 0.. {
    println!("runtime i = {}", i);
    task::sleep(Duration::from_secs(1)).await;
  }
}

async fn loop1() {
  for i in 0.. {
    println!("loop1 i = {}", i);
    task::sleep(Duration::from_secs(1)).await;
  }
}
