use k256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey};
use rand_core::OsRng; // requires 'getrandom' feature

fn main() {
  // Alice
  let alice_secret = EphemeralSecret::random(&mut OsRng);
  let alice_pk_bytes = EncodedPoint::from(alice_secret.public_key());

  // Bob
  let bob_secret = EphemeralSecret::random(&mut OsRng);
  let bob_pk_bytes = EncodedPoint::from(bob_secret.public_key());

  // Alice decodes Bob's serialized public key and computes a shared secret from it
  let bob_public =
    PublicKey::from_sec1_bytes(bob_pk_bytes.as_ref()).expect("bob's public key is invalid!"); // In real usage, don't panic, handle this!

  let alice_shared = alice_secret.diffie_hellman(&bob_public);

  // Bob deocdes Alice's serialized public key and computes the same shared secret
  let alice_public =
    PublicKey::from_sec1_bytes(alice_pk_bytes.as_ref()).expect("alice's public key is invalid!"); // In real usage, don't panic, handle this!

  let bob_shared = bob_secret.diffie_hellman(&alice_public);

  // Both participants arrive on the same shared secret
  assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());
}
