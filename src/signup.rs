use std::io;
use rusqlite::Connection;
use colored::Colorize;
use dialoguer::{theme::ColorfulTheme, Select};
use crate::auth::verify_password;
use crate::database::{retrieve_password, create_session, insert_password, create_table};
use crate::input::{get_username, get_password};

pub fn sign_up() {
    let conn = Connection::open("passwords.db").unwrap();
    create_table(&conn).unwrap();
    
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

    let username = get_username();
    let password = get_password();

    insert_password(&conn, &username, &password).unwrap();
    create_session(&conn, &username).unwrap();
    println!("Account created successfully with provider: {}", provider);
    println!("Session created successfully");
}