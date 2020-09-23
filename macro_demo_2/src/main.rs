macro_rules! show_integer {
    ($($t:ty)*) => ($(
        fn show_$t(num: $t) {
          println!("show_$t = {}", num);
        }
    )*)
}

show_integer! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

fn main() {
  show_u8(16);
}
