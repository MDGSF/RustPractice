extern crate libc;

#[link(name = "ReadBuffer")]
extern "C" {
  fn read_into_buffer(pcBuf: *mut libc::c_char, iBufSize: i32) -> i32;
}

fn main() {
  unsafe {
    println!("Hello, world!");

    let mut rbuf: Vec<u8> = vec![0u8; 100];
    //let mut buf = rbuf.into_boxed_slice();
    let buf = rbuf.as_mut_ptr() as *mut libc::c_char;

    read_into_buffer(buf, 100);

    for i in 0..rbuf.len() {
      println!("{}", rbuf[i]);
    }
  }
}
