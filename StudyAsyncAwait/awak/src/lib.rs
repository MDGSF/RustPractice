#[macro_export]
macro_rules! pin_mut {
  ($($x:ident),* $(,)?) => { $(
    let mut $x = $x;
    #[allow(unused_mut)]
    let mut $x = unsafe {
      std::pin::Pin::new_unchecked(&mut $x)
    };
  )* }
}

#[macro_export]
macro_rules! ready {
  ($e:expr $(,)?) => {
    match $e {
      std::task::Poll::Ready(t) => t,
      std::task::Poll::Pending => return core::task::Poll::Pending,
    }
  };
}

pub mod blocking;
pub mod executor;
pub mod io;
pub mod net;
mod parking;
pub mod time;
mod waker_fn;

use std::future::Future;
use std::thread;

pub use async_task::Task;
pub use blocking::block_on;
pub use executor::Executor;

pub use futures_util::stream::StreamExt;

use once_cell::sync::Lazy;

pub static EXECUTOR: Lazy<Executor> = Lazy::new(||{
  for _ in 0..num_cpus::get().max(1) {
    thread::spawn(move || {
      let ticker = EXECUTOR.ticker();
      block_on(ticker.run());
    });
  }

  Executor::new()
});

pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Task<T> {
  EXECUTOR.spawn(future)
}