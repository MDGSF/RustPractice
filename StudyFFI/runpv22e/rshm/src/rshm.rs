use libc;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::ptr;

pub const MESGSIZE: usize = 10240; /* max #bytes per message. */
pub const NMESG: usize = 16; /* max #messages */

/* default permissions for new files */
pub const FILE_MODE: u32 = libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IROTH;

#[repr(C)]
pub struct SMemory {
  pub mutex: libc::sem_t,
  pub nempty: libc::sem_t,
  pub nstored: libc::sem_t,
  pub nput: libc::size_t,
  pub nget: libc::size_t,
  pub noverflow: libc::size_t,
  pub noverflowmutex: libc::sem_t,
  pub msgoff: [libc::size_t; NMESG],
  pub msgsize: [libc::size_t; NMESG],
  pub msgdata: [libc::c_char; NMESG * MESGSIZE],
}

#[repr(C)]
pub struct SRshm {
  name: String,
  ptr: *mut libc::c_void,
}

impl SRshm {
  fn new(name: *const libc::c_char) -> Option<SRshm> {
    unsafe {
      assert!(!name.is_null());

      let cname = CStr::from_ptr(name).to_str().unwrap();

      let mut rshm = SRshm {
        name: String::from(cname),
        ptr: ptr::null_mut(),
      };

      let mut shm_already_exist = false;
      let mut fd = libc::shm_open(name, libc::O_RDWR | libc::O_CREAT | libc::O_EXCL, FILE_MODE);
      if fd == -1 {
        let errno = *libc::__errno_location();
        if errno == libc::EEXIST {
          shm_already_exist = true;
          fd = libc::shm_open(name, libc::O_RDWR, FILE_MODE);
          if fd == -1 {
            let errno = *libc::__errno_location();
            eprintln!("shm_open error, shm_already_exist = {}, errno = {}", shm_already_exist, errno);
            return None;
          }
        } else {
          eprintln!("shm_open error, shm_already_exist = {}, errno = {}", shm_already_exist, errno);
          return None;
        }
      }

      let ptr = libc::mmap(
        ptr::null_mut(),
        mem::size_of::<SMemory>(),
        libc::PROT_READ | libc::PROT_WRITE,
        libc::MAP_SHARED,
        fd,
        0,
      );
      if ptr == libc::MAP_FAILED {
        eprintln!("mmap error");
        return None;
      }
      rshm.ptr = ptr;
      let ptr = ptr as *mut SMemory;
      let ptr = &mut *ptr;

      if !shm_already_exist {
        libc::ftruncate(fd, mem::size_of::<SMemory>() as i64);
      }
      libc::close(fd);

      if !shm_already_exist {
        for index in 0..NMESG {
          ptr.msgoff[index] = index * MESGSIZE;
        }

        for index in 0..NMESG {
          ptr.msgsize[index] = 0;
        }

        ptr.nput = 0;
        ptr.nget = 0;
        ptr.noverflow = 0;

        libc::sem_init(&mut ptr.mutex, 1, 1);
        libc::sem_init(&mut ptr.nempty, 1, NMESG as u32);
        libc::sem_init(&mut ptr.nstored, 1, 0);
        libc::sem_init(&mut ptr.noverflowmutex, 1, 1);
      }

      Some(rshm)
    }
  }
}

#[no_mangle]
pub extern "C" fn rshm_create(name: *const libc::c_char) -> *mut SRshm {
  if let Some(rshm) = SRshm::new(name) {
    return Box::into_raw(Box::new(rshm));
  }
  return ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn rshm_release(prshm: *mut SRshm) -> libc::c_int {
  unsafe {
    assert!(!prshm.is_null());
    let rshm = &mut *prshm;
    let cname = CString::new(rshm.name.clone()).unwrap();
    let cname = cname.as_ptr();
    if libc::shm_unlink(cname) == -1 {
      println!("shm_unlink error");
      return -1;
    }
    Box::from_raw(prshm);
    0
  }
}

#[no_mangle]
pub extern "C" fn rshm_write(
  rshm: *mut SRshm,
  buf: *const libc::c_char,
  len: libc::size_t,
  timeout: libc::c_int,
) -> libc::c_int {
  unsafe {
    if len > MESGSIZE {
      eprintln!(
        "input buffer too large, len = {}, MESGSIZE = {}",
        len, MESGSIZE
      );
      return -1;
    }

    let rshm = &mut *rshm;
    let ptr = rshm.ptr as *mut SMemory;
    let ptr = &mut *ptr;

    if timeout > 0 {
      let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
      };
      if libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts) == -1 {
        let errno = *libc::__errno_location();
        eprintln!("clock_gettime errno = {}", errno);
        return -1;
      }
      ts.tv_sec += timeout as i64;

      let ret = libc::sem_timedwait(&mut ptr.nempty, &ts);
      if ret == -1 {
        let errno = *libc::__errno_location();
        if errno == libc::ETIMEDOUT {
          return -2;
        } else {
          eprintln!("sem_timedwait errno = {}", errno);
          return -1;
        }
      }
    } else if timeout == 0 {
      let ret = libc::sem_trywait(&mut ptr.nempty);
      if ret == -1 {
        let errno = *libc::__errno_location();
        if errno == libc::EAGAIN {
          return -1;
        } else {
          eprintln!("sem_trywait errno = {}", errno);
          return -1;
        }
      }
    } else {
      let ret = libc::sem_wait(&mut ptr.nempty);
      if ret == -1 {
        let errno = *libc::__errno_location();
        eprintln!("sem_wait errno = {}", errno);
        return -1;
      }
    }

    if libc::sem_wait(&mut ptr.mutex) == -1 {
      let errno = *libc::__errno_location();
      eprintln!("sem_wait errno = {}", errno);
      return -1;
    }

    ptr.msgsize[ptr.nput] = len;
    let offset: usize = ptr.msgoff[ptr.nput];
    ptr.nput += 1;
    if ptr.nput >= NMESG {
      ptr.nput = 0;
    }

    if libc::sem_post(&mut ptr.mutex) == -1 {
      let errno = *libc::__errno_location();
      eprintln!("sem_post errno = {}", errno);
      return -1;
    }

    libc::memcpy(
      &mut ptr.msgdata[offset] as *mut i8 as *mut libc::c_void,
      buf as *const libc::c_void,
      len,
    );

    if libc::sem_post(&mut ptr.nstored) == -1 {
      let errno = *libc::__errno_location();
      eprintln!("sem_post errno = {}", errno);
      return -1;
    }

    len as i32
  }
}

#[no_mangle]
pub extern "C" fn rshm_read(
  rshm: *mut SRshm,
  buf: *mut libc::c_char,
  len: libc::size_t,
  timeout: libc::c_int,
) -> libc::c_int {
  unsafe {
    if len < MESGSIZE {
      eprintln!(
        "input buffer too small, len = {}, MESGSIZE = {}",
        len, MESGSIZE
      );
      return -1;
    }

    let rshm = &mut *rshm;
    let ptr = rshm.ptr as *mut SMemory;
    let ptr = &mut *ptr;

    if timeout > 0 {
      let mut ts = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
      };
      if libc::clock_gettime(libc::CLOCK_REALTIME, &mut ts) == -1 {
        let errno = *libc::__errno_location();
        eprintln!("clock_gettime errno = {}", errno);
        return -1;
      }
      ts.tv_sec += timeout as i64;

      let ret = libc::sem_timedwait(&mut ptr.nstored, &ts);
      if ret == -1 {
        let errno = *libc::__errno_location();
        if errno == libc::ETIMEDOUT {
          return -2;
        } else {
          eprintln!("sem_timedwait errno = {}", errno);
          return -1;
        }
      }
    } else if timeout == 0 {
      let ret = libc::sem_trywait(&mut ptr.nstored);
      if ret == -1 {
        let errno = *libc::__errno_location();
        if errno == libc::EAGAIN {
          return -1;
        } else {
          eprintln!("sem_trywait errno = {}", errno);
          return -1;
        }
      }
    } else {
      let ret = libc::sem_wait(&mut ptr.nstored);
      if ret == -1 {
        let errno = *libc::__errno_location();
        eprintln!("sem_wait errno = {}", errno);
        return -1;
      }
    }

    if libc::sem_wait(&mut ptr.mutex) == -1 {
      let errno = *libc::__errno_location();
      eprintln!("sem_wait errno = {}", errno);
      return -1;
    }

    let size = ptr.msgsize[ptr.nget];
    let offset: usize = ptr.msgoff[ptr.nget];
    ptr.nget += 1;
    if ptr.nget >= NMESG {
      ptr.nget = 0;
    }

    if libc::sem_post(&mut ptr.mutex) == -1 {
      let errno = *libc::__errno_location();
      eprintln!("sem_post errno = {}", errno);
      return -1;
    }

    libc::memcpy(
      buf as *mut libc::c_void,
      &ptr.msgdata[offset] as *const i8 as *const libc::c_void,
      size,
    );

    if libc::sem_post(&mut ptr.nempty) == -1 {
      let errno = *libc::__errno_location();
      eprintln!("sem_post errno = {}", errno);
      return -1;
    }

    size as i32
  }
}
