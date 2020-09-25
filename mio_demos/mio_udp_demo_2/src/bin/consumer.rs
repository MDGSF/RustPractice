use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};
use std::error::Error;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
  const ECHOER: Token = Token(1);
  let mut echoer_socket = UdpSocket::bind("127.0.0.1:5000".parse()?)?;
  let mut poll = Poll::new()?;
  poll
    .registry()
    .register(&mut echoer_socket, ECHOER, Interest::READABLE)?;

  let start = Instant::now();

  let mut exit = false;
  let mut count = 0;
  let mut buffer = [0; 1];
  let mut events = Events::with_capacity(128);
  while !exit {
    poll.poll(&mut events, Some(Duration::from_millis(100)))?;
    for event in events.iter() {
      match event.token() {
        ECHOER => {
          echoer_socket.recv_from(&mut buffer)?;
          count += 1;
          if count % 1000 == 0 {
            println!("count = {}, {:?}", count, start.elapsed());
          }
          if count > 10000 {
            exit = true;
            break;
          }
        }
        _ => unreachable!(),
      }
    }
  }

  println!("{:?}", start.elapsed());
  Ok(())
}
