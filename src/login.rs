use rusqlite::Connection;
use agent_credentials::{verify_password, database::{retrieve_password, create_session}};
use crate::input::{get_username, get_password};

pub fn log_in() {
    let username = get_username();
    let password = get_password();
    let conn = Connection::open("passwords.db").unwrap();
    
    let retrieved_password = retrieve_password(&conn, &username).unwrap_or_else(|_| {
        println!("User not found, password not retrieved.");
        String::new()
    });

    if !retrieved_password.is_empty() {
        let is_valid = verify_password(&password, &retrieved_password);
        if is_valid {
            create_session(&conn, &username).unwrap();
            println!("Login successful! Session created.");
        } else {
            println!("Invalid password!");
        }
    } else {
        println!("User not found!");
    }
}
