use rusqlite::Connection;

mod auth;
mod database;
mod input;
mod login;
mod signup;
mod menu;

use database::{create_table, insert_password, retrieve_password, create_session};
use auth::verify_password;
use input::{get_username, get_password};
use login::log_in;
use signup::sign_up;
use menu::main_menu;

fn main() {
    /*
    let username = get_username();
    let password = get_password();

    let conn = Connection::open("passwords.db").unwrap();
    create_table(&conn).unwrap();
    insert_password(&conn, &username, &password).unwrap();
    create_session(&conn, &username).unwrap();
    println!("Session created successfully");

    let retrieved_password = retrieve_password(&conn, &username).unwrap_or_else(|_| {
        println!("User not found, password not retrieved.");
        String::new()
    });

    if !retrieved_password.is_empty() {
        println!("Retrieved password: {}", retrieved_password);
        let is_valid = verify_password(&password, &retrieved_password);
        println!("Is valid: {}", is_valid);
    }
    */
   // log_in();
   main_menu();
}


