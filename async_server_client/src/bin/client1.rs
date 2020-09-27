// use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use futures::join;
use std::time;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
  task::block_on(runtime())
}

async fn runtime() -> Result<()> {
  let t1 = task::spawn(c1());
  let t2 = task::spawn(c2());
  join!(t1, t2);
  Ok(())
}

async fn c1() {
  if let Err(err) = c1_inner().await {
    println!("err = {}", err);
  }
}

async fn c1_inner() -> Result<()> {
  let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
  println!("Connected to {}", &stream.peer_addr()?);

  loop {
    let msg = "c1 hello world";
    println!("<- {}", msg);
    stream.write_all(msg.as_bytes()).await?;

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    println!("-> {}", String::from_utf8_lossy(&buf[..n]));

    task::sleep(time::Duration::from_secs(1)).await;
  }
}

async fn c2() {
  if let Err(err) = c2_inner().await {
    println!("err = {}", err);
  }
}

async fn c2_inner() -> Result<()> {
  let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
  println!("Connected to {}", &stream.peer_addr()?);

  loop {
    let msg = "c2 hello world";
    println!("<- {}", msg);
    stream.write_all(msg.as_bytes()).await?;

    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    println!("-> {}", String::from_utf8_lossy(&buf[..n]));

    task::sleep(time::Duration::from_secs(1)).await;
  }
}
