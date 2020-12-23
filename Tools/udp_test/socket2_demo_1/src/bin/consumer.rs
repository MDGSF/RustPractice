use socket2::{Domain, Socket, Type};
use std::env;
use std::error::Error;
use std::net::SocketAddr;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    println!("usage: consumer <bind> <multicast>");
    std::process::exit(0);
  }
  let bindaddr = &args[1];

  let socket = Socket::new(Domain::ipv4(), Type::dgram(), None).unwrap();
  socket.set_reuse_address(true).unwrap();
  socket.set_reuse_port(true).unwrap();
  socket
    .bind(&bindaddr.parse::<SocketAddr>().unwrap().into())
    .unwrap();
  socket.set_multicast_ttl_v4(255).unwrap();

  for i in 2..args.len() {
    let multicastaddr = &args[i];
    socket.join_multicast_v4(&multicastaddr.parse()?, &"0.0.0.0".parse()?)?;
  }

  let mut buffer = [0; 4];
  loop {
    let (recved, peer) = socket.recv_from(&mut buffer)?;
    println!("{:?}: {} {}", peer, recved, u32::from_ne_bytes(buffer));
  }
}
