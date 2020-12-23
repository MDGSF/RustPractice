use hex_literal::hex;
use sha3::{Digest, Sha3_256};

fn main() {
  // create a SHA3-256 object
  let mut hasher = Sha3_256::new();

  // write input message
  hasher.update(b"abc");

  // read hash digest
  let result = hasher.finalize();

  assert_eq!(
    result[..],
    hex!(
      "
    3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532
"
    )[..]
  );

  println!("{:?}", result);
}
