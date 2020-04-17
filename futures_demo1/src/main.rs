use std::{
  future::Future,
  pin::Pin,
  sync::{
    mpsc::{channel, Sender},
    Arc, Mutex,
  },
  task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
  thread::{self, JoinHandle},
  time::{Duration, Instant},
};

fn main() {
  println!("Hello, world!");
}

// ============================= EXECUTOR ====================================
fn block_on<F: Future>(mut future: F) -> F::Output {}

// ====================== FUTURE IMPLEMENTATION ==============================
#[derive(Clone)]
struct MyWaker {
  thread: thread::Thread,
}

fn mywaker_wake(s: &MyWaker) {
  let waker_ptr: *const MyWaker = s;
  let waker_arc = unsafe { Arc::from_raw(waker_ptr) };
  waker_arc.thread.unpark();
}

fn mywaker_clone(s: &MyWaker) -> RawWaker {
  let arc = unsafe { Arc::from_raw(s).clone() };
  std::mem::forget(arc.clone());
  RawWaker::new(Arc::into_raw(arc) as *const (), &VTABLE)
}

const VTABLE: RawWakerVTable = unsafe {
  RawWakerVTable::new(
    |s| mywaker_clone(&*(s as *const MyWaker)),
    |s| mywaker_wake(&*(s as *const MyWaker)),
    |s| mywaker_wake(*(s as *const &MyWaker)),
    |s| drop(Arc::from_raw(s as *const MyWaker)),
  )
};

fn waker_into_waker(s: *const MyWaker) -> Waker {
  let raw_waker = RawWaker::new(s as *const (), &VTABLE);
  unsafe { Waker::from_raw(raw_waker) }
}

#[derive(Clone)]
pub struct Task {
  id: usize,
  reactor: Arc<Mutex<Reactor>>,
  data: u64,
  is_registered: bool,
}

// =============================== REACTOR ===================================
struct Reactor {}

#[derive(Debug)]
enum Event {
  Close,
  Timeout(Waker, u64, usize),
}
