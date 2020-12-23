use aes_gcm_siv::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm_siv::Aes256GcmSiv; // Or `Aes128GcmSiv`

fn main() {
  let key = GenericArray::from_slice(b"an example very very secret key.");
  let cipher = Aes256GcmSiv::new(key);

  let nonce = GenericArray::from_slice(b"unique nonce"); // 96-bits; unique per message

  let ciphertext = cipher
    .encrypt(nonce, b"plaintext message".as_ref())
    .expect("encryption failure!"); // NOTE: handle this error to avoid panics!

  let plaintext = cipher
    .decrypt(nonce, ciphertext.as_ref())
    .expect("decryption failure!"); // NOTE: handle this error to avoid panics!

  assert_eq!(&plaintext, b"plaintext message");
}
