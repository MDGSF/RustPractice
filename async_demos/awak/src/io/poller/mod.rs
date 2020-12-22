use std::io;
use std::os::unix::io::RawFd;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use cfg_if::cfg_if;
use parking_lot::Mutex;

macro_rules! syscall {
  ($fn:ident $args:tt) => {{
    let res = unsafe { libc::$fn $args };
    if res == -1 {
      Err(std::io::Error::last_os_error())
    } else {
      Ok(res)
    }
  }}
}

cfg_if! {
  if #[cfg(any(target_os = "linux", target_os = "android"))] {
    mod epoll;
    use epoll as sys;
  } else if #[cfg(any(
      target_os = "macos",
      target_os = "ios",
      target_os = "freebsd",
      target_os = "netbsd",
      target_os = "openbsd",
      target_os = "dragonfly",
  ))] {
    mod kqueue;
    use kqueue as sys;
  } else {
    compile_error!("does not support this target OS");
  }
}

pub struct Event {
  pub key: usize,
  pub readable: bool,
  pub writable: bool,
}

pub struct Poller {
  notified: AtomicBool,
}
