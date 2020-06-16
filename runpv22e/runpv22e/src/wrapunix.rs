use libc;
use std::ffi::CString;

pub fn rsem_open(
  pathname: &str,
  oflag: i32,
  mode: Option<libc::mode_t>,
  value: Option<u32>,
) -> *mut libc::sem_t {
  unsafe {
    let cpathname = CString::new(pathname).unwrap();
    let cpathname = cpathname.as_ptr();

    if oflag & libc::O_CREAT != 0 {
      let mode = mode.unwrap();
      let value = value.unwrap();

      let sem: *mut libc::sem_t = libc::sem_open(cpathname, oflag, mode, value);
      if sem == libc::SEM_FAILED {
        println!("sem_open error for {}", pathname);
        std::process::exit(1);
      }
      return sem;
    }

    let sem: *mut libc::sem_t = libc::sem_open(cpathname, oflag);
    if sem == libc::SEM_FAILED {
      println!("sem_open error for {}", pathname);
      std::process::exit(1);
    }
    return sem;
  }
}

pub fn rsem_close(sem: *mut libc::sem_t) {
  unsafe {
    let ret = libc::sem_close(sem);
    if ret == -1 {
      println!("sem_close error");
      std::process::exit(1);
    }
  }
}

pub fn rsem_wait(sem: *mut libc::sem_t) {
  unsafe {
    if libc::sem_wait(sem) == -1 {
      println!("sem_wait error");
      std::process::exit(1);
    }
  }
}

pub fn rsem_trywait(sem: *mut libc::sem_t) -> i32 {
  unsafe {
    let ret = libc::sem_trywait(sem);
    if ret == -1 && *libc::__errno_location() != libc::EAGAIN {
      println!("sem_trywait error");
      std::process::exit(1);
    }
    return ret;
  }
}

pub fn rsem_post(sem: *mut libc::sem_t) {
  unsafe {
    if libc::sem_post(sem) == -1 {
      println!("sem_post error");
      std::process::exit(1);
    }
  }
}

pub fn rsem_getvalue(sem: *mut libc::sem_t, valp: *mut libc::c_int) {
  unsafe {
    let ret = libc::sem_getvalue(sem, valp);
    if ret == -1 {
      println!("sem_getvalue error");
      std::process::exit(1);
    }
  }
}
