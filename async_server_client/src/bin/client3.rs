// use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::task;
use futures::join;
use std::sync::mpsc::channel;
use std::thread;
use std::time;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
  let (tx, rx) = channel();
  thread::spawn(move || loop {
    let data = rx.recv().unwrap();
    println!("data = {}", String::from_utf8(data).unwrap());
  });
  start_runtime(tx);
  Ok(())
}

fn start_runtime(tx: std::sync::mpsc::Sender<Vec<u8>>) {
  if let Err(err) = task::block_on(runtime(tx)) {
    println!("err = {}", err);
  }
}

async fn runtime(tx: std::sync::mpsc::Sender<Vec<u8>>) -> Result<()> {
  let t1 = task::spawn(c1(1, tx.clone()));
  let t2 = task::spawn(c1(2, tx.clone()));
  let t3 = task::spawn(c1(3, tx));
  join!(t1, t2, t3);
  Ok(())
}

async fn c1(id: i32, tx: std::sync::mpsc::Sender<Vec<u8>>) {
  if let Err(err) = c1_inner(id, tx).await {
    println!("err = {}", err);
  }
}

async fn c1_inner(id: i32, tx: std::sync::mpsc::Sender<Vec<u8>>) -> Result<()> {
  let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
  println!("Connected to {}", &stream.peer_addr()?);

  loop {
    let mut buf = vec![0u8; 1024];
    let n = stream.read(&mut buf).await?;
    println!("[{}] -> {}", id, String::from_utf8_lossy(&buf[..n]));

    tx.send(buf[..n].to_vec()).unwrap();

    task::sleep(time::Duration::from_secs(1)).await;
  }
}
