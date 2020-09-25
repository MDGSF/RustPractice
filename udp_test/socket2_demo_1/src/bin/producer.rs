// sudo route add default gw 192.168.0.233

use socket2::{Domain, Socket, Type};
use std::env;
use std::error::Error;
use std::net::SocketAddr;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 3 {
    println!("usage: producer <bind> <multicast>");
    std::process::exit(0);
  }
  let bindaddr = &args[1];
  let multicastaddr = &args[2];

  let socket = Socket::new(Domain::ipv4(), Type::dgram(), None).expect("socket new failed");
  socket
    .set_reuse_address(true)
    .expect("set reuse address failed");
  socket.set_reuse_port(true).expect("set reuse port failed");
  socket
    .bind(&bindaddr.parse::<SocketAddr>().unwrap().into())
    .expect("bind failed");
  socket
    .set_multicast_ttl_v4(255)
    .expect("set multicast failed");
  let mut count: u32 = 0;
  loop {
    socket
      .send_to(
        &count.to_ne_bytes(),
        &multicastaddr.parse::<SocketAddr>().unwrap().into(),
      )
      .expect("send to failed");
    count += 1;
    std::thread::sleep(std::time::Duration::from_millis(50));
  }
}
