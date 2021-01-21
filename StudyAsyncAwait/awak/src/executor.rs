use std::future::Future;
use std::panic::UnwindSafe;
use std::pin::Pin;
use std::sync::{
  atomic::{AtomicBool, AtomicUsize, Ordering},
  Arc,
};
use std::task::{Context, Poll, Waker};

use async_task::{Runnable, Task};
use concurrent_queue::ConcurrentQueue;
use futures_util::future::poll_fn;
use parking_lot::{Mutex, RwLock};

use rand::Rng;

/// A multi-threaded executor.
#[derive(Debug)]
pub struct Executor {
  global: Arc<Global>,
}

impl Executor {}

impl Default for Executor {
  fn default() -> Executor {
    Executor::new()
  }
}

#[derive(Debug)]
struct Global {
  queue: ConcurrentQueue<Runnable>,
  notified: AtomicBool,
  shards: RwLock<Vec<Arc<ConcurrentQueue<Runnable>>>>,
  sleepers: Mutex<Sleepers>,
}

impl Global {
  fn notify(&self) {
    if self
      .notified
      .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
      .is_ok()
    {
      let waker = self.sleepers.lock().notify();
      if let Some(waker) = waker {
        waker.wake();
      }
    }
  }
}

/// A list of sleeping tickers.
#[derive(Debug)]
struct Sleepers {
  count: usize,
  wakers: Vec<(usize, Waker)>,
  id_gen: usize,
}

impl Sleepers {
  fn notify(&mut self) -> Option<Waker> {
    if self.wakers.len() == self.count {
      self.wakers.pop().map(|item| item.1)
    } else {
      None
    }
  }

  fn update(&mut self, id: usize, waker: &Waker) -> bool {
    for item in &mut self.wakers {
      if item.0 == id {
        if !item.1.will_wake(waker) {
          item.1 = waker.clone();
        }
        return false;
      }
    }

    self.wakers.push((id, waker.clone()));
    true
  }

  fn is_notified(&self) -> bool {
    self.count == 0 || self.count > self.wakers.len()
  }

  fn insert(&mut self, waker: &Waker) -> usize {
    let id = self.id_gen;
    self.id_gen += 1;
    self.count += 1;
    self.wakers.push((id, waker.clone()));
    id
  }

  fn remove(&mut self, id: usize) {
    self.count -= 1;
    for i in (0..self.wakers.len()).rev() {
      if self.wakers[i].0 == id {
        self.wakers.remove(i);
        return;
      }
    }
  }
}

/// Runs tasks in a multi-threaded executor.
#[derive(Debug)]
pub struct Ticker {
  global: Arc<Global>,
  shard: Arc<ConcurrentQueue<Runnable>>,
  sleeping: AtomicUsize,
  ticks: AtomicUsize,
}

impl UnwindSafe for Ticker {}

#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
struct YieldNow(bool);

fn yield_now() -> YieldNow {
  YieldNow(false)
}

impl Future for YieldNow {
  type Output = ();

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    if !self.0 {
      self.0 = true;
      cx.waker().wake_by_ref();
      Poll::Pending
    } else {
      Poll::Ready(())
    }
  }
}

fn steal<T>(src: &ConcurrentQueue<T>, dest: &ConcurrentQueue<T>) {
  let mut count = (src.len() + 1) / 2;
  if count <= 0 {
    return;
  }

  if let Some(cap) = dest.capacity() {
    count = count.min(cap - dest.len());
  }

  for _ in 0..count {
    if let Ok(t) = src.pop() {
      assert!(dest.push(t).is_ok());
    } else {
      break;
    }
  }
}
