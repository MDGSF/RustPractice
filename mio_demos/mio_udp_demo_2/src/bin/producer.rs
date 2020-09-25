use mio::net::UdpSocket;
use std::error::Error;
use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>> {
  let mut count: u32 = 0;
  let sender_socket = UdpSocket::bind("127.0.0.1:0".parse()?)?;
  loop {
    sender_socket.send_to(&count.to_ne_bytes(), "127.0.0.1:5000".parse()?)?;
    count += 1;
    thread::sleep(time::Duration::from_millis(50));
  }
}
