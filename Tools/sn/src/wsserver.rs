use crate::exec::ExecInput;
use crate::utils;
use actix::prelude::*;
use actix_files::NamedFile;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use std::io::Write;
use std::path::PathBuf;
use std::pin::Pin;
use std::process::Stdio;
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command};
use std::time::{Duration, Instant};
use tokio_stream::{Stream, StreamExt, StreamMap};
use tokio::io::AsyncReadExt;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn view() -> impl Responder {
    NamedFile::open_async("./static/ws.html").await.unwrap()
}

/// WebSocket handshake and start `MyWebSocket` actor.
pub async fn echo_ws(
    input: web::Query<ExecInput>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::new(input.into_inner()), &req, stream)
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    input: ExecInput,
    child: Child,
    child_stdin: ChildStdin,
    child_stdout: Option<ChildStdout>,
    child_stderr: Option<ChildStderr>,
}

impl MyWebSocket {
    pub fn new(input: ExecInput) -> Self {
        let mut child = Command::new(utils::guess_shell())
            .arg("-c")
            .arg(&input.cmd)
            .current_dir(
                input
                    .working_directory
                    .as_ref()
                    .map_or(std::env::current_dir().unwrap(), |working_directory| {
                        PathBuf::from(&working_directory)
                    }),
            )
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("ws command failed to start");

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        Self {
            hb: Instant::now(),
            input,
            child,
            child_stdin: stdin,
            child_stdout: Some(stdout),
            child_stderr: Some(stderr),
        }
    }

    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                log::info!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

    fn exec(&mut self, ctx: &mut <Self as Actor>::Context) {
        let mut stdout: tokio::process::ChildStdout =
            tokio::process::ChildStdout::from_std(self.child_stdout.take().unwrap()).unwrap();
        let mut stderr: tokio::process::ChildStderr =
            tokio::process::ChildStderr::from_std(self.child_stderr.take().unwrap()).unwrap();

        tokio::spawn(async move {
            let rx1 = Box::pin(async_stream::stream! {
                loop {
                    let mut buf = [0; 1024];
                    let n = stdout.read(&mut buf).await.unwrap();
                    yield buf[..n].to_vec();
                }
            }) as Pin<Box<dyn Stream<Item = Vec<u8>> + Send>>;

            let rx2 = Box::pin(async_stream::stream! {
                loop {
                    let mut buf = [0; 1024];
                    let n = stderr.read(&mut buf).await.unwrap();
                    yield buf[..n].to_vec();
                }
            }) as Pin<Box<dyn Stream<Item = Vec<u8>> + Send>>;

            let mut map = StreamMap::new();

            // Insert both streams
            map.insert("child_stdout", rx1);
            map.insert("child_stderr", rx2);

            loop {
                let (key, val) = map.next().await.unwrap();
                if key == "child_stdout" {
                    //ctx.binary(val)
                } else if key == "child_stderr" {
                    //ctx.binary(val)
                }
            }
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        self.exec(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // process websocket messages
        log::info!("WS: {msg:?}");
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => {
                //ctx.binary(bin)
                self.child_stdin.write_all(&bin).unwrap();
                self.child_stdin.flush().unwrap();
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

// https://doc.rust-lang.org/std/process/struct.Stdio.html
// https://doc.rust-lang.org/std/process/struct.Child.html
// https://doc.rust-lang.org/std/process/struct.ChildStdout.html
// https://docs.rs/async-stream/latest/async_stream/
// https://docs.rs/tokio-stream/0.1.10/tokio_stream/
// https://docs.rs/tokio-stream/0.1.10/tokio_stream/struct.StreamMap.html
