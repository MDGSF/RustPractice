use anyhow::Result;
use openssl::base64;
use openssl::hash::MessageDigest;
use openssl::pkcs5::pbkdf2_hmac;
use openssl::symm::{decrypt, encrypt, Cipher};

fn show_slice_as_hex_format(slice: &[u8]) {
    for &byte in slice.iter() {
        print!("{:02X}", byte);
    }
}

fn demo02() -> Result<()> {
    let iter = 1000;
    let pass = b"password";
    let salt = b"DidaDida";
    let mut derived_key = vec![0; 48];
    pbkdf2_hmac(pass, salt, iter, MessageDigest::sha256(), &mut derived_key)?;
    let key = &derived_key[0..32];
    let iv = &derived_key[32..48];

    show_slice_as_hex_format(salt);
    println!();

    print!("pbkdf2_hmac key: {}, ", key.len());
    show_slice_as_hex_format(key);
    println!();

    print!("pbkdf2_hmac iv: {}, ", iv.len());
    show_slice_as_hex_format(iv);
    println!();

    let cipher_data = {
        let cipher = Cipher::aes_256_cbc();
        let plain_data = b"hello world";
        let cipher_data = encrypt(cipher, &key, Some(iv), plain_data)?;
        let cipher_data: String = base64::encode_block(&cipher_data);
        println!("cipher_data: {}", cipher_data);
        cipher_data
    };

    {
        let cipher = Cipher::aes_256_cbc();
        let cipher_data = base64::decode_block(&cipher_data)?;
        let decrypted_data = decrypt(cipher, &key, Some(iv), &cipher_data).unwrap();
        let decrypted_data = decrypted_data
            .iter()
            .map(|e| *e as char)
            .collect::<String>();
        println!("decrypted_data: {}", decrypted_data);
    }

    Ok(())
}

fn main() -> Result<()> {
    demo02()?;
    Ok(())
}
