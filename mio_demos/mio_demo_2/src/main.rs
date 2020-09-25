extern crate mio;

use mio::net::{TcpListener, TcpStream};
use mio::*;

fn main() {
    const SERVER: Token = Token(0);
    const CLIENT: Token = Token(1);

    let addr = "127.0.0.1:13265".parse().unwrap();
    let server = TcpListener::bind(&addr).unwrap();

    let poll = Poll::new().unwrap();
    poll.register(&server, SERVER, Ready::readable(), PollOpt::edge())
        .unwrap();

    let sock = TcpStream::connect(&addr).unwrap();
    poll.register(&sock, CLIENT, Ready::readable(), PollOpt::edge())
        .unwrap();

    let mut events = Events::with_capacity(1024);

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                SERVER => {
                    let _ = server.accept();
                }
                CLIENT => {
                    return;
                }
                _ => unreachable!(),
            }
        }
    }
}
