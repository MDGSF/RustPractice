use libc;
use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use std::ptr;

pub const NREADERNAMESIZE: usize = 128;
pub const MESGSIZE: usize = 8; /* max #bytes per message. */
pub const NMESG: usize = 16; /* max #messages */
pub const NREADER: usize = 8; /* max #readers */

/* default permissions for new files */
pub const FILE_MODE: u32 = libc::S_IRUSR | libc::S_IWUSR | libc::S_IRGRP | libc::S_IROTH;

#[repr(C)]
pub struct SWriter {
  pub mutex: libc::sem_t,
  pub nput: libc::size_t,
}

#[repr(C)]
pub struct SReader {
  pub mutex: libc::sem_t,
  pub nstored: libc::sem_t,
  pub enabled: libc::c_char,
  pub name: [libc::c_char; NREADERNAMESIZE],
  pub nget: libc::size_t,
}

#[repr(C)]
pub struct SMsg {
  pub rw: libc::pthread_rwlock_t,
  pub size: libc::size_t,
  pub data: [libc::c_char; MESGSIZE],
}

#[repr(C)]
pub struct SMemory {
  pub writer: SWriter,
  pub readers: [SReader; NREADER],
  pub msgs: [SMsg; NMESG],
}

#[repr(C)]
pub struct SRshm {
  shmname: String,
  readername: String,
  readeridx: usize,
  ptr: *mut libc::c_void,
}

impl SRshm {
  fn new(shmname: *const libc::c_char, programname: *const libc::c_char) -> Option<SRshm> {
    unsafe {
      assert!(!shmname.is_null());
      assert!(!programname.is_null());

      //libc::shm_unlink(shmname);

      let cshmname = CStr::from_ptr(shmname).to_str().unwrap();
      let cprogramname = CStr::from_ptr(programname).to_str().unwrap();

      let mut rshm = SRshm {
        shmname: String::from(cshmname),
        readername: String::from(cprogramname),
        readeridx: 0,
        ptr: ptr::null_mut(),
      };

      let mut shm_already_exist = false;
      let mut fd = libc::shm_open(
        shmname,
        libc::O_RDWR | libc::O_CREAT | libc::O_EXCL,
        FILE_MODE,
      );
      if fd == -1 {
        let errno = *libc::__errno_location();
        if errno == libc::EEXIST {
          shm_already_exist = true;
          fd = libc::shm_open(shmname, libc::O_RDWR, FILE_MODE);
          if fd == -1 {
            let errno = *libc::__errno_location();
            eprintln!(
              "shm_open error, shm_already_exist = {}, errno = {}",
              shm_already_exist, errno
            );
            return None;
          }
        } else {
          eprintln!(
            "shm_open error, shm_already_exist = {}, errno = {}",
            shm_already_exist, errno
          );
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
        libc::sem_init(&mut ptr.writer.mutex, 1, 1);
        ptr.writer.nput = 0;

        for reader in ptr.readers.iter_mut() {
          libc::sem_init(&mut reader.mutex, 1, 1);
          libc::sem_init(&mut reader.nstored, 1, 0);
          reader.enabled = 0;
          libc::memset(
            &mut reader.name[0] as *mut i8 as *mut libc::c_void,
            0,
            NREADERNAMESIZE,
          );
          reader.nget = 0;
        }

        for msg in ptr.msgs.iter_mut() {
          let mut mattr = mem::MaybeUninit::uninit();
          libc::pthread_rwlockattr_init(mattr.as_mut_ptr());
          let mattr = &mut *mattr.as_mut_ptr();
          libc::pthread_rwlockattr_setpshared(mattr, libc::PTHREAD_PROCESS_SHARED);
          libc::pthread_rwlock_init(&mut msg.rw, mattr);
          libc::pthread_rwlockattr_destroy(mattr);

          msg.size = 0;
          libc::memset(
            &mut msg.data[0] as *mut i8 as *mut libc::c_void,
            0,
            MESGSIZE,
          );
        }
      }

      let mut programname_already_exist = false;
      let mut found_empty_reader = false;
      let mut first_empty_idx = 0;
      for (i, reader) in ptr.readers.iter_mut().enumerate() {
        if reader.enabled == 0 {
          if !found_empty_reader {
            found_empty_reader = true;
            first_empty_idx = i;
          }
        } else {
          if libc::strcmp(&reader.name[0], programname) == 0 {
            programname_already_exist = true;
            rshm.readeridx = i;
            break;
          }
        }
      }
      if !programname_already_exist {
        if found_empty_reader {
          ptr.readers[first_empty_idx].enabled = 1;
          libc::strncpy(
            &mut ptr.readers[first_empty_idx].name[0],
            programname,
            NREADERNAMESIZE,
          );
          rshm.readeridx = first_empty_idx;
        } else {
          eprintln!("cannot find empty reader");
          return None;
        }
      }

      Some(rshm)
    }
  }
}

#[no_mangle]
pub extern "C" fn rshm_create(
  shmname: *const libc::c_char,
  programname: *const libc::c_char,
) -> *mut SRshm {
  if let Some(rshm) = SRshm::new(shmname, programname) {
    return Box::into_raw(Box::new(rshm));
  }
  return ptr::null_mut();
}

#[no_mangle]
pub extern "C" fn rshm_release(prshm: *mut SRshm) {
  unsafe {
    assert!(!prshm.is_null());
    Box::from_raw(prshm);
  }
}

#[no_mangle]
pub extern "C" fn rshm_destory(prshm: *mut SRshm) -> libc::c_int {
  unsafe {
    assert!(!prshm.is_null());
    let rshm = &mut *prshm;
    let cname = CString::new(rshm.shmname.clone()).unwrap();
    let cname = cname.as_ptr();
    if libc::shm_unlink(cname) == -1 {
      println!("shm_unlink error");
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
) -> libc::c_int {
  unsafe {
    if len > MESGSIZE {
      eprintln!(
        "input buffer too large, len = {}, MESGSIZE = {}",
        len, MESGSIZE
      );
      return -1;
    }

    println!("rshm_write 1");

    let rshm = &mut *rshm;
    let idx = rshm.readeridx;
    let ptr = rshm.ptr as *mut SMemory;
    let ptr = &mut *ptr;

    if libc::sem_wait(&mut ptr.writer.mutex) == -1 {
      let errno = *libc::__errno_location();
      eprintln!("sem_wait errno = {}", errno);
      return -1;
    }

    println!("[{}] rshm_write 2", idx);

    let nput = ptr.writer.nput;
    ptr.writer.nput += 1;
    if ptr.writer.nput >= NMESG {
      ptr.writer.nput = 0;
    }

    if libc::sem_post(&mut ptr.writer.mutex) == -1 {
      let errno = *libc::__errno_location();
      eprintln!("sem_post errno = {}", errno);
      return -1;
    }

    println!("[{}] rshm_write 3", idx);

    libc::pthread_rwlock_wrlock(&mut ptr.msgs[nput].rw);

    println!("[{}] rshm_write 4", idx);

    ptr.msgs[nput].size = len;
    libc::memcpy(
      &mut ptr.msgs[nput].data[0] as *mut i8 as *mut libc::c_void,
      buf as *const libc::c_void,
      len,
    );

    libc::pthread_rwlock_unlock(&mut ptr.msgs[nput].rw);

    println!("[{}] rshm_write 5", idx);

    for reader in ptr.readers.iter_mut() {
      if reader.enabled == 1 {
        let mut val: libc::c_int = 0;
        if libc::sem_getvalue(&mut reader.nstored, &mut val) == -1 {
          let errno = *libc::__errno_location();
          eprintln!("sem_getvalue errno = {}", errno);
          return -1;
        }
        if (val as usize) < NMESG {
          if libc::sem_post(&mut reader.nstored) == -1 {
            let errno = *libc::__errno_location();
            eprintln!("sem_post errno = {}", errno);
            return -1;
          }
        }
      }
    }

    println!("[{}] rshm_write 6", idx);

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

    println!("rshm_read 1");

    let rshm = &mut *rshm;
    let idx = rshm.readeridx;
    let ptr = rshm.ptr as *mut SMemory;
    let ptr = &mut *ptr;

    if timeout > 0 {
      println!("[{}] rshm_read 2", idx);

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

      let ret = libc::sem_timedwait(&mut ptr.readers[idx].nstored, &ts);
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
      println!("[{}] rshm_read 3", idx);

      let ret = libc::sem_trywait(&mut ptr.readers[idx].nstored);
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
      println!("[{}] rshm_read 4", idx);

      let ret = libc::sem_wait(&mut ptr.readers[idx].nstored);
      if ret == -1 {
        let errno = *libc::__errno_location();
        eprintln!("sem_wait errno = {}", errno);
        return -1;
      }
    }

    println!("[{}] rshm_read 5", idx);

    if libc::sem_wait(&mut ptr.readers[idx].mutex) == -1 {
      let errno = *libc::__errno_location();
      eprintln!("sem_wait errno = {}", errno);
      return -1;
    }

    println!("[{}] rshm_read 6", idx);

    let nget = ptr.readers[idx].nget;
    ptr.readers[idx].nget += 1;
    if ptr.readers[idx].nget >= NMESG {
      ptr.readers[idx].nget = 0;
    }

    if libc::sem_post(&mut ptr.readers[idx].mutex) == -1 {
      let errno = *libc::__errno_location();
      eprintln!("sem_post errno = {}", errno);
      return -1;
    }

    println!("[{}] rshm_read 7", idx);

    libc::pthread_rwlock_rdlock(&mut ptr.msgs[nget].rw);

    println!("[{}] rshm_read 8", idx);

    let size = ptr.msgs[nget].size;
    libc::memcpy(
      buf as *mut libc::c_void,
      &ptr.msgs[nget].data[0] as *const i8 as *const libc::c_void,
      size,
    );

    libc::pthread_rwlock_unlock(&mut ptr.msgs[nget].rw);

    println!("[{}] rshm_read 9", idx);

    size as i32
  }
}
