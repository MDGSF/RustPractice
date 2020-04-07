use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::str::from_utf8;

const SERVER: Token = Token(0);

const DATA: &[u8] = b"Hello world!\n";

fn main() -> io::Result<()> {

  let mut poll = Poll::new()?;

  let mut events = Events::with_capacity(1024);

  let addr = "127.0.0.1:9000".parse().unwrap();
  let mut server = TcpListener::bind(addr)?;

  poll.registry().register(&mut server, SERVER, Interest::READABLE)?;

  // map<Token, TcpStream>
  let mut connections = HashMap::new();

  let mut uniq_token = Token(SERVER.0 + 1);

  println!("Server Listen at: 127.0.0.1:9000");
  println!("You can use `nc 127.0.0.1 9000` to connect to server.");

  loop {
    poll.poll(&mut events, None)?;

    for event in events.iter() {
      match event.token() {
        SERVER => {
          let (mut connection, address) = server.accept()?;
          println!("Accepted connection from: {}", address);

          let token = next(&mut uniq_token);

          poll.registry().register(
            &mut connection,
            token,
            Interest::READABLE.add(Interest::WRITABLE))?;

          connections.insert(token, connection);
        }
        token => {
          let done = if let Some(connection) = connections.get_mut(&token) {
            handle_connection_event(poll.registry(), connection, event)?
          } else {
            false
          };
          if done {
            connections.remove(&token);
          }
        }
      }
    }
  }
}

fn next(current: &mut Token) -> Token {
  let next = current.0;
  current.0 += 1;
  Token(next)
}

fn handle_connection_event(
  registry: &Registry,
  connection: &mut TcpStream,
  event: &Event,
) -> io::Result<bool> {

  if event.is_writable() {
    // We can (maybe) write to the connection.
    match connection.write(DATA) {
      // We want to write the entire `DATA` buffer in a single go. If we
      // write less we'll return a short write error.
      Ok(n) if n < DATA.len() => {
        return Err(io::ErrorKind::WriteZero.into());
      },

      // After we've written something we'll reregister the connection
      // to only respond to readable events.
      Ok(_) => {
        registry.reregister(connection, event.token(), Interest::READABLE)?;
      }

      // Would block "errors" are the OS's way of saying that the connection
      // is not actually ready to perform this I/O operation.
      Err(ref err) if would_block(err) => {}

      // Got interrupted (how rude!), we'll try again.
      Err(ref err) if interrupted(err) => {
        return handle_connection_event(registry, connection, event);
      }

      // Other errors we'll consider fatal.
      Err(err) => return Err(err),
    }
  }

  if event.is_readable() {
    let mut connection_closed = false;
    let mut received_data = Vec::with_capacity(4096);

    loop {
      let mut buf = [0; 256];
      match connection.read(&mut buf) {
        Ok(0) => {
          // Reading 0 bytes means the other side has closed the
          // connection or is done writing, then so are we.
          connection_closed = true;
          break;
        }
        Ok(n) => received_data.extend_from_slice(&buf[..n]),
        Err(ref err) if would_block(err) => break,
        Err(ref err) if interrupted(err) => continue,
        Err(err) => return Err(err),
      }
    }

    if let Ok(str_buf) = from_utf8(&received_data) {
      println!("Received data: {}", str_buf.trim_end());
    } else {
      println!("Received (non UTF-8) data: {:?}", &received_data);
    }

    if connection_closed {
      println!("Connection closed");
      return Ok(true);
    }
  }

  Ok(false)
}

fn would_block(err: &io::Error) -> bool {
  err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
  err.kind() == io::ErrorKind::Interrupted
}
