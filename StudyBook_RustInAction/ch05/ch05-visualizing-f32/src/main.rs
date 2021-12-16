const BIAS: i32 = 127;
const RADIX: f32 = 2.0;

fn main() {
  let n: f32 = 42.42;
  let (signbit, exponent, fraction) = deconstruct_f32(n);
  let (sign, exponent, mantissa) =
    decode_f32_parts(signbit, exponent, fraction);
  let reconstituted_n = f32_from_parts(sign, exponent, mantissa);

  println!(
    "{} -> [sign:{}, exponent:{}, mantissa:{:?}] -> {}",
    n, signbit, exponent, mantissa, reconstituted_n
  );
}

fn deconstruct_f32(n: f32) -> (u32, u32, u32) {
  let n: u32 = unsafe { std::mem::transmute(n) };
  let sign = (n >> 31) & 1;
  let exponent = (n >> 23) & 0xff;
  let fraction = 0b00000000_01111111_11111111_11111111 & n;
  (sign, exponent, fraction)
}

fn decode_f32_parts(
  sign: u32,
  exponent: u32,
  fraction: u32,
) -> (f32, f32, f32) {
  let sign = (-1.0_f32).powf(sign as f32);

  let exponent = (exponent as i32) - BIAS;
  let exponent = RADIX.powf(exponent as f32);

  let mut mantissa: f32 = 1.0;
  for i in 0..23_u32 {
    let one_at_bit_i = 1 << i;
    if (one_at_bit_i & fraction) != 0 {
      mantissa += 2_f32.powf((i as f32) - 23.0);
    }
  }

  (sign, exponent, mantissa)
}

fn f32_from_parts(sign: f32, exponent: f32, mantissa: f32) -> f32 {
  sign * exponent * mantissa
}
