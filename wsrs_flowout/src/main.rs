//#![allow(dead_code)]
//#![allow(unused_imports)]
//Serializer::with(wr, StructMapWriter)

extern crate chrono;
extern crate structopt;
extern crate ws;

use chrono::{TimeZone, Utc};
use std::cmp::min;
use std::collections::HashMap;
use structopt::StructOpt;
use ws::{connect, CloseCode, Handler, Handshake, Message, Result, Sender};

use rust_msgpack::binary;
use rust_msgpack::decode;
use rust_msgpack::encode;
use value::from_value::FromValue;
use value::into_value::IntoValue;
use value_derive::*;

#[derive(Debug, Default, FromValue, IntoValue)]
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
        //println!("data = {:x?}", data);

        let v = decode::decode_to_value(&data).unwrap();
        let msg: TMessage = v.from_value();

        let ts = Utc.timestamp(
            (msg.time / 1000000) as i64,
            (msg.time % 1000000 * 1000) as u32,
        );
        print!(
            "[{}] {:?} @{}: \"{}\" {}\n",
            self.message_counter,
            ts,
            msg.source,
            msg.topic,
            msg.data.len(),
        );

        self.show_binary_message(&msg);
    }

    fn show_binary_message(&mut self, msg: &TMessage) {
        let total_size = msg.data.len();
        let slice_size = min(total_size, 1024);
        let result = hexdump(&msg.data[..slice_size]);
        println!("{}", result);
    }
}

fn hexdump(data: &[u8]) -> String {
    let mut result: Vec<u8> = Vec::new();

    let mut buf1: Vec<u8> = vec![0; 14];
    let mut buf2: Vec<u8> = vec![0; 14];
    let mut right_chars: Vec<u8> = vec![0; 18];
    let mut used = 0;
    let mut n = 0;
    for (i, c) in data.iter().enumerate() {
        if used == 0 {
            binary::BigEndian::put_uint32(&mut buf1[0..4], n);
            hexencode(&mut buf2[0..8], &buf1[0..4]);
            buf2[8] = b' ';
            buf2[9] = b' ';
            result.extend(&buf2[0..10]);
        }
        hexencode(&mut buf2[..], &data[i..i + 1]);
        buf2[2] = b' ';
        let mut l = 3;
        if used == 7 {
            buf2[3] = b' ';
            l = 4;
        } else if used == 15 {
            buf2[3] = b' ';
            buf2[4] = b'|';
            l = 5;
        }
        result.extend(&buf2[0..l]);

        right_chars[used] = tochar(*c);
        n += 1;
        used += 1;

        if used == 16 {
            right_chars[16] = b'|';
            right_chars[17] = b'\n';
            result.extend(&right_chars[..]);
            used = 0;
        }
    }

    String::from_utf8(result).unwrap()
}

fn hexencode(dst: &mut [u8], src: &[u8]) -> i32 {
    const HEXTABLE: &str = "0123456789abcdef";
    for (i, v) in src.iter().enumerate() {
        dst[i * 2] = HEXTABLE.as_bytes()[(v >> 4) as usize];
        dst[i * 2 + 1] = HEXTABLE.as_bytes()[(v & 0x0f) as usize];
    }
    (src.len() * 2) as i32
}

fn tochar(b: u8) -> u8 {
    if b < 32 || b > 126 {
        return b'.';
    }
    b
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
            sub.insert("source", "flowout");
            sub.insert("topic", "subscribe");
            sub.insert("data", topic);

            println!("send subscribe topic {:?}", sub);

            let bin = encode::encode(&sub).unwrap();
            self.out.send(bin)?;
        }

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
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
