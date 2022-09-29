use std::{io, thread};

use actix_web::web::Bytes;
use awc::ws;
use futures_util::{SinkExt as _, StreamExt as _};
use tokio::{select, sync::mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting echo WebSocket client");

    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let mut cmd_rx = UnboundedReceiverStream::new(cmd_rx);

    // run blocking terminal input reader on separate thread
    let _input_thread = thread::spawn(move || loop {
        let mut cmd = String::with_capacity(32);

        if io::stdin().read_line(&mut cmd).is_err() {
            log::error!("error reading line");
            return;
        }

        cmd_tx.send(cmd).unwrap();
    });

    let (hb_tx, hb_rx) = mpsc::unbounded_channel();
    let mut hb_rx = UnboundedReceiverStream::new(hb_rx);

    let _hb_thread = thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        hb_tx.send(()).unwrap();
    });

    let (res, mut ws) = awc::Client::new()
        .ws("ws://127.0.0.1:8080/ws?cmd=/tmp/echo_path.sh")
        .connect()
        .await
        .unwrap();

    log::debug!("response: {res:?}");
    log::info!("connected; server will echo messages sent");

    'outer: loop {
        select! {
            Some(msg) = ws.next() => {
                match msg {
                    Ok(ws::Frame::Text(txt)) => {
                        // log echoed messages from server
                        log::info!("Server: {txt:?}")
                    }

                    Ok(ws::Frame::Binary(txt)) => {
                        // log echoed messages from server
                        log::info!("Server: {txt:?}")
                    }

                    Ok(ws::Frame::Ping(_)) => {
                        // respond to ping probes
                        let ret = ws.send(ws::Message::Pong(Bytes::new())).await;
                        if ret.is_err() {
                            log::info!("ws send failed: {:?}", ret);
                            break 'outer;
                        }
                    }

                    Ok(ws::Frame::Pong(_)) => {
                        // do nothing
                    }

                    Ok(ws::Frame::Close(reason)) => {
                        log::info!("Server: {reason:?}");
                        break 'outer;
                    }

                    _ => {
                        log::info!("Server: {msg:?}");
                        break 'outer;
                    }
                }
            }

            Some(cmd) = cmd_rx.next() => {
                if cmd.is_empty() {
                    continue;
                }

                let ret = ws.send(ws::Message::Text(cmd.into())).await;
                if ret.is_err() {
                    log::info!("ws send failed: {:?}", ret);
                    break 'outer;
                }
            }

            Some(_) = hb_rx.next() => {
                let ret = ws.send(ws::Message::Ping(Bytes::new())).await;
                if ret.is_err() {
                    log::info!("ws send failed: {:?}", ret);
                    break 'outer;
                }
            }

            else => break 'outer
        }
    }

    //input_thread.join().unwrap();
    //hb_thread.join().unwrap();
}
