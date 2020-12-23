#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use anyhow::{anyhow, Context, Result};
use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn check_error(result: i32) -> Result<()> {
  if result < 0 {
    return Err(anyhow!(rnn_strerror()));
  }
  Ok(())
}

pub fn check_int_error(result: i32) -> Result<i32> {
  if result < 0 {
    return Err(anyhow!(rnn_strerror()));
  }
  Ok(result)
}

pub fn rnn_errno() -> ::std::os::raw::c_int {
  unsafe { nn_errno() }
}

pub fn rnn_strerror() -> String {
  unsafe {
    let result = nn_strerror(nn_errno());
    CStr::from_ptr(result).to_string_lossy().into_owned()
  }
}

pub fn rnn_socket(domain: u32, protocol: u32) -> Result<::std::os::raw::c_int> {
  unsafe {
    let socket = nn_socket(domain as i32, protocol as i32);
    check_int_error(socket).context("nn_socket failed")
  }
}

pub fn rnn_bind(s: ::std::os::raw::c_int, addr: *const ::std::os::raw::c_char) -> Result<()> {
  unsafe {
    let result = nn_bind(s, addr);
    check_error(result).context("nn_bind failed")
  }
}

pub fn rnn_connect(s: ::std::os::raw::c_int, addr: *const ::std::os::raw::c_char) -> Result<()> {
  unsafe {
    let result = nn_connect(s, addr);
    check_error(result).context("nn_connect failed")
  }
}

pub fn rnn_send(
  s: ::std::os::raw::c_int,
  buf: *const ::std::os::raw::c_void,
  len: size_t,
  flags: ::std::os::raw::c_int,
) -> Result<::std::os::raw::c_int> {
  unsafe {
    let result = nn_send(s, buf, len, flags);
    check_int_error(result).context("nn_send failed")
  }
}

pub fn rnn_recv(
  s: ::std::os::raw::c_int,
  buf: *mut ::std::os::raw::c_void,
  len: size_t,
  flags: ::std::os::raw::c_int,
) -> Result<::std::os::raw::c_int> {
  unsafe {
    let result = nn_recv(s, buf, len, flags);
    check_int_error(result).context("nn_recv failed")
  }
}

pub fn rnn_setsockopt(
  s: ::std::os::raw::c_int,
  level: u32,
  option: u32,
  optval: *const ::std::os::raw::c_void,
  optvallen: size_t,
) -> Result<()> {
  unsafe {
    let result = nn_setsockopt(s, level as i32, option as i32, optval, optvallen);
    check_error(result).context("nn_setsockopt failed")
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
