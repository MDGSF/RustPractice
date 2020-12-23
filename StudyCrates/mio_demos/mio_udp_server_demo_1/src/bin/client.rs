use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};
use std::error::Error;
use std::time::Duration;

const MTU: usize = 1400;
const SERVER_ADDR: &'static str = "127.0.0.1:5000";

fn main() -> Result<(), Box<dyn Error>> {
  const TOKEN_CLIENT: Token = Token(0);
  const TOKEN_CLIENT_2: Token = Token(1);

  let mut client_socket = UdpSocket::bind("127.0.0.1:0".parse()?)?;
  let mut client_socket_2 = UdpSocket::bind("127.0.0.1:0".parse()?)?;

  client_socket.connect(SERVER_ADDR.parse()?)?;
  client_socket_2.connect(SERVER_ADDR.parse()?)?;

  let empty_register = [0; 1];
  client_socket.send(&empty_register)?;
  client_socket_2.send(&empty_register)?;

  let mut poll = Poll::new()?;

  poll
    .registry()
    .register(&mut client_socket, TOKEN_CLIENT, Interest::READABLE)?;

  poll
    .registry()
    .register(&mut client_socket_2, TOKEN_CLIENT_2, Interest::READABLE)?;

  let mut events = Events::with_capacity(128);
  loop {
    poll.poll(&mut events, Some(Duration::from_millis(100)))?;
    for event in events.iter() {
      match event.token() {
        TOKEN_CLIENT => {
          if event.is_readable() {
            let mut buffer = [0; MTU];
            let bytes_recved = client_socket.recv(&mut buffer)?;
            println!("client1 {:?}", &buffer[..bytes_recved]);
          }
        }
        TOKEN_CLIENT_2 => {
          if event.is_readable() {
            let mut buffer = [0; MTU];
            let bytes_recved = client_socket_2.recv(&mut buffer)?;
            println!("client2 {:?}", &buffer[..bytes_recved]);
          }
        }
        _ => unreachable!(),
      }
    }
  }
}
