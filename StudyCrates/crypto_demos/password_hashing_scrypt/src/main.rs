extern crate scrypt;

use scrypt::{scrypt, scrypt_check, scrypt_simple, ScryptParams};

fn test1() {
  // First setup the ScryptParams arguments with the recommended defaults
  let params = ScryptParams::recommended();
  // Hash the password for storage
  let hashed_password =
    scrypt_simple("Not so secure password", &params).expect("OS RNG should not fail");
  // Verifying a stored password
  assert!(scrypt_check("Not so secure password", &hashed_password).is_ok());

  println!("{}", hashed_password);
}

fn test2() {
  let log_n: u8 = 10;
  let r: u32 = 8;
  let p: u32 = 16;
  let params = ScryptParams::new(log_n, r, p).unwrap();

  let mut result = vec![0u8; 16];
  let password = "password";
  let salt = "NaCl";
  scrypt(password.as_bytes(), salt.as_bytes(), &params, &mut result).unwrap();

  println!("{:?}", result);
}

fn main() {
  test1();
  test2();
}
