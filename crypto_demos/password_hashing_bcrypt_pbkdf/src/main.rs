use bcrypt_pbkdf::bcrypt_pbkdf;

fn main() {
  let password = "my_password".to_string();
  let salt = "salt".as_bytes();
  let rounds: u32 = 4;

  //let mut out = vec![0; 1024];
  //let mut out = vec![0; 128];
  let mut out = vec![0; 16];
  bcrypt_pbkdf(&password[..], &salt[..], rounds, &mut out).unwrap();

  println!("{:?}", out);
}
