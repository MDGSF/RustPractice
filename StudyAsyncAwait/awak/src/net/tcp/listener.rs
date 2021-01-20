use std::future::Future;
use std::io;
use std::net::{self, SocketAddr, ToSocketAddrs};
use std::pin::Pin;
use std::task::{Context, Poll};

use super::stream::TcpStream;
use crate::io::Async;

use futures_util::stream::Stream;

pub struct TcpListener {}
