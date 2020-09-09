use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};
use std::collections::HashSet;
use std::error::Error;
use std::sync::mpsc::sync_channel;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const MTU: usize = 1400;

fn main() -> Result<(), Box<dyn Error>> {
  let clients = Arc::new(Mutex::new(HashSet::new()));
  let clients_1 = Arc::clone(&clients);

  const TOKEN_SERVER: Token = Token(0);

  let server_socket = Arc::new(Mutex::new(UdpSocket::bind("127.0.0.1:5000".parse()?)?));
  let server_socket_1 = Arc::clone(&server_socket);

  let mut poll = Poll::new()?;

  let (tx, rx) = sync_channel::<i32>(10);

  poll.registry().register(
    &mut *server_socket.lock().unwrap(),
    TOKEN_SERVER,
    Interest::READABLE | Interest::WRITABLE,
  )?;

  thread::spawn(move || {
    let mut i = 0;
    loop {
      tx.send(i).unwrap();
      i += 1;
      println!("send i = {}", i);

      thread::sleep(Duration::from_millis(50));
    }
  });

  thread::spawn(move || loop {
    match rx.recv() {
      Ok(data) => {
        println!("data = {}", data);
        for &client in &*clients_1.lock().unwrap() {
          let msg_to_send = [9; 9];
          match server_socket_1
            .lock()
            .unwrap()
            .send_to(&msg_to_send, client)
          {
            Ok(bytes_send) => {
              assert_eq!(bytes_send, 9);
              println!("sent {:?} -> {:?} bytes", msg_to_send, bytes_send);
            }
            Err(err) => {
              println!("send to failed, err = {}", err);
            }
          }
        }
      }
      Err(err) => println!("try recv {}", err),
    }
  });

  let mut events = Events::with_capacity(128);
  loop {
    poll.poll(&mut events, Some(Duration::from_millis(100)))?;
    for event in events.iter() {
      match event.token() {
        TOKEN_SERVER => {
          if event.is_readable() {
            let mut buffer = [0; MTU];
            let (_bytes_recved, client_socket) =
              server_socket.lock().unwrap().recv_from(&mut buffer)?;
            clients.lock().unwrap().insert(client_socket);
            println!("new client: {:?}", client_socket);
          }
        }
        _ => unreachable!(),
      }
    }
  }
}
