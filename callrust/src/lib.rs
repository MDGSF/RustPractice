use libc::{c_char, c_uint};
use std::ffi::{CStr, CString};
use std::iter;
use std::slice;

#[no_mangle]
pub extern "C" fn print_hello_from_rust() {
  println!("Hello from Rust");
}

// 从C语言中传递数组到Rust中，计算数组中偶数的和
#[no_mangle]
pub extern "C" fn sum_of_even(n: *const c_uint, len: c_uint) -> c_uint {
  let numbers = unsafe {
    assert!(!n.is_null());
    slice::from_raw_parts(n, len as usize)
  };
  let sum = numbers
    .iter()
    .filter(|&v| v % 2 == 0)
    .fold(0, |acc, v| acc + v);
  sum as c_uint
}

// 从C语言中传递字符串到Rust中，计算字符串长度
#[no_mangle]
pub extern "C" fn hm_chars(s: *const c_char) -> c_uint {
  let c_str = unsafe {
    assert!(!s.is_null());
    CStr::from_ptr(s)
  };
  let r_str = c_str.to_str().unwrap();
  r_str.chars().count() as c_uint
}

// 从Rust中返回字符串到C语言中，要再次返回给Rust释放内存
#[no_mangle]
pub extern "C" fn batman_song(length: c_uint) -> *mut c_char {
  let mut song = String::from("boom ");
  song.extend(iter::repeat("nana ").take(length as usize));
  song.push_str("Batman! boom");
  let c_str_song = CString::new(song).unwrap();
  c_str_song.into_raw()
}

// 手动在C语言中调用一遍清理内存
#[no_mangle]
pub extern "C" fn free_song(s: *mut c_char) {
  unsafe {
    if s.is_null() {
      return;
    }
    CString::from_raw(s)
  };
}

// 用结构体模拟元组传递给C语言
#[repr(C)]
pub struct Tuple {
  x: c_uint,
  y: c_uint,
}

// 方便Rust的(u32, u32)转换为Tuple结构体
impl From<(u32, u32)> for Tuple {
  fn from(tup: (u32, u32)) -> Tuple {
    Tuple { x: tup.0, y: tup.1 }
  }
}

// 方便Tuple结构体转换为Rust的(u32, u32)
impl From<Tuple> for (u32, u32) {
  fn from(tup: Tuple) -> (u32, u32) {
    (tup.x, tup.y)
  }
}

fn compute_tuple(tup: (u32, u32)) -> (u32, u32) {
  let (a, b) = tup;
  (b + 1, a - 1)
}

#[no_mangle]
pub extern "C" fn flip_things_around(tup: Tuple) -> Tuple {
  compute_tuple(tup.into()).into()
}

// 传递Rust抽象结构比如，结构体实例）给C语言
// C语言使用Opaque类型，Rust使用Box<T>
use std::collections::HashMap;
pub struct Database {
  data: HashMap<String, u32>,
}
impl Database {
  fn new() -> Database {
    Database {
      data: HashMap::new(),
    }
  }
  fn insert(&mut self) {
    for i in 0..100000 {
      let zip = format!("{:05}", i);
      self.data.insert(zip, i);
    }
  }
  fn get(&self, zip: &str) -> u32 {
    self.data.get(zip).cloned().unwrap_or(0)
  }
}

#[no_mangle]
pub extern "C" fn database_new() -> *mut Database {
  Box::into_raw(Box::new(Database::new()))
}

// 由Rust分配内存，就由Rust来释放内存
#[no_mangle]
pub extern "C" fn database_free(ptr: *mut Database) {
  if ptr.is_null() {
    return;
  }
  unsafe {
    Box::from_raw(ptr);
  }
}

#[no_mangle]
pub extern "C" fn database_insert(ptr: *mut Database) {
  let database = unsafe {
    assert!(!ptr.is_null());
    &mut *ptr
  };
  database.insert();
}

#[no_mangle]
pub extern "C" fn database_query(ptr: *mut Database, zip: *const c_char) -> c_uint {
  let database = unsafe {
    assert!(!ptr.is_null());
    &*ptr
  };
  let zip = unsafe {
    assert!(!zip.is_null());
    CStr::from_ptr(zip)
  };
  let zip_str = zip.to_str().unwrap();
  database.get(zip_str)
}
