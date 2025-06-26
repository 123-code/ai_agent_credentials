use aes_gcm::{aead::{Aead,KeyInit,OsRng},Aes256Gcm,Nonce};
use pbkdf2::pbkdf2;
use hmac::Hmac;
use rand_core::RngCore;
use sha2::Sha256;

const SALT_LENGTH: usize = 16;
const NONCE_LENGTH: usize = 12;
const PBKDF2_ITERATIONS: u32 = 100_000;


pub fn encrypt_aes(password: &str,plaintext: &[u8]) -> Vec<u8> {
    let mut salt = [0u8;SALT_LENGTH];
    let mut nonce_bytes = [0u8;NONCE_LENGTH];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce_bytes);

let mut key = [0u8;32];

let _ = pbkdf2::<Hmac<Sha256>>(
    password.as_bytes(),
    &salt,
    PBKDF2_ITERATIONS,
    &mut key,
);
let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
let nonce = Nonce::from_slice(&nonce_bytes);
let ciphertext = cipher.encrypt(nonce, plaintext).unwrap();  
[&salt[..], &nonce_bytes[..], &ciphertext].concat()
}


pub fn decrypt_aes(password:&str,package:&[u8])->Result<Vec<u8>,aes_gcm::Error>{
    let salt = &package[..SALT_LENGTH];
    let nonce_bytes = &package[SALT_LENGTH..SALT_LENGTH + NONCE_LENGTH];
    let ciphertext = &package[SALT_LENGTH + NONCE_LENGTH..];

    let mut key = [0u8;32];
    let _ = pbkdf2::<Hmac<Sha256>>(
        password.as_bytes(),
        salt,
        PBKDF2_ITERATIONS,
        &mut key,
    );

    let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher.decrypt(nonce, ciphertext)

}