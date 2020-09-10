// sudo route add default gw 192.168.0.233

use socket2::{Domain, Socket, Type};
use std::error::Error;
use std::net::SocketAddr;

fn main() -> Result<(), Box<dyn Error>> {
  let socket = Socket::new(Domain::ipv4(), Type::dgram(), None).expect("socket new failed");
  socket
    .set_reuse_address(true)
    .expect("set reuse address failed");
  socket.set_reuse_port(true).expect("set reuse port failed");
  socket
    .bind(&"0.0.0.0:12345".parse::<SocketAddr>().unwrap().into())
    .expect("bind failed");
  socket
    .set_multicast_ttl_v4(255)
    .expect("set multicast failed");
  let mut count: u32 = 0;
  loop {
    socket
      .send_to(
        &count.to_ne_bytes(),
        &"224.168.2.9:1600".parse::<SocketAddr>().unwrap().into(),
      )
      .expect("send to failed");
    count += 1;
    std::thread::sleep(std::time::Duration::from_millis(50));
  }
}
