use std::mem;

//#[repr(C)]
struct FieldStruct {
  first: u8,
  second: u16,
  third: u8,
}

fn main() {
  let size = mem::size_of::<FieldStruct>();
  println!("size = {}", size);
}
