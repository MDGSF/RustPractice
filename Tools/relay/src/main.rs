use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use std::sync::Arc;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Default)]
#[structopt(name = "relay")]
struct Opt {
  /// Specify the server
  #[structopt(short = "s", long = "server", default_value = "47.107.130.67:16328")]
  server: String,

  /// Specify the local
  #[structopt(short = "l", long = "local", default_value = "192.168.42.2:16328")]
  local: String,
}

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
  let opt = Arc::new(Opt::from_args());

  let listener = TcpListener::bind(&opt.local).await?;
  println!("Listening on {}", listener.local_addr()?);

  let mut incoming = listener.incoming();
  while let Some(stream) = incoming.next().await {
    let stream = stream?;
    let cur_opt = Arc::clone(&opt);
    task::spawn(async move {
      process(stream, cur_opt).await.unwrap();
    });
  }
  Ok(())
}

async fn process(front: TcpStream, opt: Arc<Opt>) -> io::Result<()> {
  println!("Accepted from: {}", front.peer_addr()?);

  let backen = TcpStream::connect(&opt.server).await?;
  println!("Connected to remote server: {}", &backen.peer_addr()?);

  let mut front_reader = front.clone();
  let mut front_writer = front;

  let mut backen_reader = backen.clone();
  let mut backen_writer = backen;

  let a = io::copy(&mut front_reader, &mut backen_writer);
  let b = io::copy(&mut backen_reader, &mut front_writer);

  let (_, _) = a.join(b).await;
  Ok(())
}
