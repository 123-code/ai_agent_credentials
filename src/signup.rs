use std::io;
use rusqlite::Connection;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use ai_agent_credentials::{verify_password, database::{retrieve_password, create_session, insert_password, create_table,insert_master_password}};
use crate::input::{get_username, get_password};
use crate::encrypt_aes::encrypt_aes;
use crate::encrypt_rsa::{encrypt_data, generate_key};


pub fn sign_up() {
    let conn = Connection::open("passwords.db").unwrap();
    create_table(&conn).unwrap();

    let username = get_username();
    let password = get_password();
    println!("insert master password: ");
    let master_password = get_password();
    let encrypted_master_password = encrypt_aes(&password, &master_password.as_bytes());
    println!("master password: {}", master_password);
    insert_master_password(&conn, &encrypted_master_password).unwrap();

    // Generate and store RSA keys
    let (private_key, public_key) = generate_key(&conn, &username);
    println!("RSA keys generated and stored for user: {}", username);

    let options = vec![
        "Google",
        "Apple",
        "GitHub", 
        "Enter Password",
        "Exit"
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select an authentication method")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    let provider = match selection {
        0 => "Google",
        1 => "Apple", 
        2 => "GitHub",
        3 => "Manual",
        4 => {
            println!("Exiting...");
            return;
        },
        _ => {
            println!("Invalid choice");
            return;
        }
    };

    let encrypted_password = encrypt_data(&public_key, password.as_bytes());
    insert_password(&conn, &username, &encrypted_password).unwrap();

    create_session(&conn, &username).unwrap();

    println!("Account created successfully with provider: {}", provider);
    println!("Session created successfully");
}