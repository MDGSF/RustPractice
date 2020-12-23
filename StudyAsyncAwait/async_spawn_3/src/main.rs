use async_std::task;
use std::time::Duration;

fn main() {
  task::spawn(myloop());

  std::thread::sleep(std::time::Duration::from_secs(1000));
}

async fn myloop() {
  for i in 0.. {
    println!("myloop i = {}", i);
    task::sleep(Duration::from_secs(1)).await;
  }
}
