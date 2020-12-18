use hmac::Hmac;
use pbkdf2;
use sha1::Sha1;

fn main() {
  let password = "my_password".as_bytes();
  let salt = "salt".as_bytes();
  let rounds: u32 = 4;

  //let mut out = vec![0; 1024];
  //let mut out = vec![0; 128];
  let mut out = vec![0; 16];
  pbkdf2::pbkdf2::<Hmac<Sha1>>(&password, &salt, rounds, &mut out);

  println!("{:?}", out);
}
