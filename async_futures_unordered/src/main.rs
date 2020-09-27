//use async_std::io;
//use async_std::net::{TcpListener, TcpStream};
//use async_std::prelude::*;
use async_std::task;
use futures::stream::{FuturesUnordered, StreamExt};
use rand::distributions::{Distribution, Uniform};
use std::time::Duration;

async fn random_sleep(num: i64) -> i64 {
  let mut rng = rand::thread_rng();
  let secs: u64 = Uniform::from(2..10).sample(&mut rng);
  let d = Duration::from_secs(secs);
  println!("--> [{}] sleep {} seconds", num, secs);
  task::sleep(d).await;
  println!("<-- [{}] slept {} seconds", num, secs);
  num
}

async fn run() {
  let mut cnt = 0;
  let mut workers = FuturesUnordered::new();

  while workers.len() < 5 {
    workers.push(random_sleep(cnt));
    cnt += 1;
  }

  loop {
    match workers.next().await {
      Some(result) => {
        println!("    finished future [{}]", result);
        if cnt < 20 {
          workers.push(random_sleep(cnt));
          cnt += 1;
        }
      }
      None => {
        println!("Done!");
        break;
      }
    }
  }
}

// Could also use #[tokio::main]
fn main() {
  task::block_on(run());
}
