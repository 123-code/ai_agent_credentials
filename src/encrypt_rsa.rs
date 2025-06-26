use rsa::{RsaPrivateKey, RsaPublicKey, traits::PaddingScheme};
use rand::rngs::OsRng;
use ai_agent_credentials::database::{retrieve_master_password, store_private_key};
use crate::encrypt_aes::encrypt_aes;
use rsa::pkcs1::EncodeRsaPrivateKey;
use rsa::pkcs1v15::Pkcs1v15Encrypt;



pub fn generate_key(conn: &rusqlite::Connection, username: &str) -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
    let public_key = RsaPublicKey::from(&private_key);
    let encrypted_private_key = encrypt_aes(&private_key.to_pkcs1_pem(rsa::pkcs1::LineEnding::LF).unwrap().to_string(), retrieve_master_password(conn).expect("Failed to retrieve master password").as_bytes());
    // Store the encrypted private key in the database
    ai_agent_credentials::database::store_private_key(conn, username, &encrypted_private_key);
    (private_key, public_key)
}

pub fn encrypt_data(public_key: &RsaPublicKey, data: &[u8]) -> Vec<u8> {
    let mut rng = OsRng;
    let padding = Pkcs1v15Encrypt;
    let encrypted_data = public_key.encrypt(&mut rng, padding, data).expect("failed to encrypt");
    println!("Encrypted data: {:?}", encrypted_data);
    encrypted_data
}

pub fn decrypt_data(private_key: &RsaPrivateKey, data: &[u8]) -> Vec<u8> {
    let padding = Pkcs1v15Encrypt;
    let decrypted_data = private_key.decrypt(padding, data).expect("failed to decrypt");
    decrypted_data
}

