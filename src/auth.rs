use bcrypt::{hash, verify};

pub fn hash_password(password: &str) -> String {
    let hashed_password = hash(password, 12).unwrap();
    hashed_password
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let is_valid = verify(password, hashed_password).unwrap();
    is_valid
} 