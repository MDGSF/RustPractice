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

impl Executor {
  pub fn new() -> Executor {
    Executor {
      global: Arc::new(Global {
        queue: ConcurrentQueue::unbounded(),
        notified: AtomicBool::new(false),
        shards: RwLock::new(Vec::new()),
        sleepers: Mutex::new(Sleepers {
          count: 0,
          wakers: Vec::new(),
          id_gen: 1,
        }),
      }),
    }
  }

  pub fn spawn<T: Send + 'static>(
    &self,
    future: impl Future<Output = T> + Send + 'static,
  ) -> Task<T> {
    let global = self.global.clone();

    let schedule = move |runnable| {
      global.queue.push(runnable).unwrap();
      global.notify();
    };

    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();

    task
  }

  pub fn ticker(&self) -> Ticker {
    let ticker = Ticker {
      global: self.global.clone(),
      shard: Arc::new(ConcurrentQueue::bounded(512)),
      sleeping: AtomicUsize::new(0),
      ticks: AtomicUsize::new(0),
    };

    self.global.shards.write().push(ticker.shard.clone());

    ticker
  }
}

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

impl Ticker {
  pub async fn run(&self) {
    loop {
      for _ in 0..200 {
        let runnable = self.tick().await;
        runnable.run();
      }

      yield_now().await;
    }
  }

  async fn tick(&self) -> Runnable {
    poll_fn(|cx| match self.search() {
      None => {
        self.sleep(cx.waker());
        Poll::Pending
      }
      Some(r) => {
        self.wake();

        self.global.notify();

        let ticks = self.ticks.fetch_add(1, Ordering::SeqCst);
        if ticks % 64 == 0 {
          steal(&self.global.queue, &self.shard);
        }

        Poll::Ready(r)
      }
    })
    .await
  }

  /// Finds the next task to run.
  fn search(&self) -> Option<Runnable> {
    if let Ok(r) = self.shard.pop() {
      return Some(r);
    }

    // Try stealing from the global queue.
    if let Ok(r) = self.global.queue.pop() {
      return Some(r);
    }

    // Try stealing from other shards.
    let shards = self.global.shards.read();

    let n = shards.len();
    let start = rand::thread_rng().gen_range(0..n);
    let iter = shards
      .iter()
      .chain(shards.iter())
      .skip(start)
      .take(n)
      .filter(|shard| !Arc::ptr_eq(shard, &self.shard));

    for shard in iter {
      steal(shard, &self.shard);
      if let Ok(r) = self.shard.pop() {
        return Some(r);
      }
    }

    None
  }

  fn sleep(&self, waker: &Waker) -> bool {
    let mut sleepers = self.global.sleepers.lock();

    match self.sleeping.load(Ordering::SeqCst) {
      0 => self
        .sleeping
        .store(sleepers.insert(waker), Ordering::SeqCst),
      id => {
        if !sleepers.update(id, waker) {
          return false;
        }
      }
    }

    self
      .global
      .notified
      .swap(sleepers.is_notified(), Ordering::SeqCst);

    true
  }

  fn wake(&self) {
    let id = self.sleeping.swap(0, Ordering::SeqCst);
    if id == 0 {
      return;
    }

    let mut sleepers = self.global.sleepers.lock();
    sleepers.remove(id);

    self
      .global
      .notified
      .swap(sleepers.is_notified(), Ordering::SeqCst);
  }
}

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
