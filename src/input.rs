use std::io::{self, Write};
use rpassword::read_password;

pub fn get_username() -> String {
    println!("Enter your username: ");
    let mut username = String::new();
    io::stdin()
        .read_line(&mut username)
        .expect("Error al leer la línea");
    username.trim().to_string()
}

pub fn get_password() -> String {
    print!("Enter your password: ");
    io::stdout().flush().unwrap();
    read_password().unwrap()
} 