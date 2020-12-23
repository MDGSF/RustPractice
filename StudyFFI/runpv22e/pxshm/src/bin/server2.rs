use libc;
use pxshm::*;
use runpv22e::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::{mem, ptr};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "server2")]
struct Opt {
  #[structopt(name = "name")]
  name: String,
}

fn main() {
  let opt = Opt::from_args();

  unsafe {
    let cpathname = CString::new(opt.name.clone()).unwrap();
    let cpathname = cpathname.as_ptr();
    libc::shm_unlink(cpathname);
  }

  let fd = rshm_open(
    &opt.name,
    libc::O_RDWR | libc::O_CREAT | libc::O_EXCL,
    FILE_MODE,
  );

  let ptr = rmmap(
    ptr::null_mut(),
    mem::size_of::<shmstruct>(),
    libc::PROT_READ | libc::PROT_WRITE,
    libc::MAP_SHARED,
    fd,
    0,
  );
  let ptr = ptr as *mut shmstruct;
  let ptr = unsafe { &mut *ptr };

  rftruncate(fd, mem::size_of::<shmstruct>() as i64);
  rclose(fd);

  for index in 0..NMESG {
    ptr.msgoff[index] = (index * MESGSIZE) as i64;
  }

  rsem_init(&mut ptr.mutex, 1, 1);
  rsem_init(&mut ptr.nempty, 1, NMESG as u32);
  rsem_init(&mut ptr.nstored, 1, 0);
  rsem_init(&mut ptr.noverflowmutex, 1, 1);

  println!("before loop, size = {}", mem::size_of::<shmstruct>());

  let mut index = 0;
  let mut lastnoverflow = 0;
  loop {
    rsem_wait(&mut ptr.nstored);
    rsem_wait(&mut ptr.mutex);
    let offset = ptr.msgoff[index] as usize;

    unsafe {
      let cur = CStr::from_ptr(&mut ptr.msgdata[offset] as *const i8);
      println!("index = {}: {:?}", index, cur);
    }

    // show
    index += 1;
    if index >= NMESG {
      index = 0;
    }
    rsem_post(&mut ptr.mutex);
    rsem_post(&mut ptr.nempty);

    rsem_wait(&mut ptr.noverflowmutex);
    let temp = ptr.noverflow;
    rsem_post(&mut ptr.noverflowmutex);
    if temp != lastnoverflow {
      println!("noverflow = {}", temp);
      lastnoverflow = temp;
    }
  }
}
