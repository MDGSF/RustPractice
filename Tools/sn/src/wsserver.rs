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
use tokio::io::AsyncReadExt;
use tokio_stream::{Stream, StreamExt, StreamMap};

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
    log::info!("ws: {:?}", input);
    ws::start(MyWebSocket::new(input.into_inner()), &req, stream)
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
pub struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
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
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("ws command failed to start");

        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        Self {
            hb: Instant::now(),
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
        log::info!("exec start");
        let mut stdout: tokio::process::ChildStdout =
            tokio::process::ChildStdout::from_std(self.child_stdout.take().unwrap()).unwrap();
        let mut stderr: tokio::process::ChildStderr =
            tokio::process::ChildStderr::from_std(self.child_stderr.take().unwrap()).unwrap();
        let addr = ctx.address();

        tokio::spawn(async move {
            let child_stdout = Box::pin(async_stream::stream! {
                loop {
                    let mut buf = [0; 1024];
                    let n = stdout.read(&mut buf).await.unwrap();
                    if n == 0 {
                        break;
                    }
                    yield buf[..n].to_vec();
                }
            }) as Pin<Box<dyn Stream<Item = Vec<u8>> + Send>>;

            let child_stderr = Box::pin(async_stream::stream! {
                loop {
                    let mut buf = [0; 1024];
                    let n = stderr.read(&mut buf).await.unwrap();
                    if n == 0 {
                        break;
                    }
                    yield buf[..n].to_vec();
                }
            }) as Pin<Box<dyn Stream<Item = Vec<u8>> + Send>>;

            let mut map = StreamMap::new();
            map.insert("child_stdout", child_stdout);
            map.insert("child_stderr", child_stderr);

            loop {
                let msg = map.next().await;
                if msg.is_none() {
                    break;
                }

                let (key, val) = msg.unwrap();
                log::info!("exec loop, key = {}", key);
                log::info!("exec loop, val = {:?}", std::str::from_utf8(&val).unwrap());

                if val.len() == 0 {
                    break;
                }

                if key == "child_stdout" {
                    addr.do_send(ChildStdoutMsg(val));
                } else if key == "child_stderr" {
                    addr.do_send(ChildStderrMsg(val));
                }
            } // loop

            addr.do_send(ChildEnd);

            log::info!("child thread output finished");
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

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let _ = self.child.kill();
    }
}

struct ChildStdoutMsg(Vec<u8>);
impl Message for ChildStdoutMsg {
    type Result = ();
}

struct ChildStderrMsg(Vec<u8>);
impl Message for ChildStderrMsg {
    type Result = ();
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
struct ChildEnd;

impl Handler<ChildStdoutMsg> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: ChildStdoutMsg, ctx: &mut Self::Context) -> Self::Result {
        log::info!("handler stdout msg");
        ctx.binary(msg.0)
    }
}

impl Handler<ChildStderrMsg> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: ChildStderrMsg, ctx: &mut Self::Context) -> Self::Result {
        log::info!("handler stderr msg");
        ctx.binary(msg.0)
    }
}

impl Handler<ChildEnd> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, _: ChildEnd, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop()
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
            _ => {
                ctx.stop();
            }
        }
    }
}

// https://doc.rust-lang.org/std/process/struct.Stdio.html
// https://doc.rust-lang.org/std/process/struct.Child.html
// https://doc.rust-lang.org/std/process/struct.ChildStdout.html
// https://docs.rs/async-stream/latest/async_stream/
// https://docs.rs/tokio-stream/0.1.10/tokio_stream/
// https://docs.rs/tokio-stream/0.1.10/tokio_stream/struct.StreamMap.html
