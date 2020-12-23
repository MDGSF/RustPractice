use anyhow::{anyhow, Result};
use nanomsg_sys::*;
use std::env;
use std::{
  ffi::{CStr, CString},
  os::raw::c_char,
  os::raw::c_void,
};

fn server(url: &str) -> Result<()> {
  let curl = CString::new(url).unwrap();
  let curl = curl.as_bytes_with_nul().as_ptr() as *const c_char;

  let fd = rnn_socket(AF_SP, NN_PUB)?;

  rnn_bind(fd, curl)?;

  loop {
    let hello = String::from("hello");
    let hello_len = hello.len();
    let chello = CString::new(hello).unwrap();
    let chello = chello.as_bytes().as_ptr() as *const c_void;

    rnn_send(fd, chello, hello_len as u64, 0)?;

    std::thread::sleep(std::time::Duration::from_secs(1));
  }
}

fn client(url: &str) -> Result<()> {
  let curl = CString::new(url).unwrap();
  let curl = curl.as_bytes_with_nul().as_ptr() as *const c_char;

  let val = CString::new("").unwrap();
  let val = val.as_bytes_with_nul().as_ptr() as *const c_void;

  let fd = rnn_socket(AF_SP, NN_SUB)?;

  rnn_connect(fd, curl)?;

  rnn_setsockopt(fd, NN_SUB, NN_SUB_SUBSCRIBE, val, 0)?;

  loop {
    let buf_vec: Vec<u8> = vec![0u8; 1024];
    let mut buf_box_vec = buf_vec.into_boxed_slice();
    let buf_ptr_u8 = buf_box_vec.as_mut_ptr();
    let buf_ptr_i8 = buf_ptr_u8 as *mut c_char;
    let buf_ptr_void = buf_ptr_u8 as *mut c_void;

    let _rc = rnn_recv(fd, buf_ptr_void, 1024, 0)?;

    let cbuf = unsafe { CStr::from_ptr(buf_ptr_i8).to_str().unwrap() };
    let buf_str = String::from(cbuf);
    println!("{}", buf_str);
  }
}

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() <= 1 {
    return Err(anyhow!("Usage: pubsub [client|server] [url]"));
  }
  match args[1].as_str() {
    "client" => client(args[2].as_str()),
    "server" => server(args[2].as_str()),
    _ => Err(anyhow!("Usage: pubsub [client|server] [url]")),
  }
}
