extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Cursor;
use tungstenite::{connect, Message};
use url::Url;

#[derive(Debug, Default, Deserialize)]
struct TMessage {
  source: String,
  topic: String,

  #[serde(with = "serde_bytes")]
  data: Vec<u8>,
  time: i64,
}

fn main() {
  env_logger::init();

  let (mut socket, response) =
    connect(Url::parse("ws://127.0.0.1:24012").unwrap()).expect("Can't connect");

  println!("Connected to the server");
  println!("Response HTTP code: {}", response.status());
  println!("Response contains the following headers:");
  for (ref header, _value) in response.headers() {
    println!("* {}", header);
  }

  let mut sub = HashMap::new();
  sub.insert("source", "flowdds");
  sub.insert("topic", "subscribe");
  sub.insert("data", "adas");
  let mut buf = Vec::new();
  sub.serialize(&mut Serializer::new(&mut buf)).unwrap();

  socket.write_message(Message::Binary(buf)).unwrap();

  loop {
    let msg = socket.read_message().expect("Error reading message");
    if let Message::Binary(data) = msg {
      let cur = Cursor::new(&data[..]);
      let mut de = Deserializer::new(cur);
      let out: TMessage = Deserialize::deserialize(&mut de).unwrap();

      println!("Received: {:?}", out);

      println!("{}", String::from_utf8(out.data).unwrap());

    } else {
      println!("{}", msg);
    }
  }
  // socket.close(None);
}
