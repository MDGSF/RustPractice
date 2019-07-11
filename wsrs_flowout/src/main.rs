#![allow(dead_code)]
#![allow(unused_imports)]
extern crate ws;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate rmp_serialize;
extern crate rmpv;
extern crate rustc_serialize;

use rmp_serialize::{Decoder, Encoder};
use rmps::{Deserializer, Serializer};
use rustc_serialize::{Decodable, Encodable};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use ws::{connect, CloseCode, Handler, Handshake, Message, Result, Sender};

struct Client {
    out: Sender,
}

impl Handler for Client {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        if let Some(addr) = shake.remote_addr()? {
            println!(
                "Connection with {} now open, send subscribe to server..",
                addr
            );
        }

        let sub = TSubscribe {
            source: "wsrs_client".to_string(),
            topic: "subscribe".to_string(),
            data: "aaa".to_string(),
        };

        let mut buf = Vec::new();
        sub.serialize(&mut Serializer::new(&mut buf)).unwrap();

        self.out.send(buf)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Got message: {}", msg);
        Ok(())
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct TSubscribe {
    source: String,

    topic: String,

    data: String,
}

fn main() {
    connect("ws://127.0.0.1:24012", |out| Client { out: out }).unwrap()
}
