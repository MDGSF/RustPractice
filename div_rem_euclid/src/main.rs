// dividend / divisor = quotient .. remainder
// divisor * quotient + remainder = dividend
// 除数    * 商       + 余数      = 被除数
//
// remainder = dividend - divisor * quotient
// 余数      = 被除数   - 除数    * 商
//
// 7 / 4 = 1 .. 3      =>  4 *  1 +  3 =  7
// 7 / -4 = -1 .. 3    => -4 * -1 +  3 =  7
// -7 / 4 = -1 .. -3   =>  4 * -1 + -3 = -7
// -7 / -4 = 1 .. -3   => -4 *  1 + -3 = -7

fn main() {
  let a: i32 = 7;
  let b = 4;
  println!("{} / {} = {}", a, b, a / b); // 1
  println!("{} / {} = {}", a, -b, a / -b); // -1
  println!("{} / {} = {}", -a, b, -a / b); // -1
  println!("{} / {} = {}", -a, -b, -a / -b); // 1

  println!("{} % {} = {}", a, b, a % b); // 3
  println!("{} % {} = {}", a, -b, a % -b); // 3
  println!("{} % {} = {}", -a, b, -a % b); // -3
  println!("{} % {} = {}", -a, -b, -a % -b); // -3

  println!("{}.div_euclid({}) = {}", a, b, a.div_euclid(b)); // 1
  println!("{}.div_euclid({}) = {}", a, -b, a.div_euclid(-b)); // -1
  println!("({}).div_euclid({}) = {}", -a, b, (-a).div_euclid(b)); // -2
  println!("({}).div_euclid({}) = {}", -a, -b, (-a).div_euclid(-b)); // 2

  println!("{}.rem_euclid({}) = {}", a, b, a.rem_euclid(b)); // 3
  println!("{}.rem_euclid({}) = {}", a, -b, a.rem_euclid(-b)); // 3
  println!("({}).rem_euclid({}) = {}", -a, b, (-a).rem_euclid(b)); // 1
  println!("({}).rem_euclid({}) = {}", -a, -b, (-a).rem_euclid(-b)); // 1
}

// pub const fn div_euclid(self, rhs: Self) -> Self {
//   let q = self / rhs;
//   if self % rhs < 0 {
//     return if rhs > 0 { q - 1 } else { q + 1 };
//   }
//   q
// }

// pub const fn rem_euclid(self, rhs: Self) -> Self {
//   let r = self % rhs;
//   if r < 0 {
//     if rhs < 0 {
//       r - rhs
//     } else {
//       r + rhs
//     }
//   } else {
//     r
//   }
// }
