use std::{
  future::Future,
  pin::Pin,
  task::{Context, Poll},
};

use tracing::info;

pub struct DumbFuture {}

impl Future for DumbFuture {
  type Output = ();

  fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
    info!("Hello from a dumb future!");
    Poll::Ready(())

    //panic!("Oh heck no");

    //cargo build && gdb --quiet --args ./target/debug/waytoodeep
    //unsafe {
    //  *(0xF00D as *mut u64) = 0x0;
    //}
    //unreachable!();
  }
}
