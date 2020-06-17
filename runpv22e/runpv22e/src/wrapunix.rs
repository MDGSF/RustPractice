use libc;
use std::ffi::CString;

pub fn rclose(fd: libc::c_int) {
  unsafe {
    if libc::close(fd) == -1 {
      println!("close error");
      std::process::exit(1);
    }
  }
}

pub fn rftruncate(fd: libc::c_int, length: libc::off_t) {
  unsafe {
    if libc::ftruncate(fd, length) == -1 {
      println!("ftruncate error");
      std::process::exit(1);
    }
  }
}

pub fn rmmap(
  addr: *mut libc::c_void,
  len: libc::size_t,
  prot: libc::c_int,
  flags: libc::c_int,
  fd: libc::c_int,
  offset: libc::off_t,
) -> *mut libc::c_void {
  unsafe {
    let ptr = libc::mmap(addr, len, prot, flags, fd, offset);
    if ptr == libc::MAP_FAILED {
      println!("mmap error");
      std::process::exit(1);
    }
    return ptr;
  }
}

pub fn rmunmap(addr: *mut libc::c_void, len: libc::size_t) {
  unsafe {
    if libc::munmap(addr, len) == -1 {
      println!("munmap error");
      std::process::exit(1);
    }
  }
}

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
    if libc::sem_close(sem) == -1 {
      println!("sem_close error");
      std::process::exit(1);
    }
  }
}

pub fn rsem_unlink(pathname: &str) {
  unsafe {
    let cpathname = CString::new(pathname).unwrap();
    let cpathname = cpathname.as_ptr();
    if libc::sem_unlink(cpathname) == -1 {
      println!("sem_unlink error");
      std::process::exit(1);
    }
  }
}

pub fn rsem_init(sem: *mut libc::sem_t, pshared: libc::c_int, value: u32) {
  unsafe {
    if libc::sem_init(sem, pshared, value) == -1 {
      println!("sem_init error");
      std::process::exit(1);
    }
  }
}

pub fn rsem_destroy(sem: *mut libc::sem_t) {
  unsafe {
    if libc::sem_destroy(sem) == -1 {
      println!("sem_destroy error");
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

pub fn rshm_open(pathname: &str, oflag: i32, mode: libc::mode_t) -> i32 {
  unsafe {
    let cpathname = CString::new(pathname).unwrap();
    let cpathname = cpathname.as_ptr();
    let fd = libc::shm_open(cpathname, oflag, mode);
    if fd == -1 {
      println!("shm_open error");
      std::process::exit(1);
    }
    fd
  }
}

pub fn rshm_unlink(pathname: &str) {
  unsafe {
    let cpathname = CString::new(pathname).unwrap();
    let cpathname = cpathname.as_ptr();
    if libc::shm_unlink(cpathname) == -1 {
      println!("shm_unlink error");
      std::process::exit(1);
    }
  }
}
