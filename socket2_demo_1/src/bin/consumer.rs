use socket2::{Domain, Socket, Type};
use std::error::Error;
use std::net::SocketAddr;

fn main() -> Result<(), Box<dyn Error>> {
  let socket = Socket::new(Domain::ipv4(), Type::dgram(), None).unwrap();
  socket.set_reuse_address(true).unwrap();
  socket.set_reuse_port(true).unwrap();
  socket
    .bind(&"0.0.0.0:1600".parse::<SocketAddr>().unwrap().into())
    .unwrap();
  socket.set_multicast_ttl_v4(255).unwrap();

  socket.join_multicast_v4(&"224.168.2.9".parse()?, &"0.0.0.0".parse()?)?;

  let mut buffer = [0; 4];
  loop {
    let (recved, peer) = socket.recv_from(&mut buffer)?;
    println!("{:?}: {} {}", peer, recved, u32::from_ne_bytes(buffer));
  }
}
