use bcrypt::{hash, DEFAULT_COST};

pub fn hash_password(password: &str) -> String {
    let hashed = hash(password, DEFAULT_COST).unwrap();
    println!("Hashed password: {}", hashed);
    hashed
}
