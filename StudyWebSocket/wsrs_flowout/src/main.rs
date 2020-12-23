extern crate chrono;
extern crate structopt;
extern crate tar;
extern crate ws;

use std::cmp::min;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use chrono::{TimeZone, Utc};
use structopt::StructOpt;
use tar::{Builder, Header};
use ws::{connect, CloseCode, Handler, Handshake, Message, Result, Sender};

use rust_msgpack::decode;
use rust_msgpack::encode;
use value::from_value::FromValue;
use value::into_value::IntoValue;
use value_derive::*;

use flowout::*;

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
    sub_topics: Vec<&'a str>,
    text_topics: Vec<&'a str>,
    msgpack_topics: Vec<&'a str>,
    save_topics: Vec<&'a str>,
    message_counter: i32,
    has_made_save_dir: bool,
    tar_builder: Option<Builder<File>>,
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

        if utils::utils::is_string_wildcard_match_array(&msg.topic, &self.text_topics) {
            self.show_text_message(&msg);
        } else if utils::utils::is_string_wildcard_match_array(&msg.topic, &self.msgpack_topics) {
            self.show_msgpack_message(&msg);
        } else {
            self.show_binary_message(&msg);
        }

        if utils::utils::is_string_wildcard_match_array(&msg.topic, &self.save_topics) {
            self.save_message(&msg);
        }
    }

    fn show_text_message(&mut self, msg: &TMessage) {
        println!("{}", String::from_utf8(msg.data.clone()).unwrap());
    }

    fn show_msgpack_message(&mut self, msg: &TMessage) {
        let v = decode::decode_to_value(&msg.data).unwrap();
        println!("{}", v);
    }

    fn show_binary_message(&mut self, msg: &TMessage) {
        let total_size = msg.data.len();
        let slice_size = min(total_size, 1024);
        let result = hex::hexcode::hexdump(&msg.data[..slice_size]);
        println!("{}", result);
    }

    fn save_message(&mut self, msg: &TMessage) {
        let filename = format!("flowout.{}-{}-{}.dat", msg.time, msg.source, msg.topic);

        if self.opt.save_dir.len() != 0 {
            self.save_message_to_dir(msg, &filename);
        }

        if self.opt.save_tar.len() != 0 {
            self.save_message_to_tar(msg, &filename);
        }
    }

    fn save_message_to_dir(&mut self, msg: &TMessage, filename: &str) {
        if !self.has_made_save_dir {
            fs::create_dir_all(&self.opt.save_dir).unwrap();
        }

        let filenamewithpath = Path::new(&self.opt.save_dir).join(filename);
        let mut file = File::create(filenamewithpath).unwrap();
        file.write_all(&msg.data).unwrap();
    }

    fn save_message_to_tar(&mut self, msg: &TMessage, filename: &str) {
        match &mut self.tar_builder {
            Some(ar) => {
                let mut header = Header::new_gnu();
                header.set_size(msg.data.len() as u64);
                header.set_cksum();
                header.set_mode(0o644);
                ar.append_data(&mut header, filename, &msg.data[..])
                    .unwrap();
            }
            None => {
                let file = File::create(&self.opt.save_tar).unwrap();
                let ar = Builder::new(file);
                self.tar_builder = Some(ar);
            }
        }
    }
}

impl<'a> Handler for Client<'a> {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!(
            "connecting to {} success, send subscribe to server..",
            self.opt.server
        );

        for &topic in self.sub_topics.iter() {
            let mut sub = HashMap::new();
            sub.insert("source", &self.opt.name[..]);
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
    /// specify the server
    #[structopt(short = "s", long = "server", default_value = "127.0.0.1:24012")]
    server: String,

    /// specify the topics to subscribe
    #[structopt(long = "sub", default_value = "*")]
    sub: String,

    /// specify the topics to print as text
    #[structopt(long = "text", default_value = "")]
    text: String,

    /// specify the topics to print as msgpack
    #[structopt(long = "msgpack", default_value = "")]
    msgpack: String,

    /// specify the topics to save
    #[structopt(long = "save", default_value = "")]
    save: String,

    /// specify where to save the messages
    #[structopt(long = "save-dir", default_value = "")]
    save_dir: String,

    /// save the messages into a tar file
    #[structopt(long = "save-tar", default_value = "")]
    save_tar: String,

    /// specify how many messages to recv
    #[structopt(short = "l", long = "limit", default_value = "0")]
    limit: i32,

    /// specify the client's name
    #[structopt(short = "n", long = "name", default_value = "flowout")]
    name: String,
}

fn main() {
    let opt = Opt::from_args();
    let serveraddr = String::from("ws://") + &opt.server;
    connect(serveraddr, |out| Client {
        out: out,
        opt: &opt,
        sub_topics: opt.sub.split(',').collect(),
        text_topics: opt.text.split(',').collect(),
        msgpack_topics: opt.msgpack.split(',').collect(),
        save_topics: opt.save.split(',').collect(),
        message_counter: 0,
        has_made_save_dir: false,
        tar_builder: None,
    })
    .unwrap();
}
