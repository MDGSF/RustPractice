use async_std::task;
use std::time::Duration;

fn main() {
  task::spawn(loop2());

  task::block_on(runtime());
}

async fn runtime() {
  start_loop1();

  for i in 0.. {
    println!("runtime i = {}", i);
    task::sleep(Duration::from_secs(1)).await;
  }
}

fn start_loop1() {
  task::spawn(loop1());
}

async fn loop1() {
  for i in 0.. {
    println!("loop1 i = {}", i);
    task::sleep(Duration::from_secs(1)).await;
  }
}

async fn loop2() {
  for i in 0.. {
    println!("loop2 i = {}", i);
    task::sleep(Duration::from_secs(1)).await;
  }
}
