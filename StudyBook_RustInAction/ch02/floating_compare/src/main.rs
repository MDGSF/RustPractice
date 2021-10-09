fn main() {
  // assert!(0.1 + 0.2 == 0.3);

  let result: f32 = 0.1 + 0.1;
  let desired: f32 = 0.2;
  let absolute_difference = (desired - result).abs();
  assert!(absolute_difference <= f32::EPSILON);

  assert_eq!(f32::NAN == f32::NAN, false);
}
