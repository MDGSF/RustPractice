fn main() {
  println!("1 + 2 = {}", 1u32 + 2);

  println!("1 - 2 = {}", 1i32 - 2);

  println!("true && false is {}", true && false);
  println!("true || false is {}", true || false);
  println!("NOT true is {}", !true);

  println!("0011 & 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 | 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80 >> 2);

  println!("one million is written as {}", 1_000_000u32);
}
