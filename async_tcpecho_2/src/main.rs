use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;

struct Server {
  listener: TcpListener,
}

impl Server {
  pub fn new(listener: TcpListener) -> Server {
    Server { listener }
  }

  pub async fn run(&mut self) -> io::Result<()> {
    let mut incoming = self.listener.incoming();
    while let Some(stream) = incoming.next().await {
      let stream = stream?;

      let conn = Connection::new(stream);

      task::spawn(async {
        conn.run().await.unwrap();
      });
    }

    Ok(())
  }
}

struct Connection {
  socket: TcpStream,
}

impl Connection {
  pub fn new (socket: TcpStream) -> Connection {
    Connection { socket }
  }

  pub async fn run(self) -> io::Result<()> {
    println!("Accepted from: {}", self.socket.peer_addr()?);

    let mut reader = self.socket.clone();
    let mut writer = self.socket;
    io::copy(&mut reader, &mut writer).await?;
    Ok(())
  }
}

async fn run() -> io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:8080").await?;
  println!("Listening on {}", listener.local_addr()?);

  let mut server = Server::new(listener);
  server.run().await?;

  Ok(())
}

fn main() -> io::Result<()> {
  task::block_on(async {
    run().await?;
    Ok(())
  })
}
