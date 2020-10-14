#![recursion_limit="256"]

use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use futures::join;
use futures::{channel::mpsc, select, FutureExt, SinkExt};
// use std::sync::mpsc::channel;
use std::collections::HashMap;
use std::collections::HashSet;
use std::thread;
use std::time::Duration;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

fn main() -> io::Result<()> {
  let (mut tx_new_data, rx_new_data) = mpsc::unbounded();
  thread::spawn(move || {
    for i in 0.. {
      let input = format!("hello world, {}", i);
      task::block_on(async {
        tx_new_data.send(input.as_bytes().to_vec()).await.unwrap();
      });
      thread::sleep(Duration::from_secs(1));
    }
  });
  start_runtime(rx_new_data);
  Ok(())
}

fn start_runtime(rx_new_data: Receiver<Vec<u8>>) {
  if let Err(err) = task::block_on(runtime(rx_new_data)) {
    println!("err = {}", err);
  }
}

async fn runtime(rx_new_data: Receiver<Vec<u8>>) -> Result<()> {
  let (tx_new_client, rx_new_client) = mpsc::unbounded();
  let t1 = task::spawn(server(tx_new_client));
  let t2 = task::spawn(broadcast(rx_new_data, rx_new_client));
  join!(t1, t2);
  Ok(())
}

async fn broadcast(
  rx_new_data: Receiver<Vec<u8>>,
  rx_new_client: Receiver<TcpStream>,
) {
  if let Err(err) = broadcast_inner(rx_new_data, rx_new_client).await {
    println!("err = {}", err);
  }
}

async fn broadcast_inner(
  mut rx_new_data: Receiver<Vec<u8>>,
  mut rx_new_client: Receiver<TcpStream>,
) -> Result<()> {
  let mut uniq_client_id = 0;
  let mut clients: HashMap<i32, TcpStream> = HashMap::new();

  loop {
    select! {
      data = rx_new_data.next().fuse() => match data {
          Some(data) => {
            println!("new data = {:?}", String::from_utf8(data.clone()).unwrap());

            let mut invalid_clients = HashSet::new();
            for (&id, client) in clients.iter_mut() {
              if let Err(err) = client.write_all(&data).await {
                println!("write to client failed, err = {}", err);
                invalid_clients.insert(id);
              }
            }
            for client_id in invalid_clients {
              clients.remove(&client_id);
            }
          }
          None => break,
      },
      client = rx_new_client.next().fuse() => match client {
          Some(client) => {
            println!("new client = {:?}", client);
            uniq_client_id += 1;
            clients.insert(uniq_client_id, client);
          },
          None => break,
      }
    }
  }
  Ok(())
}

async fn server(tx: Sender<TcpStream>) {
  if let Err(err) = server_inner(tx).await {
    println!("err = {}", err);
  }
}

async fn server_inner(mut tx: Sender<TcpStream>) -> Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080").await?;
  println!("Listening on {}", listener.local_addr()?);

  let mut incoming = listener.incoming();
  while let Some(stream) = incoming.next().await {
    let stream = stream?;
    tx.send(stream).await.unwrap();
  }

  Ok(())
}