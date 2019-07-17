//#![allow(dead_code)]
//#![allow(unused_imports)]
//Serializer::with(wr, StructMapWriter)

extern crate ws;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
extern crate rmp_serialize;
extern crate rmpv;
extern crate rustc_serialize;

extern crate structopt;

extern crate chrono;

use std::collections::HashMap;

use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::io::Cursor;

use structopt::StructOpt;
use ws::{connect, CloseCode, Handler, Handshake, Message, Result, Sender};

use chrono::{TimeZone, Utc};

#[derive(Debug, Deserialize)]
struct TMessage {
    source: String,
    topic: String,
    data: Vec<u8>,
    time: i64,
}

struct Client<'a> {
    out: Sender,
    opt: &'a Opt,
    message_counter: i32,
}

impl<'a> Client<'a> {
    fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::Text(data) => println!("unexpected message: {}", data),
            Message::Binary(data) => {
                self.handle_binary_message(&data);
            }
        }
    }

    fn handle_binary_message(&mut self, data: &[u8]) {
        let cur = Cursor::new(&data[..]);
        let mut de = Deserializer::new(cur);
        let msg: TMessage = Deserialize::deserialize(&mut de).unwrap();

        let dt = Utc.timestamp(
            (msg.time / 1000000) as i64,
            (msg.time % 1000000 * 1000) as u32,
        );
        print!(
            "[{}] {:?} @{}: \"{}\" {}\n",
            self.message_counter,
            dt,
            msg.source,
            msg.topic,
            msg.data.len(),
        );
    }
}

impl<'a> Handler for Client<'a> {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!(
            "connecting to {} success, send subscribe to server..",
            self.opt.server
        );

        let subs = self.opt.sub.clone();
        let topics: Vec<&str> = subs.split(',').collect();
        for &topic in topics.iter() {
            let mut sub = HashMap::new();
            sub.insert("source", "flowout".to_string());
            sub.insert("topic", "subscribe".to_string());
            sub.insert("data", topic.to_string());

            println!("send subscribe topic {:?}", sub);

            let mut buf = Vec::new();
            let mut se = Serializer::new_named(&mut buf);
            sub.serialize(&mut se).unwrap();

            self.out.send(buf)?;
        }

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("Got message: {}", msg);

        self.message_counter += 1;

        self.handle_message(msg);

        if self.opt.limit > 0 && self.message_counter >= self.opt.limit {
            self.out.close(CloseCode::Normal)?;
        }

        Ok(())
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "flowout",
    about = "Used to subscribe data from libflow server."
)]
struct Opt {
    #[structopt(short = "s", long = "server", default_value = "127.0.0.1:24012")]
    server: String,

    #[structopt(short = "t", long = "sub", default_value = "*")]
    sub: String,

    #[structopt(short = "l", long = "limit", default_value = "0")]
    limit: i32,
}

fn main() {
    let opt = Opt::from_args();
    let serveraddr = String::from("ws://") + &opt.server;
    connect(serveraddr, |out| Client {
        out: out,
        opt: &opt,
        message_counter: 0,
    })
    .unwrap();
}
