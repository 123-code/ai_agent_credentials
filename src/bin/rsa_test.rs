use ai_agent_credentials::rsa::{generate_key, encrypt_data, decrypt_data};

fn main() {
    let (private_key, public_key) = generate_key();
    let message = "hello from rust";
    let data = message.as_bytes();
    
    let encrypted_data = encrypt_data(&public_key, data);
    println!("Encrypted data: {:?}", encrypted_data);
    
    let decrypted_data = decrypt_data(&private_key, &encrypted_data);
    println!("Decrypted data: {}", String::from_utf8_lossy(&decrypted_data));
    
    assert_eq!(data, &decrypted_data[..]);
    println!("RSA encryption and decryption successful!");
} 