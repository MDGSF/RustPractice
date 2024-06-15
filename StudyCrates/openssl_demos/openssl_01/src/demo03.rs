use anyhow::Result;
use openssl::base64;
use openssl::hash::MessageDigest;
use openssl::pkcs5::pbkdf2_hmac;
use openssl::symm::{decrypt, Cipher};

fn convert_slice_to_hex_string(slice: &[u8]) -> String {
    slice
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

// https://crypto.stackexchange.com/questions/3298/is-there-a-standard-for-openssl-interoperable-aes-encryption/79855#79855
fn demo03() -> Result<()> {
    let iter = 1000;
    let pass = b"password";

    let cipher_data_str = "U2FsdGVkX19EaWRhRGlkYfz1eROwQ6Qp+/JMNlR/0Gg=".to_string();
    let cipher_data = base64::decode_block(&cipher_data_str)?;
    println!("cipher_data: {}", String::from_utf8_lossy(&cipher_data));
    println!("cipher_data: {}", convert_slice_to_hex_string(&cipher_data));

    let salt = &cipher_data[8..16];
    let mut derived_key: Vec<u8> = vec![0; 48];
    pbkdf2_hmac(pass, salt, iter, MessageDigest::sha256(), &mut derived_key)?;
    let key = &derived_key[0..32];
    let iv = &derived_key[32..48];

    println!("salt: {}", convert_slice_to_hex_string(salt));
    println!(
        "pbkdf2_hmac key: {}, {}",
        key.len(),
        convert_slice_to_hex_string(key)
    );
    println!(
        "pbkdf2_hmac iv: {}, {}",
        iv.len(),
        convert_slice_to_hex_string(iv)
    );

    let cipher = Cipher::aes_256_cbc();
    let decrypted_data = decrypt(cipher, &key, Some(iv), &cipher_data[16..]).unwrap();
    let decrypted_data = decrypted_data
        .iter()
        .map(|e| *e as char)
        .collect::<String>();
    println!("decrypted_data: {}", decrypted_data);

    Ok(())
}

fn main() -> Result<()> {
    demo03()?;
    Ok(())
}
