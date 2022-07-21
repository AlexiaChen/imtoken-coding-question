use std::string::String;
use pbkdf2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Salt
    },
    Pbkdf2,
    Algorithm,
    Params
};

use aes::cipher::StreamCipher;


const salt_str: &str = "fd9d44b2e8dee6ac977525028559171f84c3b796dc8a3b81527396b1222f46c9";


// AES128 Decrypt the string
fn decrypt(ciphertext: &str, key: &str) -> String {
   

    let iv = vec![0; 16];
    let cipher = aes::ctr(128, &key, &iv);
    let mut decrypted = vec![0; ciphertext.len()];
    cipher.decrypt(&ciphertext, &mut decrypted);
    String::from_utf8(decrypted).unwrap()
}

fn main() {
    let pass_str = String::from("Insecure Pa55w0rd");
    let pass_bytes = pass_str.as_bytes();

    let salt = Salt::new(salt_str).unwrap();

    let params = Params {rounds: 10240, output_length: 32};
    let password_hash = 
        Pbkdf2.hash_password_customized(pass_bytes, Some(Algorithm::Pbkdf2Sha256.into()), 
        Some(2), params, salt).unwrap();


    let mut decrived_key: [u8; 16] = [0x00; 16];
    decrived_key.copy_from_slice(password_hash.to_string().as_bytes());



}