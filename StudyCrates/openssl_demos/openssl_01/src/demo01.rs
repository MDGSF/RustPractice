use anyhow::Result;
use openssl::base64;
use openssl::pkcs5::scrypt;
use openssl::symm::{decrypt, encrypt, Cipher};

fn demo01() -> Result<()> {
    // https://www.openssl.org/docs/man1.1.1/man7/scrypt.html
    // Memory required = 128 * N * r * p bytes = 128 * 1024 * 8 * 16 = 16MB
    let pass = b"password";
    let salt = b"Dida";
    let n = 1024;
    let r = 8;
    let p = 16;
    let maxmem = 16 * 1024 * 1024;
    let mut key = vec![0; 32];
    scrypt(pass, salt, n, r, p, maxmem, &mut key)?;
    println!("scrypt key: {}, {:?}", key.len(), key);

    let cipher_data = {
        let cipher = Cipher::aes_256_cbc();
        let plain_data = b"hello world";
        let cipher_data = encrypt(cipher, &key, None, plain_data)?;
        let cipher_data: String = base64::encode_block(&cipher_data);
        println!("cipher_data: {}", cipher_data);
        cipher_data
    };

    {
        let cipher = Cipher::aes_256_cbc();
        let cipher_data = base64::decode_block(&cipher_data)?;
        let decrypted_data = decrypt(cipher, &key, None, &cipher_data).unwrap();
        let decrypted_data = decrypted_data
            .iter()
            .map(|e| *e as char)
            .collect::<String>();
        println!("decrypted_data: {}", decrypted_data);
    }

    Ok(())
}

fn main() -> Result<()> {
    demo01()?;
    Ok(())
}
