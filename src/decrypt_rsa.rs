use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme};
use rand::rngs::OsRng;

pub fn decrypt_data(private_key: &RsaPrivateKey, encrypted_data: &[u8]) -> Vec<u8> {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    private_key.decrypt(padding, encrypted_data).expect("failed to decrypt")
}

let message = "hello from rust";

let padding = PaddingScheme::new_pkcs1v15_encrypt();


let encrypted_data = public_key.encrypt(&mut rng, padding, &data[..])
    .expect("failed to encrypt");

println!("Encrypted data: {:?}", encrypted_data);


let decrypted_data = private_key.decrypt(padding, &encrypted_data)
.expect("failed to decrypt");

assert_eq!(&data[..], &decrypted_data[..]);
println!("Decrypted data: {:?}", String::from_utf8(decrypted_data).unwrap());


