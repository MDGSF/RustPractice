// An Echo program:
// Sender -> sends a message.
// Echoer -> listens and prints the message received.

use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
  const SENDER: Token = Token(0);
  const ECHOER: Token = Token(1);

  let mut sender_socket = UdpSocket::bind("127.0.0.1:0".parse()?)?;
  let mut echoer_socket = UdpSocket::bind("127.0.0.1:0".parse()?)?;

  // If we do not use connect here, SENDER and ECHOER would need to call
  // send_to and recv_from respectively
  sender_socket.connect(echoer_socket.local_addr()?)?;

  // We need a Poll to check if SENDER is ready to to written into,
  // and if ECHOER is ready to be read from.
  let mut poll = Poll::new()?;

  // We register our sockets here so that we can check if they are
  // ready to be written/read.
  poll
    .registry()
    .register(&mut sender_socket, SENDER, Interest::WRITABLE)?;
  poll
    .registry()
    .register(&mut echoer_socket, ECHOER, Interest::READABLE)?;

  let mut events = Events::with_capacity(128);
  loop {
    poll.poll(&mut events, Some(Duration::from_millis(100)))?;
    for event in events.iter() {
      match event.token() {
        SENDER => {
          let msg_to_send = [9; 9];
          let bytes_send = sender_socket.send(&msg_to_send)?;
          assert_eq!(bytes_send, 9);
          println!("sent {:?} -> {:?} bytes", msg_to_send, bytes_send);
        }
        ECHOER => {
          let mut buffer = [0; 9];
          let num_recv = echoer_socket.recv(&mut buffer)?;
          println!("echo {:?} -> {:?}", buffer, num_recv);
        }
        _ => unreachable!(),
      }
    }
  }
}
