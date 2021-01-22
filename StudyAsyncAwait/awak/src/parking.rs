use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};
use std::sync::Arc;
use std::time::Duration;

use parking_lot::{Condvar, Mutex};

const EMPTY: usize = 0;
const PARKED: usize = 1;
const NOTIFIED: usize = 2;

pub fn pair() -> (Parker, Unparker) {
  let p = Parker::new();
  let u = p.unparker();
  (p, u)
}

pub struct Parker {
  unparker: Unparker,
}

impl Parker {
  pub fn new() -> Parker {
    Parker {
      unparker: Unparker {
        inner: Arc::new(Inner {
          state: AtomicUsize::new(0),
          lock: Mutex::new(()),
          cvar: Condvar::new(),
        }),
      },
    }
  }

  pub fn park(&self) -> bool {
    self.unparker.inner.park(None)
  }

  pub fn park_timeout(&self, timeout: Option<Duration>) -> bool {
    self.unparker.inner.park(timeout)
  }

  pub fn unparker(&self) -> Unparker {
    self.unparker.clone()
  }
}

pub struct Unparker {
  inner: Arc<Inner>,
}

impl Unparker {
  pub fn unpark(&self) -> bool {
    self.inner.unpark()
  }
}

impl Clone for Unparker {
  fn clone(&self) -> Unparker {
    Unparker {
      inner: self.inner.clone(),
    }
  }
}

pub struct Inner {
  state: AtomicUsize,
  lock: Mutex<()>,
  cvar: Condvar,
}

impl Inner {
  fn park(&self, timeout: Option<Duration>) -> bool {
    if self
      .state
      .compare_exchange(NOTIFIED, EMPTY, SeqCst, SeqCst)
      .is_ok()
    {
      return true;
    }

    if let Some(d) = timeout {
      if d == Duration::from_secs(0) {
        return false;
      }
    }

    let mut m = self.lock.lock();

    match self.state.compare_exchange(EMPTY, PARKED, SeqCst, SeqCst) {
      Ok(_) => {}
      Err(NOTIFIED) => {
        let old = self.state.swap(EMPTY, SeqCst);
        assert_eq!(old, NOTIFIED, "park state changed unexpectedly");
        return true;
      }
      Err(_) => panic!("invalid park state"),
    }

    match timeout {
      None => loop {
        self.cvar.wait(&mut m);

        if self
          .state
          .compare_exchange(NOTIFIED, EMPTY, SeqCst, SeqCst)
          .is_ok()
        {
          return true;
        }
      },
      Some(d) => {
        let _result = self.cvar.wait_for(&mut m, d);

        match self.state.swap(EMPTY, SeqCst) {
          NOTIFIED => true,
          PARKED => false,
          n => panic!("inconsistent park_timeout state: {}", n),
        }
      }
    }
  }

  fn unpark(&self) -> bool {
    match self.state.swap(NOTIFIED, SeqCst) {
      EMPTY => return true,
      NOTIFIED => return true,
      PARKED => {}
      _ => panic!("inconsistent state in unpark"),
    }

    drop(self.lock.lock());
    self.cvar.notify_one();

    true
  }
}
